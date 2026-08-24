# Ny Hon — UI Specification

## 1. Purpose

Ny Hon is a Linux-first manga, webtoon and comics reader built with Rust + Dioxus.
The application discovers and reads content from external sources; it does not host content.

This specification defines the GUI foundation only. Domain, persistence, networking, source adapters and tracking are separate layers.

## 2. Reference

Mihon is the primary product/UX reference for navigation and reader capabilities.
Ny Hon must not copy Mihon implementation code; only product patterns and interaction concepts are used as reference.

## 3. UI architecture

```text
App
└── AppShell
    ├── Sidebar / Navigation
    ├── Header / Toolbar
    └── Page
        └── Reusable Components
```

Rules:

- Pages compose components; they do not redefine shared visual primitives.
- Components own a stable visual boundary and accept behavior through props/events.
- Components must be renderable independently with deterministic props.
- UI components must not access domain services, storage, network clients or source implementations.
- Shared styling comes from centralized design tokens and component-scoped classes.

## 4. Visual contract

- Desktop Linux first.
- Dark and light themes are mandatory.
- One shared token system for color, spacing, radius, borders, typography and motion.
- Covers, cards, toolbars, filters and states are reusable primitives.
- Loading, empty, error and offline states are first-class UI states.
- Reader UI is a dedicated fullscreen/immersive shell, separate from the application shell.

## 5. Main application surfaces

Primary navigation:

1. Library
2. Updates
3. History
4. Browse
5. More

Categories are a library-management feature, not a primary navigation destination.

Required future surfaces:

- Library: categories, search, filters, grid/list modes, selection mode, refresh and library states.
- Updates: new chapters, filtering, bulk actions, download/read actions and upcoming releases.
- History: recent reading, resume, search and history management.
- Browse: sources, extensions, global search and migration.
- Manga detail: metadata, cover, chapters, reading actions, download, categories and tracking.
- Reader: configurable reading modes, directions, navigation, page actions and reader settings.
- Downloads: queue, progress, pause/resume, retry and failure states.
- Categories: create, rename, delete, reorder and assign library items.
- Settings: appearance, library, reader, sources, tracking, data/storage and about.
- Tracking: AniList and MyAnimeList UI contracts.
- Backup/Restore: local library data export/import UI.

## 6. Component foundation

Core reusable primitives include:

`AppShell`, `Sidebar`, `Header`, `Card`, `Button`, `IconButton`, `Cover`, `Badge`, `Progress`, `SectionHeader`, `EmptyState`.

The component library should grow with reusable controls such as search fields, filter chips, tabs, segmented controls, dialogs, menus, banners, skeletons, switches, sliders and status indicators before implementing large feature-specific layouts.

## 7. Implementation order

1. Navigation Shell v2
2. Library v2
3. Updates
4. Manga Detail
5. Browse / Sources
6. History
7. Downloads
8. Settings
9. Reader Shell
10. Tracking and Backup/Restore

Each step must preserve the standalone component contract and keep CI green before moving to the next surface.

## 8. Explicit non-goals for the GUI layer

The GUI layer must not implement:

- source scraping or networking;
- chapter downloading;
- database persistence;
- reading progress persistence;
- tracker API calls;
- backup serialization;
- update scheduling.

Those responsibilities belong to domain/application services consumed by the UI through explicit state and event contracts.
