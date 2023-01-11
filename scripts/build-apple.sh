#!/bin/bash

set -euxo pipefail

# Setup
BUILD_DIR=platform-build
mkdir -p $BUILD_DIR
cd $BUILD_DIR

PROFILE="${1:-"debug"}"
case "$PROFILE" in
        "debug")   PROFILE_ARGS="--profile=dev" ;;
        "release") PROFILE_ARGS="--release" ;;
        *)         PROFILE_ARGS="--profile=$PROFILE" ;;
esac

# Build static libs
for TARGET in \
        aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim \
        x86_64-apple-darwin aarch64-apple-darwin
do
    rustup target add $TARGET
    cargo build --target=$TARGET $PROFILE_ARGS
done

# Create XCFramework zip
FRAMEWORK="Polars.xcframework"
LIBNAME=libpolars_wrapper.a
mkdir -p mac-lipo ios-sim-lipo
IOS_SIM_LIPO=ios-sim-lipo/$LIBNAME
MAC_LIPO=mac-lipo/$LIBNAME
lipo -create -output $IOS_SIM_LIPO \
        ../target/aarch64-apple-ios-sim/$PROFILE/$LIBNAME \
        ../target/x86_64-apple-ios/$PROFILE/$LIBNAME
lipo -create -output $MAC_LIPO \
        ../target/aarch64-apple-darwin/$PROFILE/$LIBNAME \
        ../target/x86_64-apple-darwin/$PROFILE/$LIBNAME
xcodebuild -create-xcframework \
        -library $IOS_SIM_LIPO \
        -library $MAC_LIPO \
        -library ../target/aarch64-apple-ios/$PROFILE/$LIBNAME \
        -output $FRAMEWORK
zip -f -r $FRAMEWORK.zip $FRAMEWORK

# Cleanup
rm -rf ios-sim-lipo mac-lipo $FRAMEWORK
