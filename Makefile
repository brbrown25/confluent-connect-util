.PHONY: help build release debug test test-watch lint lint-fix fmt fmt-check coverage clean install-tools check all

# Default target
.DEFAULT_GOAL := help

# Variables
CARGO := cargo
RUSTFLAGS := -D warnings
TARGET_DIR := target
COVERAGE_DIR := coverage

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Build the project in debug mode
	$(CARGO) build

release: ## Build the project in release mode
	$(CARGO) build --release

debug: build ## Alias for build (debug mode)

test: ## Run all tests
	$(CARGO) test --all-targets

test-watch: ## Run tests in watch mode (requires cargo-watch: cargo install cargo-watch)
	@command -v cargo-watch >/dev/null 2>&1 || { echo "Error: cargo-watch not found. Install with: cargo install cargo-watch"; exit 1; }
	cargo-watch -x test

test-verbose: ## Run tests with verbose output
	$(CARGO) test --all-targets -- --nocapture

lint: ## Run clippy linter
	$(CARGO) clippy --all-targets --all-features -- -D warnings

lint-fix: ## Run clippy and automatically fix issues
	$(CARGO) clippy --all-targets --all-features --fix --allow-dirty --allow-staged

fmt: ## Format code with rustfmt
	$(CARGO) fmt

fmt-check: ## Check code formatting without making changes
	$(CARGO) fmt --check

coverage: ## Generate test coverage report (requires cargo-tarpaulin)
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "Error: cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"; exit 1; }
	@mkdir -p $(COVERAGE_DIR)
	$(CARGO) tarpaulin --out Xml --output-dir $(COVERAGE_DIR) --exclude-files 'src/connectors.rs'
	@echo "Coverage report generated in $(COVERAGE_DIR)/cobertura.xml"

coverage-html: ## Generate HTML coverage report (requires cargo-tarpaulin)
	@command -v cargo-tarpaulin >/dev/null 2>&1 || { echo "Error: cargo-tarpaulin not found. Install with: cargo install cargo-tarpaulin"; exit 1; }
	@mkdir -p $(COVERAGE_DIR)
	$(CARGO) tarpaulin --out Html --output-dir $(COVERAGE_DIR) --exclude-files 'src/connectors.rs'
	@echo "HTML coverage report generated in $(COVERAGE_DIR)/tarpaulin-report.html"

check: fmt-check lint ## Check formatting and linting without making changes

all: fmt lint test ## Format, lint, and test

clean: ## Clean build artifacts
	$(CARGO) clean

install-tools: ## Install development tools (cargo-tarpaulin, cargo-watch)
	$(CARGO) install cargo-tarpaulin cargo-watch

install-targets: ## Install Rust targets for cross-compilation
	@echo "Installing Rust targets for cross-compilation..."
	rustup target add x86_64-unknown-linux-musl
	rustup target add aarch64-unknown-linux-gnu
	@echo "✅ Targets installed. Note: For cross-compilation from macOS, consider using 'cross' tool."
	@echo "   Install with: cargo install cross --git https://github.com/cross-rs/cross"

ci: fmt-check lint test ## Run all CI checks (format, lint, test)

