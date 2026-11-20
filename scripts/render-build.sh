#!/usr/bin/env bash
set -euo pipefail

rustup target add wasm32-unknown-unknown

# Install prebuilt, checksum-verified Serenade CLI binaries and the verified Trunk
# bundle (so serenade-ssg skips its first-use Trunk download). Channels:
#   SERENADE_CLI_CHANNEL=release      (default) latest stable release, e.g. v1.0.0
#   SERENADE_CLI_CHANNEL=pre-release  rolling `dev` build (serenade-cli-pre-release)
# serenade-ssg is compiled from crates.io when no prebuilt binary is available.
REPOSITORY="https://github.com/Interchouette-ITC/Serenade"
TARGET_TRIPLE="x86_64-unknown-linux-musl"
case "${SERENADE_CLI_CHANNEL:-release}" in
    release)
        LATEST_URL="$(curl -fsSL --retry 3 -o /dev/null -w '%{url_effective}' "$REPOSITORY/releases/latest" || true)"
        TAG="${LATEST_URL##*/tag/}"
        [[ "$LATEST_URL" == */tag/* ]] || TAG=""
        SUFFIX="$TAG-$TARGET_TRIPLE.tar.gz"
        ;;
    pre-release)
        TAG="serenade-cli-pre-release"
        SUFFIX="$TARGET_TRIPLE.tar.gz"
        ;;
    *)
        echo "Unknown SERENADE_CLI_CHANNEL: $SERENADE_CLI_CHANNEL (release or pre-release)"
        exit 1
        ;;
esac
DOWNLOAD_DIR="$(mktemp -d)"
trap 'rm -rf "$DOWNLOAD_DIR"' EXIT
mkdir -p "$HOME/.local/bin" "$HOME/.serenade/tools"

# Downloads <archive> from the selected release, checks its .sha256 and extracts it.
install_archive() {
    local archive=$1 destination=$2
    curl -fsSL --retry 3 -o "$DOWNLOAD_DIR/$archive" "$REPOSITORY/releases/download/$TAG/$archive" \
        && curl -fsSL --retry 3 -o "$DOWNLOAD_DIR/$archive.sha256" "$REPOSITORY/releases/download/$TAG/$archive.sha256" \
        && (cd "$DOWNLOAD_DIR" && sha256sum -c "$archive.sha256") \
        && tar -xzf "$DOWNLOAD_DIR/$archive" -C "$destination"
}

if [[ -n "$TAG" ]] && install_archive "serenade-cli-$SUFFIX" "$HOME/.local/bin"; then
    echo "Using prebuilt Serenade CLI from $TAG"
    install_archive "serenade-trunk-$SUFFIX" "$HOME/.serenade/tools" \
        || echo "Trunk bundle unavailable; serenade-ssg downloads Trunk on first build"
else
    echo "Prebuilt Serenade CLI unavailable; compiling serenade-ssg"
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
