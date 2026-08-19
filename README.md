# Rust Service Template

A small, opinionated template for building Rust backend services with Axum.

The template follows a simple layered architecture:

```text
Router
  ↓
Handler
  ↓
Repository Trait
  ↓
Service
```

The goal is to keep services easy to understand, easy to test, and consistent across projects.

## Architecture

```text
src/
├── api/
│   ├── routes/
│   └── router.rs
├── config/
├── errors/
├── handlers/
├── models/
├── repositories/
├── services/
├── lib.rs
└── main.rs
```

### Router

Contains the HTTP API exposed by the service.

Responsibilities:

* Route definitions
* Request extraction
* Response serialization
* HTTP-specific concerns

Routes should remain thin and delegate application logic to handlers.

### Handlers

Coordinate application workflows and business logic.

A handler may call one or multiple repository contracts to fulfill a use case.

```text
Route
  ↓
Handler
  ├─ Repository A
  └─ Repository B
```

### Repositories

Repository traits define the contracts between handlers and underlying services.

Handlers depend on repository traits instead of concrete service implementations.

```rust
pub trait ExampleRepository: Send + Sync {
  async fn get_by_id(&self, id: u64) -> Result<Example, Error>;
}
```

This allows implementations to be replaced without changing the handler.

### Services

Services implement repository contracts and provide concrete capabilities.

Examples:

* Database access
* External APIs
* Package creation
* Publishing
* Storage
* Authentication

Services should focus on their specific responsibility and should not contain HTTP concerns.

### Models

Shared application structures live under `models/`.

This may include:

* Domain models
* Request models
* Response models

### Errors

Internal application errors are separated from public API errors.

```text
Internal Error
     ↓
API Error Code
     ↓
HTTP Status + JSON Response
```

This allows detailed internal errors to be logged while exposing stable and safe errors to API consumers.

## Generate a Project

Install `cargo-generate`:

```bash
cargo install cargo-generate --locked
```

Generate a project from the template:

```bash
cargo generate \
  --git https://github.com/pschugt/rust-service-template.git \
  --name my-service
```

For local development of the template:

```bash
cargo generate \
  --path ./rust-service-template \
  --name my-service
```

The generated Cargo package name uses the supplied project name while Rust crate identifiers are automatically converted to snake case.

For example:

```text
Project: my-awesome-service
Crate:   my_awesome_service
```

## Run the Service

```bash
cargo run
```

By default, the server listens on port `8080`.

Check the health endpoint:

```bash
curl http://localhost:8080/v1/status
```

## Development Checks

The template pins the Rust toolchain through `rust-toolchain.toml` and includes `rustfmt` and `clippy`.

Before committing changes, run:

```bash
cargo fmt --check
cargo clippy 
cargo nextest run
cargo deny check
cargo check
```

### Formatting

Format the project with:

```bash
cargo fmt
```

Verify formatting without modifying files:

```bash
cargo fmt --check
```

Formatting rules are defined in `rustfmt.toml`.

### Linting

Run Clippy for all targets and enabled features:

```bash
cargo clippy
```

Warnings are treated as errors to keep generated services clean.

### Testing

Tests are executed with `cargo-nextest`:

```bash
cargo nextest run
```

Install it with:

```bash
cargo install cargo-nextest --locked
```

### Dependency Checks

Dependency licenses, advisories, sources, and duplicate versions are checked with `cargo-deny`:

```bash
cargo deny check
```

Install it with:

```bash
cargo install cargo-deny --locked
```

Duplicate dependency versions are reported as warnings rather than errors, since different versions can legitimately be required by transitive dependencies.

### Toolchain

The Rust version and editor tooling are pinned through `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.97.1"
profile = "default"
components = ["rustfmt", "clippy", "rust-analyzer"]
```

This ensures the compiler, formatter, Clippy, and Rust Analyzer use the same Rust toolchain across development environments.


## Testing

Run tests with:

```bash
cargo nextest run
```

Run a compilation check with:

```bash
cargo check
```

## Configuration

Configuration is loaded from environment variables.

A local `.env` file may be used during development.

Example:

```env
PORT=8080
RUST_LOG=my_service=debug,tower_http=trace,axum::rejection=trace
```

## Design Principles

* Keep routes thin.
* Keep business workflows in handlers.
* Depend on repository traits instead of concrete services.
* Keep services focused on one capability.
* Keep HTTP concerns out of services.
* Keep internal errors separate from public API errors.
* Only introduce architectural layers when they provide a real responsibility.
* Prefer simple, explicit code over unnecessary abstractions.

## Example Request Flow

```text
GET /v1/examples/42
        ↓
Example Route
        ↓
ExampleHandler
        ↓
ExampleRepository
        ↓
ExampleService
        ↓
Example
        ↓
JSON Response
```

The included example feature serves as the reference implementation for adding new functionality.
