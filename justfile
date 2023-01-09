# in-wrapper := ["--manifest-path" "polars-wrwapper/Cargo.toml"]
root := justfile_directory()
wrapper := root / "polars-wrapper/Cargo.toml"

gen:
	cd polars-wrapper && just gen

build *args='':
	cargo build --manifest-path {{wrapper}} {{args}}

test: build
	melos exec --no-flutter dart test -x bench
	melos exec --flutter --dir-exists=test flutter test

bench: (build "--release")
	melos exec dart test -t bench

docs:
	melos exec --no-private -- dart doc -o ../../website/\$MELOS_PACKAGE_NAME

clean:
	melos clean
	cargo clean --manifest-path {{wrapper}}

# test-web test='test/series_test.dart' *args='':
# 	sed 's/{{ "{{test-entry}}" }}/{{ file_name(test) }}.js/' web/index.html.tpl > web/index.html
# 	dart run flutter_rust_bridge:serve -r web -d {{test}} --crate polars-wrapper --run-tests {{args}}


# bench-web: (test-web 'test/benchmarks/main_test.dart' '--release')