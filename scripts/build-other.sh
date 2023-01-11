#!/bin/bash

# Setup
BUILD_DIR=platform-build
mkdir $BUILD_DIR
cd $BUILD_DIR

# Install build dependencies
cargo install cargo-zigbuild
cargo install cargo-xwin

zig_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    rustup target add "$TARGET"
    cargo zigbuild --target "$TARGET" "$@"
    mkdir "$PLATFORM_NAME"
    cp "../target/$TARGET/release/$LIBNAME" "$PLATFORM_NAME/"
}

win_build () {
    local TARGET="$1"
    local PLATFORM_NAME="$2"
    local LIBNAME="$3"
    rustup target add "$TARGET"
    cargo xwin build --target "$TARGET" "$@"
    mkdir "$PLATFORM_NAME"
    cp "../target/$TARGET/release/$LIBNAME" "$PLATFORM_NAME/"
}

# Build all the dynamic libraries
LINUX_LIBNAME=libpolars_wrapper.so
zig_build aarch64-unknown-linux-gnu linux-arm64 $LINUX_LIBNAME
zig_build x86_64-unknown-linux-gnu linux-x64 $LINUX_LIBNAME
WINDOWS_LIBNAME=polars_wrapper.dll
win_build aarch64-pc-windows-msvc windows-arm64 $WINDOWS_LIBNAME
win_build x86_64-pc-windows-msvc windows-x64 $WINDOWS_LIBNAME

# Archive the dynamic libs
tar -czvf other.tar.gz linux-* windows-*

# Cleanup
rm -rf linux-* windows-*
