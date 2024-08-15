use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.arg("databases");
    cmd.assert().success();
}

#[test]
fn runs_output() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    // notion api の curl コマンドを作成する
    cmd.arg("hoge");
    cmd.assert().success().stdout("https://api.notion.com/v1/hoge\n");
}