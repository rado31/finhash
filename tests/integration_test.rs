use std::process::Command;

#[test]
fn run_with_valid_args() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-N")
        .arg("3")
        .arg("-F")
        .arg("3")
        .output()
        .expect("Failed to execute process");

    assert!(output.status.success());
    let output_str = String::from_utf8_lossy(&output.stdout);

    let expected_hashes_count = 3;
    let hashes: Vec<_> =
        output_str.lines().filter(|line| line.ends_with("000")).collect();

    assert_eq!(hashes.len(), expected_hashes_count);
}

#[test]
fn run_with_invalid_n() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-N")
        .arg("0")
        .arg("-F")
        .arg("0")
        .output()
        .expect("Failed to execute process");

    assert!(!output.status.success());
}

#[test]
fn run_with_missing_args() {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("-F")
        .arg("5")
        .output()
        .expect("Failed to execute process");

    assert!(!output.status.success());
}
