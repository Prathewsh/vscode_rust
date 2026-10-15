# Visual Studio Code (Native Rust Edition)

A high-performance, native desktop implementation of the Visual Studio Code workbench written in Rust.

## Features

- **Native GPU Acceleration**: Rendered with `wgpu` and `eframe` across macOS (Metal), Windows (DirectX 12/Vulkan), and Linux (Vulkan/Wayland/X11).
- **Interactive Text Editing**:
  - Full typing, cursor navigation, indentation, and deletion.
  - Multi-level Undo & Redo (`Cmd/Ctrl+Z`, `Cmd/Ctrl+Shift+Z` / `Ctrl+Y`).
  - Active cursor rendered at character precision in the buffer.
  - File saving to disk (`Cmd/Ctrl+S`).
- **TextMate Syntax Highlighting**:
  - Token-level highlighting using `syntect` with VS Code Dark+ color palette.
  - Supports Rust, TOML, JSON, JavaScript, Markdown, HTML, C/C++, and other standard file extensions.
- **Integrated Terminal (PTY)**:
  - Spawns interactive platform shells (`/bin/zsh`, `bash`, or `PowerShell`) via `portable-pty`.
  - Live output streaming with ANSI escape sequence filtering.
  - Toggle terminal pane visibility via button or `Cmd/Ctrl+` ` ` shortcut.
- **VS Code Workbench Layout**:
  - **Activity Bar**: Navigation tabs for Explorer, Search, Source Control, Extensions, and Settings.
  - **Primary Sidebar**: Recursive filesystem explorer with directory browsing and file opening.
  - **Editor Tabs**: Multi-document tab strip tracking open buffers and unsaved changes (`●`).
  - **Status Bar**: Git branch indicator, document encoding, language mode, and live line/column counter.

## Architecture

- [`crates/vscode-native`](crates/vscode-native): Core native desktop application.
  - [`src/main.rs`](crates/vscode-native/src/main.rs): Workbench layout, event loop, and keyboard shortcut handler.
  - [`src/buffer.rs`](crates/vscode-native/src/buffer.rs): Rope-backed text buffer with undo/redo stacks and cursor movement.
  - [`src/syntax.rs`](crates/vscode-native/src/syntax.rs): TextMate grammar tokenization and style mapping.
  - [`src/terminal.rs`](crates/vscode-native/src/terminal.rs): Pseudoterminal subsystem and output stream.
  - [`src/explorer.rs`](crates/vscode-native/src/explorer.rs): File system explorer and directory tree.
  - [`src/theme.rs`](crates/vscode-native/src/theme.rs): VS Code Default Dark+ colors and widget styles.
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

| Shortcut | Action |
| --- | --- |
| `Cmd/Ctrl + S` | Save file |
| `Cmd/Ctrl + Z` | Undo |
| `Cmd/Ctrl + Shift + Z` / `Ctrl + Y` | Redo |
| `Cmd/Ctrl + ` ` ` | Toggle integrated terminal |
| `Arrow Keys` | Move cursor (Line & Column) |
| `Tab` | Indent (4 spaces) |

## License

[MIT](LICENSE.txt)
