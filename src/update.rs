use anyhow::{Context, Result};
use reqwest::Client;
use semver::Version;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

const DEFAULT_REPO: &str = "amogower/freeagent-cli";
const UPDATE_ENV_SKIP: &str = "FREEAGENT_NO_UPDATE";
const UPDATE_ENV_REPO: &str = "FREEAGENT_GITHUB_REPO";

#[derive(Debug, Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Debug, Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

pub async fn maybe_auto_update(disabled: bool) -> Result<bool> {
    if disabled || env::var_os(UPDATE_ENV_SKIP).is_some() {
        return Ok(false);
    }

    if !io::stdin().is_terminal() {
        return Ok(false);
    }

    let repo = env::var(UPDATE_ENV_REPO).unwrap_or_else(|_| DEFAULT_REPO.to_string());
    let client = new_http_client()?;

    let release = match fetch_latest_release(&client, &repo).await {
        Ok(release) => release,
        Err(err) => {
            eprintln!("Auto-update check failed: {err}");
            return Ok(false);
        }
    };

    let latest_version = match parse_version(&release.tag_name) {
        Some(version) => version,
        None => return Ok(false),
    };
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))
        .context("Failed to parse current version")?;

    if latest_version <= current_version {
        return Ok(false);
    }

    if !prompt_update(&current_version, &latest_version)? {
        return Ok(false);
    }

    if let Err(err) = perform_update(&client, &release).await {
        eprintln!("Auto-update failed: {err}");
        return Ok(false);
    }

    eprintln!(
        "Updated to v{}. Please re-run your command.",
        latest_version
    );
    Ok(true)
}

pub async fn run_update(force: bool) -> Result<()> {
    let repo = env::var(UPDATE_ENV_REPO).unwrap_or_else(|_| DEFAULT_REPO.to_string());
    let client = new_http_client()?;

    let release = fetch_latest_release(&client, &repo).await?;
    let latest_version = parse_version(&release.tag_name)
        .context("Failed to parse latest release version tag")?;
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"))
        .context("Failed to parse current version")?;

    if latest_version <= current_version {
        eprintln!("Already on the latest version (v{}).", current_version);
        return Ok(());
    }

    let should_update = if force {
        true
    } else {
        if !io::stdin().is_terminal() {
            return Err(anyhow::anyhow!(
                "Non-interactive session: pass --yes to update"
            ));
        }
        prompt_update(&current_version, &latest_version)?
    };

    if !should_update {
        return Ok(());
    }

    perform_update(&client, &release).await?;
    eprintln!(
        "Updated to v{}. Please re-run your command.",
        latest_version
    );
    Ok(())
}

fn new_http_client() -> Result<Client> {
    Client::builder()
        .user_agent(format!("freeagent/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .context("Failed to build update HTTP client")
}

fn prompt_update(current: &Version, latest: &Version) -> Result<bool> {
    eprint!(
        "A new version is available (v{} → v{}). Update now? [y/N]: ",
        current, latest
    );
    io::stderr().flush().ok();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .context("Failed to read update prompt response")?;
    let response = input.trim().to_ascii_lowercase();
    Ok(matches!(response.as_str(), "y" | "yes"))
}

async fn fetch_latest_release(client: &Client, repo: &str) -> Result<Release> {
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to fetch latest release metadata")?;

    let response = response
        .error_for_status()
        .context("GitHub API responded with an error")?;
    let release = response
        .json::<Release>()
        .await
        .context("Failed to parse GitHub release metadata")?;
    Ok(release)
}

async fn perform_update(client: &Client, release: &Release) -> Result<()> {
    let target = target_triple().context("Unsupported platform for auto-update")?;
    let asset = select_asset(release, &target)
        .context("Could not find a matching release asset for this platform")?;

    let tarball_bytes = download_bytes(client, &asset.browser_download_url).await?;
    verify_checksum(client, release, &asset, &tarball_bytes).await?;

    let temp_dir = TempDir::new()?;
    extract_tarball(&tarball_bytes, temp_dir.path())?;

    let extracted = temp_dir.path().join("freeagent");
    if !extracted.exists() {
        return Err(anyhow::anyhow!(
            "Downloaded release did not contain the freeagent binary"
        ));
    }

    install_binary(&extracted)?;
    Ok(())
}

fn select_asset<'a>(release: &'a Release, target: &str) -> Option<&'a Asset> {
    let suffix = format!("-{target}.tar.gz");
    release.assets.iter().find(|asset| {
        asset.name.starts_with("freeagent-") && asset.name.ends_with(&suffix)
    })
}

