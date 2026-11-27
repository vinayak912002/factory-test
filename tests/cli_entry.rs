use assert_cmd::Command;

/// @AC-042: `tldr --version` prints the tool name, version, and client
/// specification version, then exits 0 (BR-31: `tldr 1.0.0 (Client Specification 2.3)`).
#[test]
fn ac_042__version_prints_name_version_and_spec_then_exits_0() {
    let output = Command::cargo_bin("tldr")
        .expect("tldr binary is built")
        .arg("--version")
        .output()
        .expect("tldr runs");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("tldr 1.0.0 (Client Specification 2.3)"),
        "stdout was: {stdout:?}"
    );
}

/// @AC-041: `tldr` with no arguments prints help to stderr and exits 1 (BR-30).
#[test]
fn ac_041__no_arguments_prints_help_to_stderr_and_exits_1() {
    let output = Command::cargo_bin("tldr")
        .expect("tldr binary is built")
        .output()
        .expect("tldr runs");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Usage:"),
        "stderr was: {stderr:?}; help text (with a `Usage:` line) must be printed to stderr"
    );
    assert!(
        output.stdout.is_empty(),
        "stdout was: {:?}; help must go to stderr only",
        String::from_utf8_lossy(&output.stdout)
    );
}

/// @AC-043: `tldr --platform invalid_plat` exits with status 2, the argument
/// parsing error status (BR-14: valid platform names are the nine in VALID_PLATFORMS).
#[test]
fn ac_043__invalid_platform_flag_exits_2() {
    let output = Command::cargo_bin("tldr")
        .expect("tldr binary is built")
        .args(["--platform", "invalid_plat"])
        .output()
        .expect("tldr runs");

    assert_eq!(output.status.code(), Some(2));
}
