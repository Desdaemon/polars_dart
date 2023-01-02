build:
	cd polars-wrapper && cargo build

test: build
	dart test
