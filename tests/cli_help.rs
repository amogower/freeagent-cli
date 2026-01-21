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

#[test]
fn cli_help_includes_extended_commands() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("freeagent"));
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("bank-transaction-explanations"))
        .stdout(predicate::str::contains("journal-sets"))
        .stdout(predicate::str::contains("sales-tax-periods"))
        .stdout(predicate::str::contains("price-list-items"))
        .stdout(predicate::str::contains("properties"))
        .stdout(predicate::str::contains("account-managers"))
        .stdout(predicate::str::contains("capital-asset-types"));
}
