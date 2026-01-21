use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn cli_help_includes_core_commands() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("freeagent"));
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("freeagent"))
        .stdout(predicate::str::contains("login"))
        .stdout(predicate::str::contains("invoices"));
}

#[test]
fn cli_version_matches_package() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("freeagent"));
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
