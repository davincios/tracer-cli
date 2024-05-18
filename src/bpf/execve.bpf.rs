// src/bpf/execve.bpf.rs
#![no_std]
#![no_main]

use aya_bpf::{bindings::*, macros::*, programs::TracePointContext};
use aya_log_ebpf::info;

#[tracepoint(name = "sys_enter_execve")]
pub fn sys_enter_execve(ctx: TracePointContext) -> u32 {
    match unsafe { try_sys_enter_execve(ctx) } {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}

unsafe fn try_sys_enter_execve(ctx: TracePointContext) -> Result<u32, i64> {
    let filename: *const u8 = core::ptr::null();
    bpf_probe_read_user_str(filename as *mut u8, 256, ctx.arg::<*const *const u8>(0)?)?;

    let mut buf = [0u8; 256];
    bpf_probe_read_user_str(&mut buf as *mut _ as *mut u8, 256, filename)?;

    if buf.starts_with(b"/usr/bin/") {
        info!(&ctx, "Executing command: {:?}", &buf);
    }

    Ok(0)
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    core::arch::asm!("ud2");
}
