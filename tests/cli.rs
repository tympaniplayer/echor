use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    let expected = get_expected_from_file("tests/expected/hello1.txt")?;
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("Hello there").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello2() -> TestResult {
    let expected = get_expected_from_file("tests/expected/hello2.txt")?;
    let mut cmd = get_echor_command()?;
    let args = vec!["Hello", "there"];
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1_n() -> TestResult {
    let expected = get_expected_from_file("tests/expected/hello1.n.txt")?;
    let mut cmd = get_echor_command()?;
    cmd.arg("-n")
        .arg("Hello  there")
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello2_n() -> TestResult {
    let expected = get_expected_from_file("tests/expected/hello2.n.txt")?;
    let mut cmd = get_echor_command()?;
    let args = vec!["-n", "Hello", "there"];
    cmd.args(args).assert().success().stdout(expected);
    
    Ok(())
}

fn get_echor_command() -> Result<Command, Box<dyn std::error::Error>> {
    let command = Command::cargo_bin("echor")?;
    Ok(command)
}

fn get_expected_from_file(outfile: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = fs::read_to_string(outfile)?;
    Ok(file)
}
