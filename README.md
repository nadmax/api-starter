# rs-starter

A curated collection of production-ready Rust project templates.  
Each template is self-contained, opinionated, and ready to clone.

## Templates

### Web & API

| Template | Description |
|---|---|
| [`axum-template`](./axum-template) | REST API with full CRUD, error handling, validation, and structured logging  |
### Coming Soon

| Template | Description |
|---|---|
| `cli-template` | Command-line tool with subcommands, config file support, and rich terminal output |
| `worker-template` | Background job processor with graceful shutdown and retry logic |

## Usage

Each template directory is independently usable. Copy the one you need:

```sh
# Clone the full repository
git clone https://github.com/your-org/rs-starter

# Or use cargo-generate to scaffold directly from a template
cargo generate --git https://github.com/your-org/rs-starter --name my-project
```

Then follow the `README.md` inside the template directory for setup instructions.

## Design Principles

Templates in this repository follow a consistent set of standards:

- **Idiomatic Rust** — borrowing over cloning, `Result`-based error handling, no `unwrap()` in production paths
- **Layered configuration** — TOML files overridden by environment variables; no hardcoded values
- **Structured logging** — `tracing` throughout; JSON output in release builds
- **Tested** — integration tests included and passing before merge
- **Docker-ready** — multi-stage `Dockerfile` producing a lean runtime image
- **Lint-clean** — `clippy::pedantic` enabled; `unsafe_code` forbidden


## Contributing

Found a pattern worth generalising? Want to add a new template?

1. Create a directory named after the template (e.g. `grpc-service`)
2. Include a standalone `README.md`, working tests, and a `Dockerfile`
3. Add a row to the table above
4. Open a pull request with a brief description of the use case

All templates must pass `cargo clippy --all-targets -- -D warnings` and `cargo test` before merge.

## License

This project is licensed under the MIT License.