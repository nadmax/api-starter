# api-starter

This is a template repository to start writing an API in Rust

## Features

| Concern | Choice |
|---|---|
| Web framework | [Axum](https://docs.rs/axum) |
| Async runtime | [Tokio](https://tokio.rs) |
| Serialization | [Serde JSON](https://serde.rs) |
| Error handling | [thiserror](https://docs.rs/thiserror) |
| Validation | [validator](https://docs.rs/validator) |
| Logging | [tracing](https://docs.rs/tracing) + [tracing-subscriber](https://docs.rs/tracing-subscriber) |
| Config | [config](https://docs.rs/config) (TOML + env vars) |
| Request IDs | tower-http `SetRequestId` |
| CORS | tower-http `CorsLayer` |
| Timeout | tower-http `TimeoutLayer` |
| Tests | [axum-test](https://docs.rs/axum-test) |

## Project layout

```
.
├── config/
│   ├── default.toml      # base config, all environments
│   └── production.toml   # production overrides
├── src/
│   ├── main.rs           # entry point
│   ├── config.rs         # AppConfig (typed config loading)
│   ├── state.rs          # AppState (shared across handlers)
│   ├── errors.rs         # ApiError → HTTP response mapping
│   ├── middleware/
│   │   └── tracing.rs    # logging init
│   ├── models.rs        # domain types + request/response DTOs
│   └── routes/
│       ├── mod.rs        # router construction + middleware stack
│       ├── health.rs     # GET /health
│       └── items.rs      # CRUD /api/v1/items
└── tests/
    └── .gitkeep      # write your tests here
```

## Getting started

```sh
# Clone the template
git clone https://github.com/you/axum-template my-api
cd my-api

# Run in development
make dev

# Run tests
make test

# Build optimised release binary
make build
```

The server starts on [http://localhost:3000](http://localhost:3000) by default.

## Configuration

Override any value via environment variables prefixed with `APP__`:

```sh
APP__SERVER__PORT=8080 cargo run
```

Or add a `config/local.toml` (git-ignored):

```toml
[server]
port = 8080
```

Environment (`APP_ENV`) can be `development` (default), `production`, or any custom name — a matching `config/{APP_ENV}.toml` file will be loaded automatically.

## API reference

### Health

```
GET /health           → 200 OK
```

### Items

```
GET    /api/v1/items          → 200  { data, total, page, per_page }
POST   /api/v1/items          → 201  Item
GET    /api/v1/items/:id      → 200  Item | 404
PUT    /api/v1/items/:id      → 200  Item | 404
DELETE /api/v1/items/:id      → 204       | 404
```

### Error shape

```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "resource not found"
  }
}
```

Internal errors include a `request_id` for correlation with server logs.

## Docker

```sh
# Build Docker image
make docker-build

# Run Docker image
make docker-run
```

## Extending the template

1. **Add a database** – put an SQLx pool in `AppState` and swap the in-memory store in `routes/items.rs` for real queries.
2. **Authentication** – add a `middleware/auth.rs` extractor and protect routes with `.route_layer(...)`.
3. **New resource** – copy `routes/items.rs`, rename types, and `nest` the new router in `routes/mod.rs`.

