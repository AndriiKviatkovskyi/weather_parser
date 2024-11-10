# Makefile for weather_parser

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

build:
	cargo build	

test:
	cargo test

run:
	cargo run -- $(ARGS)

help:
	@echo "Commands:"
	@echo "  fmt            - Format code using cargo fmt"
	@echo "  clippy         - Check code against standards using cargo clippy"
	@echo "  build          - Build the project using cargo build"
	@echo "  test           - Run tests using cargo test"
	@echo "  run ARGS='...' - Run the program with arguments ARGS"
	@echo "  help           - Display this help message"
