gen:
	cd polars-wrapper && just gen

build *args='':
	cargo build --manifest-path polars-wrapper/Cargo.toml {{args}}

test: build
	dart test -x bench
bench: (build "--release")
	dart test -t bench