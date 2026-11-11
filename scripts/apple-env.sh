#!/bin/sh
# Hands the Apple signing secrets to the macOS build (release.yml, nightly.yml, build.yml), so
# Tauri signs the app with the Developer ID certificate and has Apple notarize it. Without the
# secrets (a fork, or before they are set) it does nothing and the app is ad-hoc signed, as
# tauri.conf.json's signingIdentity "-" asks. The wiki's Development page, "macOS signing", says
# where each secret comes from.
set -eu
if [ -z "${APPLE_CERTIFICATE:-}" ]; then
  echo "No Apple certificate in this repository's secrets: the app is ad-hoc signed."
  exit 0
fi
# Tauri reads the App Store Connect key from a file.
key="$RUNNER_TEMP/AuthKey_$APPLE_API_KEY.p8"
printf '%s\n' "$APPLE_API_KEY_P8" > "$key"
{
  echo "APPLE_CERTIFICATE=$APPLE_CERTIFICATE"
  echo "APPLE_CERTIFICATE_PASSWORD=$APPLE_CERTIFICATE_PASSWORD"
  echo "APPLE_SIGNING_IDENTITY=$APPLE_SIGNING_IDENTITY"
  echo "APPLE_API_KEY=$APPLE_API_KEY"
  echo "APPLE_API_ISSUER=$APPLE_API_ISSUER"
  echo "APPLE_API_KEY_PATH=$key"
} >> "$GITHUB_ENV"
echo "Signing as: $APPLE_SIGNING_IDENTITY"
