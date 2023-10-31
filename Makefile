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
	dotenv cargo tauri dev
.PHONY: run


# =============================================================================
# CI
# =============================================================================
ci: generate web-ci core-ci e2e-test  ## Run CI tasks
.PHONY: ci

generate: web-generate core-generate  ## Generate stubs
.PHONY: generate

format: web-format core-format  ## Run autoformatters
.PHONY: format

lint: generate web-lint core-lint  ## Run linters
.PHONY: lint

scan: web-scan core-scan  ## Run scans
	checkov --quiet --directory .
.PHONY: scan

test: generate web-test core-test  ## Run tests
.PHONY: test

benchmark: web-benchmark core-benchmark  ## Run benchmarks
.PHONY: benchmark

e2e-test:  ## Run e2e tests
	cd src-tauri && cargo build --release && cd -
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
