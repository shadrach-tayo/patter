use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[tokio::test]
async fn run_patter() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("patter")?;
    cmd.arg("action").arg("pin_file");
    cmd.arg("file_path").arg("./test.json");
    cmd.arg("provider").arg("web3");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("error"));
    Ok(())
}
