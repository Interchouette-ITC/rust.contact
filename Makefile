
SITE_BASE_URL ?= https://rust.contact
SERENADE_REPOSITORY ?= https://github.com/Interchouette-ITC/serenade.git
SERENADE_SSG ?= serenade-ssg

.PHONY: build tools test preview clean

# Build the hydration bundle with Serenade's pinned Trunk, then prerender every
# route into complete documents carrying Trunk's generated assets.
build:
	SITE_BASE_URL="$(SITE_BASE_URL)" $(SERENADE_SSG) --release --app-dir apps/site

# Install the Serenade SSG command.
tools:
	cargo install --locked --git $(SERENADE_REPOSITORY) --branch dev serenade-cli --bin serenade-ssg

test:
	cargo test --locked --manifest-path apps/site/Cargo.toml --no-default-features --features ssr

preview:
	python3 -m http.server 4174 --bind 127.0.0.1 --directory apps/site/dist

clean:
	cargo clean --manifest-path Cargo.toml
	rm -rf apps/site/dist
