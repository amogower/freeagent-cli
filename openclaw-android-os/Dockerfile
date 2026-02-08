# ═══════════════════════════════════════════════════════════════════════════════
# OpenClaw Calling Node — Android Build Environment
# ═══════════════════════════════════════════════════════════════════════════════
#
# Multi-stage Docker build for compiling the Android application.
# Produces a signed APK or AAB ready for deployment.
#
# Usage:
#   docker build -t openclaw-android-build .
#   docker run --rm -v $(pwd)/output:/output openclaw-android-build
#
# ═══════════════════════════════════════════════════════════════════════════════

FROM eclipse-temurin:17-jdk AS builder

# ── Android SDK Setup ──
ENV ANDROID_HOME=/opt/android-sdk
ENV ANDROID_SDK_ROOT=${ANDROID_HOME}
ENV PATH="${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${PATH}"

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    git \
    unzip \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Download and install Android command-line tools
RUN mkdir -p ${ANDROID_HOME}/cmdline-tools && \
    cd ${ANDROID_HOME}/cmdline-tools && \
    wget -q https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip -O tools.zip && \
    unzip -q tools.zip && \
    mv cmdline-tools latest && \
    rm tools.zip

# Accept licenses and install SDK components
RUN yes | sdkmanager --licenses > /dev/null 2>&1 && \
    sdkmanager \
    "platform-tools" \
    "platforms;android-35" \
    "build-tools;35.0.0" \
    "ndk;27.0.12077973"

# ── Build Stage ──
WORKDIR /app
COPY . .

# Make gradlew executable
RUN chmod +x ./gradlew

# Build the release APK
RUN ./gradlew assembleRelease --no-daemon --stacktrace

# ── Output Stage ──
FROM alpine:latest AS output

COPY --from=builder /app/app/build/outputs/apk/release/ /output/

# Copy script to extract APK
RUN echo '#!/bin/sh' > /entrypoint.sh && \
    echo 'cp -r /output/* /host-output/ 2>/dev/null || echo "No APK found"' >> /entrypoint.sh && \
    chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
