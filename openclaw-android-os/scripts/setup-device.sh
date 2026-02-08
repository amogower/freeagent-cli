#!/usr/bin/env bash
# ═══════════════════════════════════════════════════════════════════════════════
# OpenClaw Calling Node — Device Setup Script
# ═══════════════════════════════════════════════════════════════════════════════
#
# Configures a connected Android device for optimal OpenClaw Calling Node
# operation. This script:
#
# 1. Disables battery optimization for the app
# 2. Grants required runtime permissions
# 3. Sets the app as a default phone handler (if applicable)
# 4. Configures always-on display settings
# 5. Enables the app to run on boot
#
# Usage:
#   ./scripts/setup-device.sh
#
# Prerequisites:
#   - ADB installed and in PATH
#   - Device connected via USB with debugging enabled
#   - App already installed on the device
#
# ═══════════════════════════════════════════════════════════════════════════════

set -euo pipefail

PACKAGE="com.openclaw.callingnode"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

# Check ADB
if ! command -v adb &>/dev/null; then
    log_error "ADB not found. Please install Android platform-tools."
    exit 1
fi

# Check device connection
DEVICE_COUNT=$(adb devices | grep -c "device$" || true)
if [ "$DEVICE_COUNT" -eq 0 ]; then
    log_error "No device connected. Please connect a device with USB debugging enabled."
    exit 1
fi

log_info "Device found. Configuring for OpenClaw Calling Node..."

# ── 1. Grant Permissions ──
log_info "Granting runtime permissions..."

PERMISSIONS=(
    "android.permission.RECORD_AUDIO"
    "android.permission.CALL_PHONE"
    "android.permission.READ_PHONE_STATE"
    "android.permission.READ_PHONE_NUMBERS"
    "android.permission.POST_NOTIFICATIONS"
    "android.permission.BLUETOOTH_CONNECT"
    "android.permission.CAMERA"
)

for perm in "${PERMISSIONS[@]}"; do
    adb shell pm grant "$PACKAGE" "$perm" 2>/dev/null && \
        log_ok "  Granted: $perm" || \
        log_warn "  Skipped: $perm (may not be applicable)"
done

# ── 2. Disable Battery Optimization ──
log_info "Disabling battery optimization..."
adb shell dumpsys deviceidle whitelist +$PACKAGE 2>/dev/null && \
    log_ok "  Battery optimization disabled" || \
    log_warn "  Could not disable battery optimization"

# ── 3. Allow Background Activity ──
log_info "Configuring background activity..."
adb shell cmd appops set $PACKAGE RUN_IN_BACKGROUND allow 2>/dev/null && \
    log_ok "  Background activity allowed" || \
    log_warn "  Could not set background activity"

adb shell cmd appops set $PACKAGE RUN_ANY_IN_BACKGROUND allow 2>/dev/null && \
    log_ok "  Any background activity allowed" || \
    log_warn "  Could not set any background activity"

# ── 4. Keep Alive Settings ──
log_info "Configuring keep-alive settings..."

# Prevent the system from killing the app
adb shell cmd appops set $PACKAGE SYSTEM_ALERT_WINDOW allow 2>/dev/null || true

# Set high priority
adb shell cmd appops set $PACKAGE WAKE_LOCK allow 2>/dev/null && \
    log_ok "  Wake lock allowed" || \
    log_warn "  Could not set wake lock"

# ── 5. Auto-start on Boot ──
log_info "Enabling auto-start..."
adb shell cmd appops set $PACKAGE BOOT_COMPLETED allow 2>/dev/null && \
    log_ok "  Boot completed receiver enabled" || \
    log_warn "  Could not enable boot receiver"

# ── 6. Network Settings ──
log_info "Configuring network settings..."
adb shell cmd netpolicy add restrict-background-whitelist "$(adb shell cmd package list packages -U $PACKAGE | cut -d: -f3 | head -1)" 2>/dev/null && \
    log_ok "  Background data allowed" || \
    log_warn "  Could not configure background data"

# ── Summary ──
echo ""
log_ok "Device setup complete!"
echo ""
echo "Next steps:"
echo "  1. Open the app and configure API keys in Settings"
echo "  2. Set the Gateway URL to your OpenClaw instance"
echo "  3. Test a call using the dial pad"
echo ""
echo "For dedicated phone mode, consider also:"
echo "  - Enabling 'Stay Awake' in Developer Options"
echo "  - Setting the app as the default launcher (kiosk mode)"
echo "  - Disabling screen timeout"
echo ""
