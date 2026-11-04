[private]
default: menu

[private]
menu:
    @bash scripts/menu.sh

# Build debug binaries.
build:
    cargo build

# Build optimized binaries.
build-release:
    cargo build --release

# Run the debug binary.
run:
    cargo run

# Run the optimized binary.
run-release:
    cargo run --release

# Run the test suite, including doc tests.
test:
    cargo test

# Run tests with nextest.
nextest:
    cargo nextest run --all-targets --all-features --no-tests=pass

# Check compilation.
check:
    cargo check

# Lint all targets and features.
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Format Rust sources.
fmt:
    cargo fmt --all

# Check Rust formatting.
fmt-check:
    cargo fmt --all --check

# Build API documentation.
docs:
    cargo doc --no-deps

# Check formatting, lint, and test.
verify: fmt-check lint test

# Build optimized binaries with profiling symbols.
profile-build:
    cargo build --profile profiling

# Debug the binary with GDB.
gdb: build
    rust-gdb --args target/debug/netblk

# Record CPU samples using a software event supported by VMs.
perf: profile-build
    perf record -e cpu-clock -F 99 --call-graph dwarf -o target/perf.data -- target/profiling/netblk

# Record heap allocations.
heaptrack: profile-build
    heaptrack -o target/heaptrack.netblk target/profiling/netblk
