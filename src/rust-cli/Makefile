help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	git-cliff --version || cargo install git-cliff
	cargo-set-version --help || cargo install cargo-edit
	cargo watch --version || cargo install cargo-watch
	cargo outdated --version || cargo install --locked cargo-outdated
	dprint --version || cargo install dprint

dev: ## Develop the app.
	cargo watch -x fmt -x clippy -x run

fmt: ## Format the codebase.
	cargo +nightly fmt --all
	dprint fmt --config configs/dprint.json

fmt_check: ## Check is the codebase properly formatted.
	cargo +nightly fmt --all --check
	dprint check --config configs/dprint.json

lint: ## Lint the codebase.
	cargo clippy --all --all-features --all-targets

doc_check: ## Check the documentation.
	cargo doc --all-features --no-deps

test: ## Lint the codebase.
	cargo test --all --all-features --all-targets

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check:  fmt_check lint test doc_check ## Check if the repository comply with the rules. Mostly used in CI.


release:  ## Create a release
	bash scripts/release.sh $(version)

#
# Misc
#

check_dependencies: ## Check dependencies health.
	cargo +nightly udeps
	cargo outdated --root-deps-only
