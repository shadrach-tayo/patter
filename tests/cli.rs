use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[tokio::test]
async fn run_patter_web3() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("patter")?;
    let assert = cmd.arg("-a=pin_file").arg("-f=./test.json").arg("-p=web3").assert();
    assert.success();
    Ok(())
}

#[tokio::test]
async fn pin_file_pinata() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("patter")?;
    let assert = cmd.arg("-a=pin_file").arg("-f=./test.json").arg("-p=pinata").assert();
    assert.success();
    Ok(())
}

#[tokio::test]
async fn fail_arg() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("patter")?;
    let assert = cmd.arg("-a pin_file").arg("-f=./test.json").arg("-p=web3").assert();
    assert
        .failure()
        .stderr(predicate::str::contains("--action pin_file"));
    Ok(())
}
