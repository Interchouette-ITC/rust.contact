
#!/usr/bin/env bash
set -euo pipefail

mkdir -p "$HOME/.local/bin"

curl -fsSL https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz \
  | tar -xz -C "$HOME/.local/bin"

export PATH="$HOME/.local/bin:$PATH"

rustup target add wasm32-unknown-unknown

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

# Remove project pages from the main site's published output.
# They remain available in their respective Render deployments.
for PAGE in serenade rustashop rangular opentrading interfire itcy; do
    rm -rf "$DIST/$PAGE"
done
