# Default recipe
default:
    just --list

# Create project structure (folders & files)
init:
    @echo "Creating directories..."
    # mkdir -p src/scheduler

    @echo "Creating files..."
    # touch src/main.rs

    # touch README.md
    # touch .gitignore

    @echo "Project structure created ✔"

# Add one or more dependencies to your Cargo.toml
add dependency:
    cargo add {{ dependency }}

# Install one or more binaries
install binary:
    cargo add {{ binary }}

# Run the application
run:
    cargo run

# COMMENTS for build-windows-msvc
# cargo install cargo-watch
# Other useful options
# cargo watch -x "run --release" - runs release builds
# cargo watch -x test - runs tests on save
# cargo watch -c -x run - clears the screen before each run
# cargo watch -x "run -- arg1 arg2" - passes arguments to your program

# Run the application in watch mode
run-watch:
    cargo watch -x run

# Build for current platform
build:
    cargo build

# Build for macOS (release)
build-macos:
    cargo build --release

# COMMENTS for build-windows-msvc
#  brew install mingw-w64
# rustup target add x86_64-pc-windows-msvc
# cargo install cargo-xwin --locked

# Build for Windows (x86_64-pc-windows-msvc)
build-windows-msvc:
    RUSTFLAGS="-C link-arg=/FORCE:MULTIPLE" cargo xwin build --release --target x86_64-pc-windows-msvc

# COMMENTS for build-windows-msvc
#  brew install mingw-w64
# rustup target add x86_64-pc-windows-gnu
# Add to Cargo.toml:
# [target.x86_64-pc-windows-gnu]
# rustflags = ["-C", "link-arg=-fuse-ld=lld"]
# linker = "x86_64-w64-mingw32-gcc"
# ar = "x86_64-w64-mingw32-ar"

# Build for Windows (x86_64-pc-windows-gnu)
build-windows-gnu:
    cargo build --release --target x86_64-pc-windows-gnu

# Run tests
test:
    cargo test

# COMMENTS for test-cov-tarpaulin
# cargo install cargo-tarpaulin
# will produce an HTML file like tarpaulin-report.html

# Run tests with coverage (tarpaulin)
test-cov-tarpaulin:
    cargo tarpaulin --out Html

# COMMENTS for test-cov-llvm
# cargo install cargo-llvm-cov
# builds your project, runs tests, collects coverage data, and outputs an HTML report in target/llvm-cov/html/index.html

# Run tests with coverage (cargo-llvm-cov)
test-cov-llvm:
    cargo llvm-cov --html

# Clean build artifacts
clean:
    cargo clean

# Format code
fmt:
    cargo fmt

# Check if code is formatted
fmt-check:
    cargo fmt -- --check

# Run linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# COMMENTS for package-macos
# cargo install cargo-bundle
# rustup target add x86_64-apple-darwin

# Generate application bundle (macos)
package-macos:
    cargo build --release --target x86_64-apple-darwin
    cargo bundle --release --target x86_64-apple-darwin --icon assets/icon.icns

# COMMENTS for package-windows-msvc
# cargo install cargo-bundle
# follow build-windows-msvc instructions

# Generate application bundle (windows-msvc)
package-windows-msvc:
    just build-windows-msvc
    cargo bundle --release --target x86_64-pc-windows-gnu --icon assets/icon.ico

# COMMENTS for package-windows-gnu
# cargo install cargo-bundle
# follow build-windows-gnu instructions

# Generate application bundle (windows-gnu)
package-windows-gnu:
    just build-windows-gnu
    cargo bundle --release --target x86_64-pc-windows-gnu --icon assets/icon.ico
