# Ny Hon UI Contract

Ny Hon treats the UI as a standalone harness: pages compose reusable primitives, while primitives own their markup and visual class boundary.

## Layers

- `src/app.rs`: application state and top-level theme orchestration.
- `src/ui/pages`: route-level composition only; no styling primitives should be defined here.
- `src/ui/components`: reusable visual contracts (`AppShell`, `Sidebar`, `Header`, `Card`, `LibraryCard`).
- `assets/app.css`: visual tokens, component styles, and desktop layout rules.

## Component contract

Each component must:

1. Own a stable root class prefixed with `ny-`.
2. Expose behavior through props and `EventHandler`s rather than reaching into application state.
3. Be renderable independently with deterministic props.
4. Avoid page-specific data fetching, persistence, or source logic.
5. Prefer semantic HTML and accessible labels.

## Standalone contract

A component is standalone when it can be moved into another page without importing page state or depending on sibling selectors. Shared styling may depend only on CSS tokens and the component's own class subtree.

## Visual contract

The first UI pass establishes:

- paired dark/light surfaces;
- restrained neutral palette with one accent;
- consistent radius, borders, spacing and motion tokens;
- sticky desktop navigation/header;
- cards as the primary information container;
- no content/source implementation in the UI layer.

## Future boundaries

The eventual reader, source browser, library manager, categories, tracker integrations, backup UI and settings must consume these same primitives rather than creating parallel card, shell or navigation systems.
