#!/bin/bash
# Cross-compilation for Windows using Docker

echo "Building for Current OS (Should be MacOS)"
cargo build --release

echo "Building for Windows"
docker build -t pinniped-win . -f Dockerfile.windows
docker run --rm -v .:/app pinniped-win

echo "Building for Linux"
docker build -t pinniped-linux . -f Dockerfile.linux
docker run --rm -v .:/app pinniped-linux

echo "Combining"
rm -rf build
mkdir -p build

mkdir build/macos
cp target/release/pinniped build/macos/pinniped
cp -r examples build/macos/examples
cp README.md build/macos/README.md
cd build && zip -r macos.zip macos && cd ..
rm -rf build/macos

mkdir build/windows
cp target/x86_64-pc-windows-gnu/release/pinniped.exe build/windows/pinniped.exe
cp -r examples build/windows/examples
cp README.md build/windows/README.md
cd build && zip -r windows.zip windows && cd ..
rm -rf build/windows

mkdir build/linux
cp target/release/pinniped build/linux/pinniped
cp -r examples build/linux/examples
cp README.md build/linux/README.md
cd build && zip -r linux.zip linux && cd ..
rm -rf build/linux
