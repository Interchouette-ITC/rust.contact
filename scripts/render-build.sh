#!/usr/bin/env bash
set -euo pipefail

rustup target add wasm32-unknown-unknown

# Download the prebuilt, checksum-verified serenade-ssg from the rolling
# serenade-cli-dev pre-release; fall back to compiling it if unavailable.
# Trunk is not installed here: serenade-ssg downloads and verifies Serenade's
# managed Trunk (fork release trunk-itc-dev) on its first build.
RELEASE="https://github.com/Interchouette-ITC/Serenade/releases/download/serenade-cli-dev"
TARGET_TRIPLE="x86_64-unknown-linux-musl"
ARCHIVE="serenade-cli-$TARGET_TRIPLE.tar.gz"
DOWNLOAD_DIR="$(mktemp -d)"
trap 'rm -rf "$DOWNLOAD_DIR"' EXIT
mkdir -p "$HOME/.local/bin"
if curl -fsSL --retry 3 -o "$DOWNLOAD_DIR/$ARCHIVE" "$RELEASE/$ARCHIVE" \
    && curl -fsSL --retry 3 -o "$DOWNLOAD_DIR/$ARCHIVE.sha256" "$RELEASE/$ARCHIVE.sha256" \
    && (cd "$DOWNLOAD_DIR" && sha256sum -c "$ARCHIVE.sha256") \
    && tar -xzf "$DOWNLOAD_DIR/$ARCHIVE" -C "$HOME/.local/bin"; then
    echo "Using prebuilt serenade-cli-dev binaries"
else
    echo "Prebuilt binaries unavailable; compiling serenade-ssg"
    make tools
fi
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"

make build

DIST="apps/site/dist"

# Copy shared Trunk assets into each subdomain's published directory.
for PAGE in serenade rustashop rangular opentrading interfire itcy; do
    TARGET="$DIST/$PAGE"

    if [[ ! -f "$TARGET/index.html" ]]; then
        echo "Missing prerendered page: $TARGET/index.html"
        exit 1
    fi

    find "$DIST" -maxdepth 1 -type f ! -name index.html \
        -exec cp -f -t "$TARGET" -- {} +

    echo "Prepared assets for $PAGE"
done

# Remove project pages only from the rust.contact Render deployment.
if [[ "${RENDER_SERVICE_ID:-}" == "srv-dbla25jbc2fs73dlj0ng" ]]; then
    for PAGE in serenade rustashop rangular opentrading interfire itcy; do
        rm -rf "$DIST/$PAGE"
    done
fi
