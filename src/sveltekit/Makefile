help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	dprint --version || cargo install --locked dprint

	pnpm i
	pnpx husky install

clean: ## Clean build output
	rm -rf build

dev: ## Develop the app.
	pnpm run dev

build: ## Build the app.
	pnpm run build-only

fmt: ## Format the codebase.
	dprint fmt --config configs/dprint.json
	pnpm run fmt

fmt_check: ## Check is the codebase properly formatted.
	dprint check --config configs/dprint.json

lint: fmt_check ## Lint the codebase.
	pnpm run lint

test: ## Test the codebase.
	pnpm run test-e2e

comply: fmt lint ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check: fmt_check lint test_ci  ## Check if the repository comply with the rules. Mostly used in CI.

release: ## Create a new release. Example `make release version=v2.2.0`
	bash scripts/release.sh $(version)

##
## Misc

check_dependencies: ## Check dependencies health.
	ncu
