#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# OpenClaw Calling Node — Build & Deploy Script
# ═══════════════════════════════════════════════════════════════════════════════
#
# Usage:
#   ./scripts/deploy.sh [command]
#
# Commands:
#   build-debug     Build debug APK
#   build-release   Build release APK (requires keystore)
#   install-debug   Build and install debug APK on connected device
#   install-release Build and install release APK on connected device
#   docker-build    Build using Docker (no local Android SDK needed)
#   clean           Clean build artifacts
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

log_info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

# ── Commands ──

build_debug() {
    log_info "Building debug APK..."
    cd "$PROJECT_DIR"
    ./gradlew assembleDebug --no-daemon
    log_ok "Debug APK built: app/build/outputs/apk/debug/"
}

build_release() {
    log_info "Building release APK..."
    cd "$PROJECT_DIR"

    if [ ! -f "app/release-keystore.jks" ]; then
        log_warn "No release keystore found. Generating a debug-signed release..."
        ./gradlew assembleRelease --no-daemon
    else
        ./gradlew assembleRelease --no-daemon \
            -Pandroid.injected.signing.store.file="$PROJECT_DIR/app/release-keystore.jks" \
            -Pandroid.injected.signing.store.password="${KEYSTORE_PASSWORD:-}" \
            -Pandroid.injected.signing.key.alias="${KEY_ALIAS:-}" \
            -Pandroid.injected.signing.key.password="${KEY_PASSWORD:-}"
    fi

    log_ok "Release APK built: app/build/outputs/apk/release/"
}

install_debug() {
    build_debug
    log_info "Installing debug APK on connected device..."
    adb install -r app/build/outputs/apk/debug/app-debug.apk
    log_ok "Debug APK installed"
    log_info "Launching app..."
    adb shell am start -n com.openclaw.callingnode/.ui.MainActivity
}

install_release() {
    build_release
    log_info "Installing release APK on connected device..."
    adb install -r app/build/outputs/apk/release/app-release.apk
    log_ok "Release APK installed"
    log_info "Launching app..."
    adb shell am start -n com.openclaw.callingnode/.ui.MainActivity
}

docker_build() {
    log_info "Building with Docker..."
    cd "$PROJECT_DIR"
    mkdir -p output

    docker build -t openclaw-android-build .
    docker run --rm -v "$(pwd)/output:/host-output" openclaw-android-build

    log_ok "Docker build complete. APK in: output/"
}

generate_keystore() {
    log_info "Generating release keystore..."
    cd "$PROJECT_DIR/app"

    if [ -f "release-keystore.jks" ]; then
        log_warn "Keystore already exists. Skipping."
        return
    fi

    keytool -genkeypair \
        -v \
        -keystore release-keystore.jks \
        -keyalg RSA \
        -keysize 2048 \
        -validity 10000 \
        -alias openclaw \
        -storepass "${KEYSTORE_PASSWORD:-changeme}" \
        -keypass "${KEY_PASSWORD:-changeme}" \
        -dname "CN=OpenClaw, OU=CallingNode, O=OpenClaw, L=London, ST=England, C=GB"

    log_ok "Keystore generated: app/release-keystore.jks"
    log_warn "IMPORTANT: Never commit this file to version control!"
}

clean() {
    log_info "Cleaning build artifacts..."
    cd "$PROJECT_DIR"
    ./gradlew clean --no-daemon
    rm -rf output/
    log_ok "Clean complete"
}

setup_env() {
    log_info "Setting up development environment..."

    # Check Java
    if command -v java &>/dev/null; then
        JAVA_VER=$(java -version 2>&1 | head -1 | cut -d'"' -f2 | cut -d'.' -f1)
        log_ok "Java $JAVA_VER found"
    else
        log_error "Java not found. Please install JDK 17+"
        exit 1
    fi

    # Check Android SDK
    if [ -n "${ANDROID_HOME:-}" ] && [ -d "$ANDROID_HOME" ]; then
        log_ok "Android SDK found: $ANDROID_HOME"
    else
        log_warn "ANDROID_HOME not set. Set it to your Android SDK path."
    fi

    # Check ADB
    if command -v adb &>/dev/null; then
        log_ok "ADB found"
        DEVICES=$(adb devices | grep -c "device$" || true)
        log_info "$DEVICES device(s) connected"
    else
        log_warn "ADB not found in PATH"
    fi

    log_ok "Environment check complete"
}

# ── Main ──

COMMAND="${1:-help}"

case "$COMMAND" in
    build-debug)     build_debug ;;
    build-release)   build_release ;;
    install-debug)   install_debug ;;
    install-release) install_release ;;
    docker-build)    docker_build ;;
    gen-keystore)    generate_keystore ;;
    clean)           clean ;;
    setup)           setup_env ;;
    help|*)
        echo ""
        echo "OpenClaw Calling Node — Build & Deploy"
        echo ""
        echo "Usage: $0 <command>"
        echo ""
        echo "Commands:"
        echo "  build-debug       Build debug APK"
        echo "  build-release     Build release APK"
        echo "  install-debug     Build and install debug APK on device"
        echo "  install-release   Build and install release APK on device"
        echo "  docker-build      Build using Docker container"
        echo "  gen-keystore      Generate a release signing keystore"
        echo "  clean             Clean build artifacts"
        echo "  setup             Check development environment"
        echo ""
        ;;
esac
