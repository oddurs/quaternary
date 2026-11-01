//! The command line's contract: output streams and exit codes.

use std::process::{Command, Output};

fn q4(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_q4"))
        .args(args)
        .output()
        .expect("the q4 binary runs")
}

#[test]
fn version_prints_the_package_version() {
    let out = q4(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        format!("q4 {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn help_goes_to_stdout_and_succeeds() {
    let out = q4(&["--help"]);
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("usage: q4"));
}

#[test]
fn no_arguments_is_a_usage_error() {
    let out = q4(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("usage: q4"));
}

#[test]
fn an_unknown_argument_is_named_in_the_error() {
    let out = q4(&["distance"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown argument `distance`"));
}
