curr_version := "polars-v" + `awk '/^version: /{print $2}' packages/polars/pubspec.yaml`
frb_bin := "cargo run --manifest-path ~/flutter_rust_bridge/frb_codegen/Cargo.toml -- generate"

export CARGO_TERM_COLOR := "always"

# generate bindings
gen:
	{{frb_bin}}

# builds the local library for testing
build *args:
	cargo build --manifest-path polars-wrapper/Cargo.toml {{args}}

build-apple *args:
	dart scripts/build_apple.dart {{args}}

build-android profile='release':
	bash scripts/build-android.sh {{profile}}

build-other *args:
	dart scripts/build_other.dart {{args}}

# (melos)
test: test-dart # test-flutter

# (melos)
test-dart: build
	melos run test-dart

# softlinks library archives from platform-build to their expected locations
link:
	-ln -sf $(pwd)/platform-build/PolarsWrapper.xcframework.zip packages/flutter_polars/macos/Frameworks/{{curr_version}}.zip
	-ln -sf $(pwd)/platform-build/PolarsWrapper.xcframework.zip packages/flutter_polars/ios/Frameworks/{{curr_version}}.zip
	-ln -sf $(pwd)/platform-build/other.tar.gz packages/flutter_polars/linux/{{curr_version}}.tar.gz
	-ln -sf $(pwd)/platform-build/other.tar.gz packages/flutter_polars/windows/{{curr_version}}.tar.gz
	-ln -sf $(pwd)/platform-build/android.tar.gz packages/flutter_polars/android/{{curr_version}}.tar.gz

# (melos)
test-flutter: build-apple build-android build-other
	melos run test-flutter

# (melos)
bench: (build "--release")
	melos run bench

# (melos) use instead of flutter pub get
init *args:
	melos bootstrap {{args}}

# (melos) generate docs
docs:
	melos run docs

# (melos)
clean:
	melos clean

check:
	flutter analyze

# Open melos.yaml
melos:
	@$EDITOR melos.yaml
