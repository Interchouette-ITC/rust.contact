
#!/usr/bin/env bash
set -euo pipefail

rustup target add wasm32-unknown-unknown

# Download prebuilt serenade-ssg and Serenade's pinned Trunk from the rolling
# serenade-cli-dev pre-release; fall back to compiling them if unavailable.
RELEASE="https://github.com/Interchouette-ITC/Serenade/releases/download/serenade-cli-dev"
TARGET_TRIPLE="x86_64-unknown-linux-musl"
mkdir -p "$HOME/.local/bin" "$HOME/.serenade/tools"
if curl -fsSL "$RELEASE/serenade-cli-$TARGET_TRIPLE.tar.gz" | tar -xz -C "$HOME/.local/bin" \
    && curl -fsSL "$RELEASE/serenade-trunk-$TARGET_TRIPLE.tar.gz" | tar -xz -C "$HOME/.serenade/tools"; then
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
