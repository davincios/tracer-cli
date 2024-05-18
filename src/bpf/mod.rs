use aya::{Bpf, programs::TracePoint, maps::perf::PerfEventArray};
use aya_log::BpfLogger;
use tokio::{self, time, fs};
use std::convert::TryInto;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio_stream::wrappers::IntervalStream;
use futures::StreamExt;

#[repr(C)]
struct LogEntry {
    filename: [u8; 256],
}

async fn run_bpf_test() -> Result<(), Box<dyn std::error::Error>> {
    // Load the BPF program from the local file
    let mut bpf = Bpf::load_file("bpf/monitor.o")?;

    // Set up logging in a separate scope to avoid multiple mutable borrows
    {
        BpfLogger::init(&mut bpf)?;
    }

    // Attach the program to the sys_enter_execve tracepoint
    let program: &mut TracePoint = bpf.program_mut("bpf_prog").unwrap().try_into()?;
    program.load()?;
    let link_id = program.attach("syscalls", "sys_enter_execve")?;

    // Open the perf event array
    let mut perf_array = PerfEventArray::try_from(bpf.map_mut("ringbuf_map").ok_or("map not found")?)?;
    for cpu_id in aya::util::online_cpus()? {
        perf_array.open(cpu_id, None)?;
    }
    
    let mut log_file = fs::File::create("/var/log/bpf_execve.log").await?;

    // Run for 30 seconds and read logs
    let timeout = IntervalStream::new(time::interval(Duration::from_secs(30)));
    tokio::pin!(timeout);

    while timeout.next().await.is_none() {
        let events = perf_array.read_events()?;
        for event in events {
            let data = event.data;
            let log_entry = unsafe { std::ptr::read_unaligned(data.as_ptr() as *const LogEntry) };
            let filename = std::str::from_utf8(&log_entry.filename)?.trim_end_matches(char::from(0));
            if filename.starts_with("/usr/bin/") {
                println!("Executing command: {}", filename);
                log_file.write_all(format!("Executing command: {}\n", filename).as_bytes()).await?;
            }
        }
    }

    // Detach the program
    program.detach(link_id)?;
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_bpf_test().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bpf_program() {
        let result = run_bpf_test().await;
        assert!(result.is_ok());
    }
}
