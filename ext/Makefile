.PHONY: test

target/debug/libfun.so: rust-toolchain.toml Cargo.toml $(wildcard src/*.rs) $(wildcard src/**/*.rs)
	cargo build

run-tests.php:
	cp $$(php-config --lib-dir)/php/build/run-tests.php .

test: target/debug/libfun.so run-tests.php
	php run-tests.php -n -d extension=target/debug/libfun.so tests/phpt/