async fn download_bytes(client: &Client, url: &str) -> Result<Vec<u8>> {
    let response = client
        .get(url)
        .send()
        .await
        .context("Failed to download release asset")?;
    let response = response
        .error_for_status()
        .context("Release asset download failed")?;
    let bytes = response.bytes().await.context("Failed to read asset")?;
    Ok(bytes.to_vec())
}

async fn verify_checksum(client: &Client, release: &Release, asset: &Asset, data: &[u8]) -> Result<()> {
    let sha_asset = release.assets.iter().find(|item| item.name == "SHA256SUMS");
    let Some(sha_asset) = sha_asset else {
        return Ok(());
    };

    let sha_data = download_bytes(client, &sha_asset.browser_download_url).await?;
    let sha_text = String::from_utf8_lossy(&sha_data);
    let expected = sha_text
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let hash = parts.next()?;
            let name = parts.next()?;
            if name == asset.name {
                Some(hash.to_string())
            } else {
                None
            }
        })
        .next();

    let Some(expected) = expected else {
        return Ok(());
    };

    let actual = sha256_hex(data);
    if expected != actual {
        return Err(anyhow::anyhow!("Checksum verification failed"));
    }
    Ok(())
}

fn extract_tarball(bytes: &[u8], dest: &Path) -> Result<()> {
    let decoder = flate2::read::GzDecoder::new(bytes);
    let mut archive = tar::Archive::new(decoder);
    archive
        .unpack(dest)
        .context("Failed to extract release archive")?;
    Ok(())
}

fn install_binary(extracted: &Path) -> Result<()> {
    let current_exe = env::current_exe().context("Failed to locate current executable")?;
    let dest_dir = current_exe
        .parent()
        .context("Failed to resolve executable directory")?;
    let temp_dest = dest_dir.join(format!(".freeagent-{}.new", std::process::id()));

    fs::copy(extracted, &temp_dest).context("Failed to stage updated binary")?;
    set_executable(&temp_dest)?;

    if let Err(rename_err) = fs::rename(&temp_dest, &current_exe) {
        let copy_result = fs::copy(&temp_dest, &current_exe);
        let _ = fs::remove_file(&temp_dest);
        if let Err(copy_err) = copy_result {
            if matches!(
                rename_err.kind(),
                io::ErrorKind::PermissionDenied | io::ErrorKind::Other
            ) || matches!(copy_err.kind(), io::ErrorKind::PermissionDenied)
            {
                return Err(anyhow::anyhow!(
                    "Permission denied writing to {}",
                    current_exe.display()
                ));
            }
            return Err(copy_err).context("Failed to install updated binary");
        }
    }

    Ok(())
}

fn set_executable(path: &Path) -> Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = fs::Permissions::from_mode(0o755);
        fs::set_permissions(path, perms).context("Failed to set executable permissions")?;
    }
    Ok(())
}

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}

fn parse_version(tag: &str) -> Option<Version> {
    let cleaned = tag.trim().trim_start_matches('v');
    Version::parse(cleaned).ok()
}

fn target_triple() -> Option<String> {
    let arch = match env::consts::ARCH {
        "x86_64" => "x86_64",
        "aarch64" => "aarch64",
        "arm" => "aarch64",
        "amd64" => "x86_64",
        _ => return None,
    };

    let os = match env::consts::OS {
        "macos" => "apple-darwin",
        "linux" => "unknown-linux-gnu",
        _ => return None,
    };

    Some(format!("{arch}-{os}"))
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new() -> Result<Self> {
        let path = env::temp_dir().join(format!("freeagent-update-{}", Uuid::new_v4()));
        fs::create_dir_all(&path).context("Failed to create update temp directory")?;
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}
