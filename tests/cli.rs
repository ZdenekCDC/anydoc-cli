use std::process::Command;

#[test]
fn converts_csv_to_markdown_table_on_stdout() {
    let output = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg("--stdout")
        .arg("tests/fixtures/sample.csv")
        .output()
        .expect("failed to run anydoc-cli");

    assert!(output.status.success());
    let markdown = String::from_utf8(output.stdout).unwrap();
    assert!(markdown.contains("| name | city |"));
    assert!(markdown.contains("| Ada | London |"));
}

#[test]
fn writes_sibling_md_file_by_default() {
    let input = std::env::temp_dir().join("anydoc-cli-test-default-output.csv");
    let out_path = input.with_extension("md");
    std::fs::copy("tests/fixtures/sample.csv", &input).unwrap();
    let _ = std::fs::remove_file(&out_path);

    let status = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg(&input)
        .status()
        .expect("failed to run anydoc-cli");

    assert!(status.success());
    let markdown = std::fs::read_to_string(&out_path).unwrap();
    assert!(markdown.contains("| Ada | London |"));

    std::fs::remove_file(&input).ok();
    std::fs::remove_file(&out_path).ok();
}

#[test]
fn batch_converts_multiple_inputs_to_sibling_md_files() {
    let dir = std::env::temp_dir().join("anydoc-cli-test-batch");
    std::fs::create_dir_all(&dir).unwrap();
    let a = dir.join("a.csv");
    let b = dir.join("b.csv");
    std::fs::copy("tests/fixtures/sample.csv", &a).unwrap();
    std::fs::copy("tests/fixtures/sample.csv", &b).unwrap();
    let a_md = a.with_extension("md");
    let b_md = b.with_extension("md");
    let _ = std::fs::remove_file(&a_md);
    let _ = std::fs::remove_file(&b_md);

    let status = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg(&a)
        .arg(&b)
        .status()
        .expect("failed to run anydoc-cli");

    assert!(status.success());
    assert!(std::fs::read_to_string(&a_md).unwrap().contains("| Ada | London |"));
    assert!(std::fs::read_to_string(&b_md).unwrap().contains("| Ada | London |"));

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn output_flag_rejects_multiple_inputs() {
    let output = Command::new(env!("CARGO_BIN_EXE_anydoc-cli"))
        .arg("-o")
        .arg(std::env::temp_dir().join("anydoc-cli-test-rejected.md"))
        .arg("tests/fixtures/sample.csv")
        .arg("tests/fixtures/sample.csv")
        .output()
        .expect("failed to run anydoc-cli");

    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("single input"));
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
