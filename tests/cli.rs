use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.assert().success();
}

#[test]
fn runs_output() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}