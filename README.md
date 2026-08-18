# dioxus-shared

Pure Rust business logic library for Dioxus applications — no Tauri dependency.

## Start Here

**New to this library?** Read the [docs/INDEX.md](docs/INDEX.md) for a map of all documentation.

**Quick start** (desktop app with full features):

```toml
[dependencies]
dioxus-shared = { path = "../dioxus-shared", features = ["dioxus-ui"] }
```

**Quick start** (web/WASM app, no database or network):

```toml
[dependencies]
dioxus-shared = { path = "../dioxus-shared", features = ["web-ui"] }
```

---

## Feature Flags

| Feature | Enables | Use Case |
|---------|---------|----------|
| `dioxus-ui` | Full UI + database + network + desktop features | Desktop Dioxus apps (GhostGuardian, LumenCast, TaskFlow, Designer, ZenithDB, PacMan3D, UniChat) |
| `web-ui` | UI components only, no database/network | Web/WASM Dioxus apps |
| `dioxus-desktop` | Desktop-specific network (WebSocket) | When used alongside `dioxus` from another crate |
| `dotenvy` | Environment variable loading from `.env` | Development configuration |

Default features: `dotenvy`, `dioxus-ui`

---

## Terminology

| Term | Meaning |
|------|---------|
| **Foundation API** | APIs in `dioxus-shared/src/` available to any Dioxus app — algorithms, CRUD, schema, RBAC, storage, themes, error handling, logging |
| **Application-specific export** | APIs in `dioxus-shared/src/unichat.rs` — UniChat KAS handlers, chat entities, platform integrations (Twitch IRC, Kick, YouTube), overlay system |
| **SDUI** | Schema-Driven UI — a page is defined by a JSON schema, rendered by `DynamicPage` / `DynamicRenderer` |
| **KAS** | Kernel Algorithm System — command/query dispatch pattern via `KasCommand` |
| **SignalStore** | Reactive state container built on Dioxus signals |
| **Feature flag** | Cargo feature in `Cargo.toml` `[features]` section controlling what code compiles |

---

## Documentation Map

All library documentation lives under [`docs/`](docs/).

| Document | What It Covers |
|----------|---------------|
| [INDEX.md](docs/INDEX.md) | **Start here** — complete documentation index |
| [TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) | Common mistakes, anti-patterns, LLM navigation rules, decision tree |
| [LOGIC_INVENTORY.md](docs/LOGIC_INVENTORY.md) | Which logic is shared (foundation) vs app-specific; migration status |
| [UI_COMPONENTS.md](docs/UI_COMPONENTS.md) | All 25+ SDUI components, semantic variants, missing components by priority |
| [THEME_AUDIT.md](docs/THEME_AUDIT.md) | Theme system audit: tokens, CSS var bridging, dark mode, fixes applied |
| [PROJECT_BOOTSTRAP.md](docs/PROJECT_BOOTSTRAP.md) | How to create a new Dioxus project that uses dioxus-shared: feature selection, DDD layout, SDUI vs RSX, theme embedding |
| [COMPATIBILITY_MATRIX.md](docs/COMPATIBILITY_MATRIX.md) | Valid feature flags, consumer app configurations, Dioxus versions, stale feature names to avoid |
| [TEMPLATE_MAINTENANCE.md](docs/TEMPLATE_MAINTENANCE.md) | How to keep sdui-dioxus-template synchronized with dioxus-shared; known drift and drift-detection procedure |
| [TESTING_AND_CI.md](docs/TESTING_AND_CI.md) | Local verification commands, failure triage, CI gaps and recommended job boundaries |

---

## Foundation APIs (Available to All Apps)

These modules are the stable, app-agnostic core:

```rust
// From lib.rs re-exports
use dioxus_shared::algorithms::{algo_execute, Algorithm, AlgorithmRegistry, AlgorithmInput, AlgorithmOutput};
use dioxus_shared::crud::{CrudFilter, CrudQuery, CrudResult, CrudService, PaginatedResult};
use dioxus_shared::storage::{SignalStore, JsonProvider, SchemaConfig, SchemaSystem, setup_schema_system};
use dioxus_shared::schema::{AppConfig, Page, Component, Layout, UiSchema, Schema, Shortcut, Modal};
use dioxus_shared::rbac::{Role, Permission, login, logout, register, get_current_user};
use dioxus_shared::error::AppError;
use dioxus_shared::response::{Response, Status};
use dioxus_shared::logger::{Logger, LogLevel, LogEntry};
use dioxus_shared::env::EnvConfig;
use dioxus_shared::get_theme_css; // Embeds Tailwind v4 theme CSS
```

Core modules (also available directly):

```rust
pub mod algorithms; // Sorting, search, graph, sanitization
pub mod crud;       // CRUD service + KAS command system
pub mod storage;    // SignalStore, JsonProvider, schema sync
pub mod schema;     // SDUI schema types
pub mod rbac;       // Authentication + role-based access control
pub mod themes;     // MD3-based theme system (12 variants)
pub mod ui_engine;  // SDUI rendering engine (requires dioxus-ui)
pub mod ui;         // UI components (requires dioxus-ui)
pub mod shortcuts;   // Global keyboard shortcuts
pub mod mcp;        // MCP bridge protocol types
pub mod entities;   // Generic entity traits
pub mod error;      // AppError type
pub mod response;   // Response<T> wrapper
pub mod result;     // Result alias
pub mod logger;     // Structured logging
pub mod env;        // Environment config
pub mod update;     // Update checking
```

---

## Application-Specific Exports (UniChat)

These types and handlers are **specific to the UniChat application** and should not be used by other apps:

```rust
// From lib.rs UniChat exports
pub use unichat::{
    ChatMessage, ChatMessageCreate, ChatChannel, ChatChannelCreate,
    ChatAccount, ChatAccountCreate, CustomEmote, CustomEmoteCreate,
    DashboardPreferences, DashboardPreferencesUpdate, Platform,
    Badge, Emote, IrcMessage, AuthStatus, StorageEntry,
    OverlayConfig, OverlaySource, OverlayMessage, UpdateInfo, VersionInfo,
    TwitchIcon, KickChatroomInfo, KickUserInfo, YouTubeChannelInfo,
    // ... all create/get/update/patch/delete handlers
    // ... all Twitch IRC, Kick, YouTube fetch handlers
    // ... overlay server, storage, update handlers
};
```

Other apps should not depend on `unichat` module behavior.

---

## Maintenance Rules

**Source of truth order** (highest to lowest):

1. `src/lib.rs` — what is actually exported and how
2. `Cargo.toml` `[features]` section — what actually compiles
3. Prose documentation — may explain but never overrides code

If prose contradicts `lib.rs` or `Cargo.toml`, the code wins. Open an issue when you find a contradiction.

**Feature flag changes** must update `Cargo.toml` first, then this README.

**Do not copy** `sdui-dioxus-template` patterns into documentation. That template is not part of this repository.

---

## TypeScript Code Generation

This library uses `ts-rs` to generate TypeScript type definitions from Rust structs.

### Triggering TypeScript Generation

```bash
cargo run -p dioxus-shared --release
```

Generated TypeScript files are output to the `gen/` directory.

### Adding New TS-Exported Types

1. Add `use ts_rs::TS;` to the imports
2. Add `TS` to the derive macro: `#[derive(Debug, Clone, Serialize, Deserialize, TS)]`
3. Add `#[ts(export)]` above the struct or enum definition
4. Run `cargo run -p dioxus-shared --release` to regenerate types

## Adding to Your Project

```toml
# Cargo.toml — desktop app
[dependencies]
dioxus-shared = { path = "../dioxus-shared", features = ["dioxus-ui"] }
```

```toml
# Cargo.toml — web app
[dependencies]
dioxus-shared = { path = "../dioxus-shared", features = ["web-ui"] }
```

Or from git:

```toml
[dependencies]
dioxus-shared = { git = "https://github.com/your-org/your-repo", branch = "main" }
```
