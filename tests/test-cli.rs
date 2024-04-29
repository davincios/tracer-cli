// use anyhow::Result;
// use std::process::Command;
// use std::str;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // Helper function to run the command and capture the output
//     fn run_command(args: &[&str]) -> Command {
//         let mut command = Command::new("cargo");
//         command
//             .arg("run")
//             .arg("--") // This ensures that the arguments are passed to the application, not cargo
//             .args(args);
//         command
//     }

//     #[test]
//     fn test_tracer_setup() -> Result<()> {
//         // Set up the API key for the tests
//         let api_key_setup = run_command(&["setup", "dDRE5rxJEjktQxCtzsYyz"]).output()?;
//         assert!(
//             api_key_setup.status.success(),
//             "Setup failed: {}",
//             str::from_utf8(&api_key_setup.stderr)?
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_tracer_start() -> Result<()> {
//         let output_start = run_command(&["start"]).output()?;
//         assert!(
//             output_start.status.success(),
//             "Start failed: {}",
//             str::from_utf8(&output_start.stderr)?
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_tracer_log() -> Result<()> {
//         // Test 'log'
//         let output_log = run_command(&[
//             "log",
//             "--type",
//             "warning",
//             "QC mapping reads GC content below 61% threshold",
//         ])
//         .output()?;
//         assert!(
//             output_log.status.success(),
//             "Logging failed: {}",
//             str::from_utf8(&output_log.stderr)?
//         );
//         Ok(())
//     }

//     #[test]
//     fn test_tracer_tool() -> Result<()> {
//         // Test 'tool'
//         // please fix the following issue:

//         let output_tool = run_command(&["tool", "BWA_MEM2", "1.0"]).output()?;
//         assert!(
//             output_tool.status.success(),
//             "Tool command failed: {}",
//             str::from_utf8(&output_tool.stderr)?
//         );
//         Ok(())
//     }
//     #[test]
//     fn test_tracer_end() -> Result<()> {
//         // Test 'end'
//         let output_end = run_command(&["end"]).output()?;
//         assert!(
//             output_end.status.success(),
//             "End failed: {}",
//             str::from_utf8(&output_end.stderr)?
//         );
//         Ok(())
//     }
// }
