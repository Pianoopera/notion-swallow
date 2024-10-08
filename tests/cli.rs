use assert_cmd::Command;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("notion-swallow").unwrap();
    cmd.arg("databases");
    cmd.assert().success();
}

fn runs_output(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;

    // notion api の curl コマンドを作成する
    Command::cargo_bin("notion-swallow")
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
    runs_output(
        &["databases", "--id", "12345678-1234-1234-1234-123456789012"],
        "tests/expected/databases2.txt",
    )
}

#[test]
fn databases_post1() -> TestResult {
    runs_output(
        &[
            "databases",
            "-x",
            "POST",
            "--file",
            "tests/fixture/post/request1.json",
        ],
        "tests/expected/databases_post1.txt",
    )
}

#[test]
fn query_databases_post() -> TestResult {
    runs_output(
        &[
            "query_databases",
            "--id",
            "897e5a76ae524b489fdfe71f5945d1af",
            "--file",
            "tests/fixture/post/request2.json",
        ],
        "tests/expected/query_databases_post2.txt",
    )
}

#[test]
fn databases_patch1() -> TestResult {
    runs_output(
        &[
            "databases",
            "-x",
            "PATCH",
            "-i",
            "668d797c-76fa-4934-9b05-ad288df2d136",
            "-f",
            "tests/fixture/patch/request1.json",
        ],
        "tests/expected/databases_patch1.txt",
    )
}

#[test]
fn pages_create() -> TestResult {
    runs_output(
        &[
            "pages",
            "-x",
            "POST",
            "-f",
            "tests/fixture/post/create_request1.json",
        ],
        "tests/expected/pages_create1.txt",
    )
}

#[test]
fn pages_retrieve() -> TestResult {
    runs_output(
        &["pages", "-i", "12345678-1234-1234-1234-123456789012"],
        "tests/expected/pages_retrieve1.txt",
    )
}

#[test]
fn pages_property_item() -> TestResult {
    runs_output(
        &[
            "property_pages",
            "-i",
            "12345678-1234-1234-1234-123456789012",
            "-p",
            "aBcd123",
        ],
        "tests/expected/property_pages_retrieve1.txt",
    )
}

#[test]
fn pages_updated() -> TestResult {
    runs_output(
        &[
            "pages",
            "-x",
            "PATCH",
            "-i",
            "12345678-1234-1234-1234-123456789012",
            "-f",
            "tests/fixture/patch/pages_patch_request1.json",
        ],
        "tests/expected/pages_patch1.txt",
    )
}

#[test]
fn append_blocks() -> TestResult {
    runs_output(
        &[
            "append_blocks",
            "-x",
            "PATCH",
            "-i",
            "12345678-1234-1234-1234-123456789012",
            "-f",
            "tests/fixture/patch/append_blocks_request1.json",
        ],
        "tests/expected/append_blocks1.txt",
    )
}

#[test]
fn update_block() -> TestResult {
    runs_output(
        &[
            "blocks",
            "-x",
            "PATCH",
            "-i",
            "12345678-1234-1234-1234-123456789012",
            "-f",
            "tests/fixture/patch/blocks_update_request1.json",
        ],
        "tests/expected/blocks_update1.txt",
    )
}

#[test]
fn delete_block() -> TestResult {
    runs_output(
        &[
            "blocks",
            "-x",
            "DELETE",
            "-i",
            "12345678-1234-1234-1234-123456789012",
        ],
        "tests/expected/bloks_delete1.txt",
    )
}

#[test]
fn ret_blocks() -> TestResult {
    runs_output(
        &["blocks", "-i", "12345678-1234-1234-1234-123456789012"],
        "tests/expected/blocks_ret1.txt",
    )
}

#[test]
fn block_children() -> TestResult {
    runs_output(
        &[
            "children_blocks",
            "-i",
            "12345678-1234-1234-1234-123456789012",
        ],
        "tests/expected/child_blocks.txt",
    )
}

#[test]
fn search_by_title() -> TestResult {
    runs_output(
        &[
            "search",
            "-f",
            "tests/fixture/post/search_by_title_request1.json",
        ],
        "tests/expected/search_by_title.txt",
    )
}
