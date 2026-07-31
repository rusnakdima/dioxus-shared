# dioxus-shared

Pure Rust business logic library for Dioxus applications — no Tauri dependency.

## TypeScript Code Generation

This library uses `ts-rs` to generate TypeScript type definitions from Rust structs.

### Triggering TypeScript Generation

To generate TypeScript types, run:

```bash
cargo run -p dioxus-shared --release
```

Generated TypeScript files are output to the `gen/` directory.

### Adding New TS-Exported Types

When adding new structs or enums that need TypeScript generation:

1. Add `use ts_rs::TS;` to the imports
2. Add `TS` to the derive macro: `#[derive(Debug, Clone, Serialize, Deserialize, TS)]`
3. Add `#[ts(export)]` above the struct or enum definition (not individual fields — all fields are exported by default)
4. Run `cargo run -p dioxus-shared --release` to regenerate types

## Adding to Your Project

```toml
# Cargo.toml
[dependencies]
dioxus-shared = { path = "../dioxus-shared" }
```

Or from git:

```toml
[dependencies]
dioxus-shared = { git = "https://github.com/your-org/your-repo", branch = "main" }
```
