# Repository Guidelines

## Project Structure & Module Organization

This directory is the Rust backend workspace for `smit.codes`. The site binary lives in `apps/smit-backend/`, and reusable backend functionality lives in `crates/blogkit/`.

- `apps/smit-backend/src/main.rs` wires the Axum router, CORS, rate limits, content loading, and server startup for `smit.codes`.
- `crates/blogkit/src/content/` parses and loads Markdown-backed writing, project, and photo content.
- `crates/blogkit/src/analytics/` owns SQLite setup and page/post view handlers.
- `crates/blogkit/src/system/` exposes runtime system info and metrics.
- `crates/blogkit/src/utils/` contains shared helpers such as caching and read-time calculation.

Content is stored under `content/writing/`, `content/projects/`, and `content/photos/`. The analytics schema is embedded from `crates/blogkit/src/database/schema.sql`. Runtime SQLite files are created under `db/` and should not be treated as source assets.

## Build, Test, and Development Commands

- `cargo run -p smit-backend` starts the API locally, defaulting to `127.0.0.1:3001`.
- `SERVER_ADDR=0.0.0.0:3001 cargo run -p smit-backend` runs the server on an explicit bind address.
- `cargo build -p smit-backend` compiles the debug binary.
- `cargo build -p smit-backend --release --locked` matches the Docker build path and uses `Cargo.lock`.
- `cargo fmt --check` verifies Rust formatting.
- `cargo clippy --all-targets --all-features` checks common Rust issues.
- `cargo test` runs unit and integration tests when present.
- `docker build -f Dockerfile ..` builds from the parent directory because the Dockerfile copies `backend/...` paths.

## Coding Style & Naming Conventions

Use Rust 2021 idioms and standard `rustfmt` formatting. Keep modules small and grouped by feature, matching the existing `analytics`, `content`, `system`, and `utils` layout. Use `snake_case` for functions, files, variables, and route handlers; use `PascalCase` for structs and enums. Prefer explicit error context near startup boundaries and keep handler logic focused on request/response behavior.

## Testing Guidelines

Add tests close to the code they exercise using `#[cfg(test)]` modules, or create integration tests under `tests/` for API-level behavior. Name tests after the behavior under test, for example `parses_markdown_frontmatter` or `returns_cached_content_etag`. Run `cargo test` before submitting changes; add fixture Markdown content only when it clarifies the behavior being tested.

## Commit & Pull Request Guidelines

Recent history uses concise, imperative commits, with occasional Conventional Commit prefixes such as `feat:` and `fix(content):`. Follow that style: `feat: add photo feed endpoint`, `fix(content): resolve asset lookup`. Pull requests should include a short summary, commands run, linked issue when relevant, and sample API responses or screenshots for user-visible behavior changes.

## Security & Configuration Tips

Configuration is environment-driven. Important variables include `SERVER_ADDR`, `ANALYTICS_DB_URL`, `ANALYTICS_DB_MAX_CONNECTIONS`, `ENABLE_RATE_LIMIT`, and `RATE_LIMIT_TRUST_PROXY_HEADERS`. Do not commit generated `db/` files, secrets, or machine-specific environment files.
