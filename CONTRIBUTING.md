# Contributing

Thanks for helping improve this native Rust implementation of the VS Code
workbench. Contributions to the desktop editor, the CLI and tunnel service,
documentation, tests, and platform packaging are welcome.

## Before you start

- Search [existing issues](https://github.com/PrathewshCysgen/vscode_rust/issues)
  and pull requests before opening a new one.
- Open an issue first for substantial features or architectural changes so the
  intended design can be discussed.
- Report security vulnerabilities privately; see [SECURITY.md](SECURITY.md).

## Development setup

Install the current stable Rust toolchain through [rustup](https://rustup.rs/).
The desktop application also requires the graphics support appropriate to your
platform: Metal on macOS, DirectX 12 or Vulkan on Windows, and Vulkan with a
Wayland or X11 environment on Linux.

Clone the repository and work from its root:

```bash
git clone https://github.com/Prathewsh/vscode_rust.git
cd vscode_rust
```

The native editor is the workspace package:

```bash
cargo run -p vscode-native
```

The CLI is its own Cargo package under `cli`:

```bash
cargo run --manifest-path cli/Cargo.toml -- --help
```

On Windows, building the CLI currently requires OpenSSL. Follow the setup notes
in [`cli/CONTRIBUTING.md`](cli/CONTRIBUTING.md) before building it.

## Project layout

- `crates/vscode-native/` — the GPU-accelerated desktop editor.
- `cli/` — the `code` command-line client, tunnel support, and integration
  tests.
- `resources/` — platform launchers, icons, installers, and server resources.

## Making a change

Keep pull requests focused and avoid unrelated formatting or generated-file
updates. Follow the surrounding Rust style; the repository uses tabs as defined
in `.editorconfig`. Add or update tests when behavior changes, especially for
buffer editing, filesystem handling, CLI arguments, RPC, and tunnel behavior.

Use clear commit and pull-request descriptions that explain the problem, the
solution, and how it was tested. For UI changes, include a screenshot or short
recording when useful. For bug reports, include a minimal reproduction, expected
behavior, actual behavior, platform, and relevant logs.

## Validate before opening a pull request

Run the checks that apply to the area you changed:

```bash
# Native editor workspace
cargo fmt --all --check
cargo check -p vscode-native
cargo test --workspace

# CLI package
cargo fmt --manifest-path cli/Cargo.toml -- --check
cargo check --manifest-path cli/Cargo.toml
cargo test --manifest-path cli/Cargo.toml
```

Also test the affected workflow manually. For desktop changes, launch
`vscode-native`; for CLI changes, exercise the relevant `code` subcommand.

## Pull requests

Please make sure your pull request:

- Has a single, reviewable purpose and links any related issue.
- Includes tests and documentation when they are needed.
- Passes the applicable validation commands above.
- Does not include secrets, credentials, private paths, or generated build
  output.

By submitting a contribution, you agree that it may be distributed under the
repository's [MIT License](LICENSE.txt).
