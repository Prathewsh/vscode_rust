# Visual Studio Code (Native Rust Edition)

A high-performance, native desktop implementation of the Visual Studio Code workbench written in Rust.

## Features

- **Native GPU Acceleration**: Rendered with `wgpu` and `eframe` across macOS (Metal), Windows (DirectX 12/Vulkan), and Linux (Vulkan/Wayland/X11).
- **Dark Theme**: Deep obsidian editor (`#121214`), dark zinc panels (`#18181b`), activity bar (`#141416`), and sky blue accents (`#38bdf8`).
- **Official Material & Seti Vector Icon System**:
  - Crisp vector file icons for Rust (cog), Markdown (`M↓`), Git (diamond), JSON (`{}`), TOML, Lock (padlock), TypeScript, JavaScript, Shell (`>_`), Web, Images, and tabbed folders.
  - Zero emoji dependencies, ensuring sharp rendering without tofu boxes (`□`).
- **Virtualized High-Performance Editor**:
  - Viewport virtualization using `show_rows` to restrict syntax highlighting and token rendering to visible lines.
  - **Minimap**: 72px minimap column on the right showing syntax-colored miniature lines and an interactive translucent viewport slider for click/drag scrolling.
  - Wide 14px high-contrast scrollbars and clean line-number gutter spacing.
  - Multi-level Undo & Redo (`Cmd/Ctrl+Z`, `Cmd/Ctrl+Shift+Z` / `Ctrl+Y`).
- **Code Search & Replace Panel**:
  - Live workspace search with regex (`.*`), whole word (`ab`), and case sensitivity (`Aa`) toggles.
  - Search & Replace with preserve case (`AB`) and Replace All across workspace files.
  - Expandable search details (`...`) for files to include and exclude filters.
  - Header action buttons: Refresh, Clear, New Search Editor, Collapse All, and Open in Editor.
- **Integrated Multi-Session Terminal (PTY)**:
  - Supports multiple concurrent platform shell sessions (`zsh`, `bash`, `PowerShell`) via `portable-pty`.
  - Right sidebar listing active terminal sessions (`[>_] zsh`) with active indicator and close buttons.
  - Panel header tabs: `Problems (0)`, `Output`, `Debug Console`, `Terminal`, `Ports`.
  - Live streaming output with ANSI escape filtering and inline interactive prompt (`o prathewsh@...`).
  - Panel size maximize toggle (`[□]`).
- **Command Palette & Quick Open**:
  - `Cmd/Ctrl+P`: Fast file search with cached workspace file index.
  - `Cmd/Ctrl+Shift+P`: Command palette for build commands, theme switching, and actions.
- **Git Integration**:
  - Tracks branch name, modified files, untracked files, and unstaged status.
  - `1K+` badge on Source Control activity bar icon.

## Architecture

- [`crates/vscode-native`](crates/vscode-native): Core native desktop application.
  - [`src/main.rs`](crates/vscode-native/src/main.rs): Workbench layout, event loop, CentralPanel editor with minimap, and sidebar routing.
  - [`src/buffer.rs`](crates/vscode-native/src/buffer.rs): Rope-backed text buffer with undo/redo stacks and cursor movement.
  - [`src/icons.rs`](crates/vscode-native/src/icons.rs): Vector graphic renderers for Material/Seti file icons, activity bar tabs, and workbench buttons.
  - [`src/palette.rs`](crates/vscode-native/src/palette.rs): Command palette, Quick Open file filtering, workspace Code Search engine, and Git state.
  - [`src/syntax.rs`](crates/vscode-native/src/syntax.rs): TextMate grammar tokenization and style mapping via `syntect`.
  - [`src/terminal.rs`](crates/vscode-native/src/terminal.rs): Pseudoterminal subsystem (`portable-pty`), `TerminalManager`, and multi-session handler.
  - [`src/explorer.rs`](crates/vscode-native/src/explorer.rs): File system explorer and directory tree.
  - [`src/theme.rs`](crates/vscode-native/src/theme.rs): Antigravity Dark, Dark+, and Monokai color presets with scrollbar styling.
- [`cli`](cli): Official VS Code tunnel and CLI service in Rust.

## Getting Started

### Prerequisites

- [Rust Toolchain](https://rustup.rs/) (2021 edition or newer)
- Native GPU drivers (Metal, DirectX 12, or Vulkan)

### Verification

```bash
cargo check -p vscode-native
```

### Running the Native Editor

```bash
cargo run -p vscode-native
```

### Release Build

```bash
cargo build --release -p vscode-native
```

The compiled binary will be placed at `target/release/code-native`.

## Keyboard Shortcuts

| Shortcut                            | Action                             |
| ----------------------------------- | ---------------------------------- |
| `Cmd/Ctrl + P`                      | Quick Open (Search Files)          |
| `Cmd/Ctrl + Shift + P`              | Command Palette                    |
| `Cmd/Ctrl + Shift + F`              | Code Search in Files               |
| `Cmd/Ctrl + \``                     | Toggle Integrated Terminal         |
| `Cmd/Ctrl + S`                      | Save File                          |
| `Cmd/Ctrl + N`                      | New File                           |
| `Cmd/Ctrl + F`                      | Find in Active Buffer              |
| `Cmd/Ctrl + Z`                      | Undo                               |
| `Cmd/Ctrl + Shift + Z` / `Ctrl + Y` | Redo                               |
| `Alt + C`                           | Toggle Match Case (Search)         |
| `Alt + W`                           | Toggle Whole Word (Search)         |
| `Alt + R`                           | Toggle Regular Expression (Search) |
| `Alt + P`                           | Toggle Preserve Case (Replace)     |
| `Arrow Keys`                        | Move cursor (Line & Column)        |
| `Tab`                               | Indent (4 spaces)                  |

## License

[MIT](LICENSE.txt)
