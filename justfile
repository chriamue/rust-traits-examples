# Justfile for Rust Traits Examples
# Run examples, tests, and other tasks

# Default recipe to display available commands
default:
    @just --list

# Run all examples
all: relay triathlon unified vehicle

# Build the project
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test

# Run tests with output
test-verbose:
    cargo test -- --nocapture

# Run tests with coverage
test-coverage:
    @echo "🧪 Running tests with coverage..."
    @echo "================================="
    cargo tarpaulin --out Html --out Xml --output-dir coverage/
    @echo ""
    @echo "✅ Coverage report generated!"
    @echo "📊 Open coverage/index.html to view detailed report"

# View coverage report in browser
view-coverage:
    @echo "📊 Opening coverage report..."
    @if [ -f coverage/index.html ]; then \
        if command -v xdg-open > /dev/null; then \
            xdg-open coverage/index.html; \
        elif command -v open > /dev/null; then \
            open coverage/index.html; \
        elif command -v start > /dev/null; then \
            start coverage/index.html; \
        else \
            echo "Please open coverage/index.html manually"; \
        fi \
    else \
        echo "❌ Coverage report not found. Run 'just test-coverage' first."; \
    fi

# Show coverage summary
coverage-summary:
    @echo "📊 Coverage Summary"
    @echo "==================="
    @if [ -f coverage/cobertura.xml ]; then \
        COVERAGE=$$(grep -oP 'line-rate="\K[0-9.]+' coverage/cobertura.xml | head -1 | awk '{printf "%.1f", $$1*100}'); \
        LINES_COVERED=$$(grep -oP 'lines-covered="\K[0-9]+' coverage/cobertura.xml | head -1); \
        LINES_VALID=$$(grep -oP 'lines-valid="\K[0-9]+' coverage/cobertura.xml | head -1); \
        echo "Coverage: $$COVERAGE%"; \
        echo "Lines: $$LINES_COVERED / $$LINES_VALID"; \
        if [ $$(echo "$$COVERAGE >= 80" | bc -l) -eq 1 ]; then \
            echo "🟢 Excellent coverage!"; \
        elif [ $$(echo "$$COVERAGE >= 60" | bc -l) -eq 1 ]; then \
            echo "🟡 Good coverage"; \
        else \
            echo "🔴 Coverage needs improvement"; \
        fi \
    else \
        echo "❌ Coverage data not found. Run 'just test-coverage' first."; \
    fi

# Run tests for a specific module
test-module MODULE:
    cargo test {{MODULE}}

# Check code without building
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Check formatting
fmt-check:
    cargo fmt -- --check

# Run clippy linter
lint:
    cargo clippy -- -D warnings

# Run clippy with all features
lint-all:
    cargo clippy --all-features -- -D warnings

# Clean build artifacts
clean:
    cargo clean

# ====================
# Example Runners
# ====================

# Run the Relay Competition example
relay:
    @echo "🏃‍♂️ Running Relay Competition..."
    @echo "================================"
    cargo run --example relay_competition

# Run the Triathlon Competition example
triathlon:
    @echo "🏊‍♂️ Running Triathlon Competition..."
    @echo "===================================="
    cargo run --example triathlon_competition

# Run the Unified Race Competition example
unified:
    @echo "🌐 Running Unified Race Competition..."
    @echo "======================================"
    cargo run --example unified_race_competition

# Run the Vehicle Race Competition example
vehicle:
    @echo "🏁 Running Vehicle Race Competition..."
    @echo "======================================"
    cargo run --example vehicle_race_competition

# Run a specific example by name
run-example EXAMPLE:
    cargo run --example {{EXAMPLE}}

# ====================
# Release Mode Examples
# ====================

# Run all examples in release mode
all-release: relay-release triathlon-release unified-release vehicle-release

# Run relay in release mode
relay-release:
    @echo "🏃‍♂️ Running Relay Competition (Release)..."
    @echo "==========================================="
    cargo run --release --example relay_competition

# Run triathlon in release mode
triathlon-release:
    @echo "🏊‍♂️ Running Triathlon Competition (Release)..."
    @echo "=============================================="
    cargo run --release --example triathlon_competition

# Run unified in release mode
unified-release:
    @echo "🌐 Running Unified Race Competition (Release)..."
    @echo "================================================"
    cargo run --release --example unified_race_competition

# Run vehicle in release mode
vehicle-release:
    @echo "🏁 Running Vehicle Race Competition (Release)..."
    @echo "================================================"
    cargo run --release --example vehicle_race_competition

# ====================
# Documentation
# ====================

# Generate and open documentation
doc:
    cargo doc --open
