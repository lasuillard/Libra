#!/usr/bin/env -S make -f

MAKEFLAGS += --warn-undefined-variable
MAKEFLAGS += --no-builtin-rules
MAKEFLAGS += --silent

-include Makefile.*

SHELL := bash
.ONESHELL:
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
.DEFAULT_GOAL := help

help: Makefile  ## Show help
	for makefile in $(MAKEFILE_LIST)
	do
		@echo "$${makefile}"
		@grep -E '(^[a-zA-Z_-]+:.*?##.*$$)|(^##)' "$${makefile}" | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[32m%-30s\033[0m %s\n", $$1, $$2}' | sed -e 's/\[32m##/[33m/'
	done


# =============================================================================
# Common
# =============================================================================
install:  ## Install the app locally
	cargo fetch
	pnpm install
.PHONY: install

init:  ## Initialize project repository
	pre-commit autoupdate
	pre-commit install --install-hooks --hook-type pre-commit --hook-type commit-msg
.PHONY: init

run:  ## Run development application
	dotenv sea migrate --migration-dir ./core/migration
	dotenv cargo tauri dev
.PHONY: run


# =============================================================================
# CI
# =============================================================================
ci: generate lint scan test benchmark e2e-test  ## Run CI tasks
.PHONY: ci

generate:  ## Generate stubs
	pnpm exec svelte-kit sync
.PHONY: generate

format:  ## Run autoformatters
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged --allow-no-vcs
	pnpm exec prettier --list-different --write .
	pnpm exec eslint --fix .
.PHONY: format

lint: generate  ## Run linters
	cargo fmt --check
	cargo clippy
	pnpm exec prettier --check .
	pnpm exec eslint .
	pnpm exec tsc --noEmit
.PHONY: lint

scan:  ## Run scans
	checkov --quiet --directory .
.PHONY: scan

test: generate ## Run tests
	cargo llvm-cov nextest --workspace --lcov --output-path lcov.info \
		&& cargo llvm-cov report --summary-only
	pnpm run test
.PHONY: test

benchmark:  ## Run benchmarks
	cargo bench --workspace
.PHONY: benchmark

e2e-test:  ## Run e2e tests
	cargo build --release
	xvfb-run pnpm run e2e
.PHONY: e2e-test

build: generate  ## Build application
	cargo tauri build
.PHONY: build

docs:  ## Generate dev documents

.PHONY: docs


# =============================================================================
# Handy Scripts
# =============================================================================
clean:  ## Remove temporary files
	rm -rf .svelte-kit/ build/ coverage/ dist/ lcov.info rustc-ice-*.txt
	find . -path '*.log*' -delete
.PHONY: clean
