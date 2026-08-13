use std::process::Command;

#[test]
fn converts_csv_to_markdown_table() {
    let output = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg("tests/fixtures/sample.csv")
        .output()
        .expect("failed to run anydoc-cli");

    assert!(output.status.success());
    let markdown = String::from_utf8(output.stdout).unwrap();
    assert!(markdown.contains("| name | city |"));
    assert!(markdown.contains("| Ada | London |"));
}

#[test]
fn writes_output_file_when_requested() {
    let out_path = std::env::temp_dir().join("anydoc-cli-test-output.md");
    let _ = std::fs::remove_file(&out_path);

    let status = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg("tests/fixtures/sample.csv")
        .arg("-o")
        .arg(&out_path)
        .status()
        .expect("failed to run anydoc-cli");

    assert!(status.success());
    let markdown = std::fs::read_to_string(&out_path).unwrap();
    assert!(markdown.contains("| Grace | New York |"));

    std::fs::remove_file(&out_path).ok();
}

#[test]
fn fails_clearly_on_missing_input() {
    let output = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg("tests/fixtures/does-not-exist.docx")
        .output()
        .expect("failed to run anydoc-cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("does-not-exist.docx"));
}
