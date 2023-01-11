#!/bin/bash

CURR_VERSION=polars-v`awk '/^version: /{print $2}' packages/polars/pubspec.yaml`

# iOS & macOS
APPLE_HEADER="release_tag_name = '$CURR_VERSION' # generated; do not edit"
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_polars/ios/flutter_polars.podspec
sed -i.bak "1 s/.*/$APPLE_HEADER/" packages/flutter_polars/macos/flutter_polars.podspec
rm packages/flutter_polars/macos/*.bak packages/flutter_polars/ios/*.bak

# CMake platforms (Linux, Windows, and Android)
CMAKE_HEADER="set(LibraryVersion \"$CURR_VERSION\") # generated; do not edit"
for CMAKE_PLATFORM in android linux windows
do
    sed -i.bak "1 s/.*/$CMAKE_HEADER/" packages/flutter_polars/$CMAKE_PLATFORM/CMakeLists.txt
    rm packages/flutter_polars/$CMAKE_PLATFORM/*.bak
done

git add packages/flutter_polars/