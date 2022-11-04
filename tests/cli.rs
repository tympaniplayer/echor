use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn hello1() {
    let expected = get_expected_from_file("tests/expected/hello1.txt");
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn hello2() {
    let expected = get_expected_from_file("tests/expected/hello2.txt");
    let mut cmd = get_echor_command();
    let args = vec!["Hello", "there"];
    cmd.args(args).assert().success().stdout(expected);
}

#[test]
fn hello1_n() {
    let expected = get_expected_from_file("tests/expected/hello1.n.txt");
    let mut cmd = get_echor_command();
    cmd.arg("-n")
        .arg("Hello  there")
        .assert()
        .success()
        .stdout(expected);
}

#[test]
fn hello2_n() {
    let expected = get_expected_from_file("tests/expected/hello2.n.txt");
    let mut cmd = get_echor_command();
    let args = vec!["-n", "Hello", "there"];
    cmd.args(args).assert().success().stdout(expected);
}

fn get_echor_command() -> Command {
    Command::cargo_bin("echor").unwrap()
}

fn get_expected_from_file(outfile: &str) -> String {
    fs::read_to_string(outfile).unwrap()
}
