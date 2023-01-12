curr_version := "polars-v" + `awk '/^version: /{print $2}' packages/polars/pubspec.yaml`

# Generate bindings.
gen:
	cd polars-wrapper && just gen

# Builds the local library for testing.
build *args:
	cargo build --manifest-path polars-wrapper/Cargo.toml {{args}}

build-apple *args:
	dart scripts/build_apple.dart {{args}}

build-android profile='release':
	bash scripts/build-android.sh {{profile}}

build-other *args:
	dart scripts/build_other.dart {{args}}

# (melos)
test: test-dart test-flutter

# (melos)
test-dart: build
	melos run test-dart

# Softlinks library archives from platform-build to their expected locations.
link:
	-ln -s $(pwd)/platform-build/PolarsWrapper.xcframework.zip packages/flutter_polars/macos/Frameworks/{{curr_version}}.zip
	-ln -s $(pwd)/platform-build/PolarsWrapper.xcframework.zip packages/flutter_polars/ios/Frameworks/{{curr_version}}.zip
	-ln -s $(pwd)/platform-build/other.tar.gz packages/flutter_polars/linux/{{curr_version}}.tar.gz
	-ln -s $(pwd)/platform-build/other.tar.gz packages/flutter_polars/windows/{{curr_version}}.tar.gz

# (melos)
test-flutter: build-apple build-android build-other
	melos run test-flutter

# (melos)
bench: (build "--release")
	melos run bench

# (melos) use instead of flutter pub get
init:
	melos bootstrap

# (melos) generate docs
docs:
	melos run docs

# (melos)
clean:
	melos clean

# Open melos.yaml
melos:
	@$EDITOR melos.yaml