gen:
	cd polars-wrapper && just gen

build *args:
	cargo build --manifest-path polars-wrapper/Cargo.toml {{args}}

build-apple *args:
	dart scripts/build_apple.dart {{args}}

build-android profile='release':
	bash scripts/build-android.sh {{profile}}

build-other profile='release':
	bash scripts/build-other.sh {{profile}}

# Requires melos.
test: test-dart test-flutter

# Requires melos.
test-dart: build
	melos run test-dart

link:
	-ln -s $(pwd)/platform-build/PolarsWrapper.xcframework.zip packages/flutter_polars/macos/Frameworks/polars-v0.1.0.zip
	-ln -s $(pwd)/platform-build/PolarsWrapper.xcframework.zip packages/flutter_polars/ios/Frameworks/polars-v0.1.0.zip

# Requires melos.
test-flutter: build-apple build-android build-other
	melos run test-flutter

# Requires melos.
bench: (build "--release")
	melos run bench

# Requires melos.
init:
	melos bootstrap

# Requires melos.
docs:
	melos run docs

# Requires melos.
clean:
	melos clean

melos:
	@$EDITOR melos.yaml