#!/usr/bin/env bash

# Exit immediately if a command exits with a non-zero status
set -e

echo "=== Genesis Cross-Architecture Build Script ==="

# Get version and name from Cargo.toml
VERSION=$(grep -m 1 '^version =' Cargo.toml | cut -d '"' -f 2)
NAME=$(grep -m 1 '^name =' Cargo.toml | cut -d '"' -f 2)

if [ -z "$NAME" ] || [ -z "$VERSION" ]; then
  echo "Error: Could not read name or version from Cargo.toml"
  exit 1
fi

echo "Project Name: $NAME"
echo "Project Version: $VERSION"

# 1. Setup rustup targets
echo "Adding Rustup targets..."
rustup target add x86_64-unknown-linux-musl \
  aarch64-unknown-linux-musl \
  aarch64-unknown-linux-gnu \
  x86_64-pc-windows-gnu

# 2. Check and install system packages if necessary
if ! command -v aarch64-linux-gnu-gcc &> /dev/null; then
  echo "gcc-aarch64-linux-gnu is not installed. Installing via sudo apt-get..."
  sudo apt-get update && sudo apt-get install -y gcc-aarch64-linux-gnu
fi

if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
  echo "gcc-mingw-w64-x86-64 is not installed. Installing via sudo apt-get..."
  sudo apt-get update && sudo apt-get install -y gcc-mingw-w64-x86-64
fi

# 3. Compile binaries
echo "Compiling targets..."

# A. Linux x86_64 Dynamic (glibc)
echo "Building x86_64-unknown-linux-gnu..."
cargo build --release

# B. Linux x86_64 Static (musl)
echo "Building x86_64-unknown-linux-musl..."
cargo build --release --target x86_64-unknown-linux-musl

# C. Linux ARM64 Static (musl)
echo "Building aarch64-unknown-linux-musl..."
RUSTFLAGS="-C linker-flavor=ld.lld -C linker=rust-lld" \
  cargo build --release --target aarch64-unknown-linux-musl

# D. Linux ARM64 Dynamic (glibc)
echo "Building aarch64-unknown-linux-gnu..."
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
  cargo build --release --target aarch64-unknown-linux-gnu

# E. Windows x86_64 (mingw)
echo "Building x86_64-pc-windows-gnu..."
CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \
  cargo build --release --target x86_64-pc-windows-gnu

# 4. Gather binaries and clean intermediate folders
echo "Gathering release binaries and cleaning build folders..."
TEMP_DIR=$(mktemp -d -t genesis-release-XXXXXX)

# Copy and rename according to format: name-vVERSION-arch
cp target/release/"${NAME}" "${TEMP_DIR}/${NAME}-v${VERSION}-x86_64-gnu"
cp target/x86_64-unknown-linux-musl/release/"${NAME}" "${TEMP_DIR}/${NAME}-v${VERSION}-x86_64-musl"
cp target/aarch64-unknown-linux-musl/release/"${NAME}" "${TEMP_DIR}/${NAME}-v${VERSION}-aarch64-musl"
cp target/aarch64-unknown-linux-gnu/release/"${NAME}" "${TEMP_DIR}/${NAME}-v${VERSION}-aarch64-gnu"
cp target/x86_64-pc-windows-gnu/release/"${NAME}".exe "${TEMP_DIR}/${NAME}-v${VERSION}-x86_64-windows.exe"

# Clean build folders
rm -rf target/

# Recreate clean target/release directory and move binaries back
mkdir -p target/release
mv "${TEMP_DIR}"/* target/release/
rm -rf "${TEMP_DIR}"

echo "=== Build Completed Successfully ==="
ls -lh target/release/
