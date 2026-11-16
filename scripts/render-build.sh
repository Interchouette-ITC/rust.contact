
#!/usr/bin/env bash
set -euo pipefail

rustup target add wasm32-unknown-unknown

# serenade-ssg installs and runs Serenade's pinned Trunk release.
make tools
export PATH="$HOME/.cargo/bin:$PATH"

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
