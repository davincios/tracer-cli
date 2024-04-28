use std::process::Command;

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Output;
    use std::str;

    // Helper function to run the command and capture the output
    fn run_command(args: &[&str]) -> Output {
        Command::new("cargo")
            .arg("run")
            .arg("--") // This ensures that the arguments are passed to the application, not cargo
            .args(args)
            .output()
            .expect("Failed to execute command")
    }

    #[test]
    fn test_cli() -> Result<(), Box<dyn std::error::Error>> {
        // Test 'tracer setup --api-key 1234'
        let output = run_command(&["setup", "--api-key", "1234"]);
        assert!(
            output.status.success(),
            "Setup failed: {}",
            str::from_utf8(&output.stderr)?
        );

        // Test 'tracer start'
        let output = run_command(&["start"]);
        assert!(
            output.status.success(),
            "Start failed: {}",
            str::from_utf8(&output.stderr)?
        );

        // Test 'tracer log --type warning "QC mapping reads GC content below 61% threshold"'
        let output = run_command(&[
            "log",
            "--type",
            "warning",
            "QC mapping reads GC content below 61% threshold",
        ]);
        assert!(
            output.status.success(),
            "Logging failed: {}",
            str::from_utf8(&output.stderr)?
        );

        // Test 'tracer end'
        let output = run_command(&["end"]);
        assert!(
            output.status.success(),
            "End failed: {}",
            str::from_utf8(&output.stderr)?
        );

        Ok(())
    }
}
