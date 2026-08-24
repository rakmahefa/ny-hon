# Ny Hon

Ny Hon is a Linux-first open-source reader built entirely with Rust and Dioxus. It is designed to discover and read manga, webtoons and comics from external sources; it does not host content.

## UI foundation

The current branch establishes only the barebone desktop GUI and its architecture harness:

- reusable `AppShell`, `Sidebar`, `Header`, `Card` and `LibraryCard` components;
- a page layer that only composes components;
- a token-driven dark/light visual contract;
- explicit standalone-component rules in `docs/UI_CONTRACT.md`;
- Linux CI with `cargo check`.

The future reader, source adapters, library persistence, update checks, categories, MAL/AniList tracking and local backups should stay outside the visual component layer.

## Development

Install the Dioxus CLI, then run:

```bash
dx serve --desktop
```

For a compile-only check:

```bash
cargo check --all-targets
```

Dioxus currently provides the desktop renderer used by this project; the branch intentionally keeps the initial dependency surface small. citeturn361749search0turn361749search3
