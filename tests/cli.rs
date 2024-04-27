use assert_cmd::prelude::*; // Add methods on commands
use assert_fs::prelude::*;
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("tracer_cli")?; // Adjust the binary name to "tracer_cli"

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("tracer_cli")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}
