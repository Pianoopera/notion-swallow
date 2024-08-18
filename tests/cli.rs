use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;


#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.arg("databases");
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
    runs_output(&["databases"], "tests/expected/databases1.txt")
}

#[test]
fn databases2() -> TestResult {
    runs_output(&["databases", "--id", "12345678-1234-1234-1234-123456789012"], "tests/expected/databases2.txt")
}

#[test]
fn databases_post1() -> TestResult {
    runs_output(&[
        "databases",
        "-x",
        "POST",
        "--file",
        "tests/fixture/post/request1.json",
    ], "tests/expected/databases_post1.txt")
}

#[test]
fn qdatabases_post() -> TestResult {
    runs_output(&[
        "qdatabases",
        "--id",
        "897e5a76ae524b489fdfe71f5945d1af",
        "--file",
        "tests/fixture/post/request2.json",
    ], "tests/expected/query_databases_post2.txt")
}

#[test]
fn databases_post3() -> TestResult {
    runs_output(&[
        "databases",
        "-x",
        "PATCH",
        "-i",
        "668d797c-76fa-4934-9b05-ad288df2d136",
        "-f",
        "tests/fixture/patch/request1.json",
    ], "tests/expected/databases_patch1.txt")
}