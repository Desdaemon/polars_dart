gen:
	cd polars-wrapper && just gen

build *args='':
	cargo build --manifest-path polars-wrapper/Cargo.toml {{args}}

test: build
	dart test -x bench

bench: (build "--release")
	dart test -t bench

# test-web test='test/series_test.dart' *args='':
# 	sed 's/{{ "{{test-entry}}" }}/{{ file_name(test) }}.js/' web/index.html.tpl > web/index.html
# 	dart run flutter_rust_bridge:serve -r web -d {{test}} --crate polars-wrapper --run-tests {{args}}


# bench-web: (test-web 'test/benchmarks/main_test.dart' '--release')