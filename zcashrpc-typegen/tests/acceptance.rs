//! Acceptance test: runs the application as a subprocess and asserts its
//! output for given argument combinations matches what is expected.
//!
//! Modify and/or delete these as you see fit to test the specific needs of
//! your application.
//!
//! For more information, see:
//! <https://docs.rs/abscissa_core/latest/abscissa_core/testing/index.html>

// Tip: Deny warnings with `RUSTFLAGS="-D warnings"` environment variable in CI

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    trivial_casts,
    unused_lifetimes,
    unused_qualifications
)]

use abscissa_core::testing::prelude::*;
use once_cell::sync::Lazy;
use zcashrpc_typegen::config::ZcashrpcTypegenConfig;

/// Executes your application binary via `cargo run`.
///
/// Storing this value as a [`Lazy`] static ensures that all instances of
/// the runner acquire a mutex when executing commands and inspecting
/// exit statuses, serializing what would otherwise be multithreaded
/// invocations as `cargo test` executes tests in parallel by default.
pub static RUNNER: Lazy<CmdRunner> = Lazy::new(|| CmdRunner::default());

/// Use `ZcashrpcTypegenConfig::default()` value if no config or args
#[test]
fn generate_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner.arg("generate").capture_stdout().run();
    cmd.stdout().expect_line("Hello, world!");
    cmd.wait().unwrap().expect_success();
}

/// Use command-line argument value
#[test]
fn generate_with_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .args(&["generate", "acceptance", "test"])
        .capture_stdout()
        .run();

    cmd.stdout().expect_line("Hello, acceptance test!");
    cmd.wait().unwrap().expect_success();
}

/// Use configured value
#[test]
fn generate_with_config_no_args() {
    let mut config = ZcashrpcTypegenConfig::default();
    config.hello.recipient = "configured recipient".to_owned();
    let expected_line = format!("Hello, {}!", &config.hello.recipient);

    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .config(&config)
        .arg("generate")
        .capture_stdout()
        .run();
    cmd.stdout().expect_line(&expected_line);
    cmd.wait().unwrap().expect_success();
}

/// Override configured value with command-line argument
#[test]
fn generate_with_config_and_args() {
    let mut config = ZcashrpcTypegenConfig::default();
    config.hello.recipient = "configured recipient".to_owned();

    let mut runner = RUNNER.clone();
    let mut cmd = runner
        .config(&config)
        .args(&["generate", "acceptance", "test"])
        .capture_stdout()
        .run();

    cmd.stdout().expect_line("Hello, acceptance test!");
    cmd.wait().unwrap().expect_success();
}

/// Example of a test which matches a regular expression
#[test]
fn version_no_args() {
    let mut runner = RUNNER.clone();
    let mut cmd = runner.arg("version").capture_stdout().run();
    cmd.stdout().expect_regex(r"\A\w+ [\d\.\-]+\z");
}
