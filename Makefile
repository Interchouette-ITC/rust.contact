
SITE_BASE_URL ?= https://rust.contact

.PHONY: build test preview clean

build:
	cd apps/site && SITE_BASE_URL="$(SITE_BASE_URL)" trunk build --release --locked --features hydrate
	SITE_BASE_URL="$(SITE_BASE_URL)" cargo run --locked --release --manifest-path apps/site/Cargo.toml --no-default-features --features ssr --bin prerender

test:
	cargo test --locked --manifest-path apps/site/Cargo.toml --no-default-features --features ssr

preview:
	python3 -m http.server 4174 --bind 127.0.0.1 --directory apps/site/dist

clean:
	cargo clean --manifest-path Cargo.toml
	rm -rf apps/site/dist
