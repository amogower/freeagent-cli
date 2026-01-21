#!/usr/bin/env node
const fs = require("fs");
const path = require("path");
const { execSync } = require("child_process");

function run(cmd) {
  return execSync(cmd, { stdio: "pipe" }).toString().trim();
}

function runInherit(cmd) {
  execSync(cmd, { stdio: "inherit" });
}

function getBumpType() {
  const args = process.argv.slice(2);
  const argType = args.find((arg) => ["--major", "--minor", "--patch"].includes(arg));
  const envType =
    process.env.npm_config_major ? "--major" :
    process.env.npm_config_minor ? "--minor" :
    process.env.npm_config_patch ? "--patch" :
    null;

  const type = argType || envType;
  if (!type) {
    throw new Error("Provide --major, --minor, or --patch (e.g., npm run release --major)");
  }
  if (args.filter((arg) => ["--major", "--minor", "--patch"].includes(arg)).length > 1) {
    throw new Error("Provide only one of --major, --minor, or --patch.");
  }
  return type.replace("--", "");
}

function isDryRun() {
  const args = process.argv.slice(2);
  return (
    args.includes("--dry-run") ||
    args.includes("--dryrun") ||
    process.env.npm_config_dry_run === "true" ||
    process.env.npm_config_dry_run === "1"
  );
}

function bumpVersion(version, bump) {
  const parts = version.split(".").map((part) => Number(part));
  if (parts.length !== 3 || parts.some((part) => Number.isNaN(part))) {
    throw new Error(`Unsupported version format: ${version}`);
  }
  let [major, minor, patch] = parts;
  if (bump === "major") {
    major += 1;
    minor = 0;
    patch = 0;
  } else if (bump === "minor") {
    minor += 1;
    patch = 0;
  } else if (bump === "patch") {
    patch += 1;
  } else {
    throw new Error(`Unknown bump type: ${bump}`);
  }
  return `${major}.${minor}.${patch}`;
}

function updateCargoToml(filePath, bump) {
  const lines = fs.readFileSync(filePath, "utf8").split(/\r?\n/);
  let inPackage = false;
  let oldVersion = null;

  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    if (line.trim().startsWith("[") && line.trim().endsWith("]")) {
      inPackage = line.trim() === "[package]";
    }
    if (inPackage) {
      const match = line.match(/^version\s*=\s*"([^"]+)"/);
      if (match) {
        oldVersion = match[1];
        const newVersion = bumpVersion(oldVersion, bump);
        lines[i] = line.replace(oldVersion, newVersion);
        return { updated: lines.join("\n"), oldVersion, newVersion };
      }
    }
  }

  throw new Error("Could not find [package] version in Cargo.toml");
}

function updateCargoLock(filePath, packageName, newVersion) {
  if (!fs.existsSync(filePath)) {
    return null;
  }

  const lines = fs.readFileSync(filePath, "utf8").split(/\r?\n/);
  let inPackage = false;
  let nameMatches = false;
  let updated = false;

  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    if (line.trim() === "[[package]]") {
      inPackage = true;
      nameMatches = false;
      continue;
    }
    if (inPackage) {
      const nameMatch = line.match(/^name\s*=\s*"([^"]+)"/);
      if (nameMatch) {
        nameMatches = nameMatch[1] === packageName;
        continue;
      }
      if (nameMatches) {
        const versionMatch = line.match(/^version\s*=\s*"([^"]+)"/);
        if (versionMatch) {
          lines[i] = line.replace(versionMatch[1], newVersion);
          updated = true;
          nameMatches = false;
          inPackage = false;
        }
      }
    }
  }

  if (!updated) {
    return null;
  }

  return lines.join("\n");
}

function ensureCleanGit() {
  const status = run("git status --porcelain");
  if (status.length > 0) {
    throw new Error("Working tree is not clean. Commit or stash changes before releasing.");
  }
}

function main() {
  const bump = getBumpType();
  const dryRun = isDryRun();
  ensureCleanGit();

  const cargoTomlPath = path.join(process.cwd(), "Cargo.toml");
  const cargoLockPath = path.join(process.cwd(), "Cargo.lock");

  const { updated, oldVersion, newVersion } = updateCargoToml(cargoTomlPath, bump);
  if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
    throw new Error(`Invalid version format: ${newVersion}`);
  }
  const tagName = `v${newVersion}`;

  if (dryRun) {
    console.log(`[dry-run] bump: ${oldVersion} -> ${newVersion}`);
    console.log(`[dry-run] update ${cargoTomlPath}`);
    if (fs.existsSync(cargoLockPath)) {
      console.log(`[dry-run] update ${cargoLockPath}`);
    }
    console.log(`[dry-run] git add ${cargoTomlPath} ${fs.existsSync(cargoLockPath) ? cargoLockPath : ""}`.trim());
    console.log(`[dry-run] git commit -m "chore(release): v${newVersion}"`);
    console.log(`[dry-run] git tag ${tagName}`);
    console.log("[dry-run] git push origin HEAD");
    console.log(`[dry-run] git push origin ${tagName}`);
    return;
  }

  fs.writeFileSync(cargoTomlPath, updated, "utf8");

  const packageNameMatch = updated.match(/^name\s*=\s*"([^"]+)"/m);
  const packageName = packageNameMatch ? packageNameMatch[1] : "freeagent";

  const updatedLock = updateCargoLock(cargoLockPath, packageName, newVersion);
  if (updatedLock) {
    fs.writeFileSync(cargoLockPath, updatedLock, "utf8");
  }

  runInherit(`git add ${cargoTomlPath} ${fs.existsSync(cargoLockPath) ? cargoLockPath : ""}`.trim());
  runInherit(`git commit -m "chore(release): v${newVersion}"`);
  runInherit(`git tag ${tagName}`);

  runInherit("git push origin HEAD");
  runInherit(`git push origin ${tagName}`);

  console.log(`Released ${tagName} (was ${oldVersion}).`);
}

try {
  main();
} catch (err) {
  console.error(err.message || err);
  process.exit(1);
}
