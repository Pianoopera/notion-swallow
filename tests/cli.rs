use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("rf-notion").unwrap();
    cmd.assert().success();
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
