use assert_cmd::Command;
use chrono::{Duration, Utc};
use directories::ProjectDirs;
use predicates::prelude::*;
use serial_test::serial;
use serde_json::json;
use std::fs;
use tempfile::tempdir;

struct EnvGuard {
    home: Option<String>,
    xdg_config_home: Option<String>,
    appdata: Option<String>,
    local_appdata: Option<String>,
    userprofile: Option<String>,
}

impl EnvGuard {
    fn new(temp_path: &str) -> Self {
        let guard = Self {
            home: std::env::var("HOME").ok(),
            xdg_config_home: std::env::var("XDG_CONFIG_HOME").ok(),
            appdata: std::env::var("APPDATA").ok(),
            local_appdata: std::env::var("LOCALAPPDATA").ok(),
            userprofile: std::env::var("USERPROFILE").ok(),
        };

        std::env::set_var("HOME", temp_path);
        std::env::set_var("XDG_CONFIG_HOME", temp_path);
        std::env::set_var("APPDATA", temp_path);
        std::env::set_var("LOCALAPPDATA", temp_path);
        std::env::set_var("USERPROFILE", temp_path);

        guard
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(value) = &self.home {
            std::env::set_var("HOME", value);
        } else {
            std::env::remove_var("HOME");
        }

        if let Some(value) = &self.xdg_config_home {
            std::env::set_var("XDG_CONFIG_HOME", value);
        } else {
            std::env::remove_var("XDG_CONFIG_HOME");
        }

        if let Some(value) = &self.appdata {
            std::env::set_var("APPDATA", value);
        } else {
            std::env::remove_var("APPDATA");
        }

        if let Some(value) = &self.local_appdata {
            std::env::set_var("LOCALAPPDATA", value);
        } else {
            std::env::remove_var("LOCALAPPDATA");
        }

        if let Some(value) = &self.userprofile {
            std::env::set_var("USERPROFILE", value);
        } else {
            std::env::remove_var("USERPROFILE");
        }
    }
}

fn apply_env(cmd: &mut Command, temp_path: &str) {
    cmd.env("HOME", temp_path)
        .env("XDG_CONFIG_HOME", temp_path)
        .env("APPDATA", temp_path)
        .env("LOCALAPPDATA", temp_path)
        .env("USERPROFILE", temp_path)
        .env("NO_COLOR", "1");
}

fn token_file_path() -> std::path::PathBuf {
    ProjectDirs::from("com", "freeagent", "freeagent-cli")
        .expect("project dirs")
        .config_dir()
        .join("tokens.json")
}

fn write_tokens_file(path: &std::path::Path) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create config dir");
    }
    let now = Utc::now();
    let payload = json!({
        "access_token": "test-access",
        "refresh_token": "test-refresh",
        "expires_at": (now + Duration::hours(1)).to_rfc3339(),
        "sandbox": false,
        "last_refreshed": now.to_rfc3339(),
    });
    fs::write(path, serde_json::to_string_pretty(&payload).unwrap()).expect("write tokens");
}

#[test]
#[serial]
fn status_reports_not_logged_in_when_missing_tokens() {
    let temp_dir = tempdir().expect("temp dir");
    let temp_path = temp_dir.path().to_str().expect("temp path");
    let _guard = EnvGuard::new(temp_path);

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("freeagent"));
    apply_env(&mut cmd, temp_path);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Not logged in"));
}

#[test]
#[serial]
fn status_reports_logged_in_when_tokens_present() {
    let temp_dir = tempdir().expect("temp dir");
    let temp_path = temp_dir.path().to_str().expect("temp path");
    let _guard = EnvGuard::new(temp_path);

    let token_path = token_file_path();
    write_tokens_file(&token_path);

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("freeagent"));
    apply_env(&mut cmd, temp_path);
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Logged in"))
        .stdout(predicate::str::contains("Environment: Production"));
}

#[test]
#[serial]
fn logout_deletes_tokens_file() {
    let temp_dir = tempdir().expect("temp dir");
    let temp_path = temp_dir.path().to_str().expect("temp path");
    let _guard = EnvGuard::new(temp_path);

    let token_path = token_file_path();
    write_tokens_file(&token_path);
    assert!(token_path.exists());

    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("freeagent"));
    apply_env(&mut cmd, temp_path);
    cmd.arg("logout");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Successfully logged out"));

    assert!(!token_path.exists());
}
