use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.arg("--type").arg("databases");
    cmd.assert().success();
}

fn runs_output(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    
    // notion api の curl コマンドを作成する
    Command::cargo_bin("rf-notion")
        .unwrap()
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn databases1() -> TestResult {
    runs_output(&["--type", "databases"], "tests/expected/databases1.txt")
}

#[test]
fn databases2() -> TestResult {
    runs_output(&["--type", "databases", "--id", "12345678-1234-1234-1234-123456789012"], "tests/expected/databases2.txt")
}