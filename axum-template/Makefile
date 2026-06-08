.PHONY: help dev build test lint

## help: Show this help message
help:
	@printf "  \033[1;33m%-20s %s\033[0m\n" "Target" "Description"
	@printf "  \033[90m%-20s %s\033[0m\n"  "──────────────────" "───────────────────────────────────────────"
	@grep -E '^## ' Makefile | sed 's/^## //' | awk -F': ' \
		'{ printf "  \033[1;36m%-20s\033[0m \033[37m%s\033[0m\n", $$1, $$2 }'
	@printf "\n"

## dev: Run the app locally with cargo
dev:
	cargo run

## build: Build the release binary
build:
	cargo build --release --locked

## test: Run all tests
test:
	cargo test

## lint: Run Clippy
lint:
	cargo clippy -- -D warnings -W clippy::pedantic

## docker-build: Build Docker image
docker-build:
	docker build -t server .

## docker-run: Run a container from the built image
docker-run:
	docker run -p 3000:3000 server
