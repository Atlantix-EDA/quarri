# Altera Quartus Prime Dark Mode — Linux

![Platform](https://img.shields.io/badge/platform-Linux-blue)
![Rust](https://img.shields.io/badge/rust-%E2%9C%93-orange?logo=rust)
![Qt](https://img.shields.io/badge/Qt_6.5-compatible-blue?logo=qt)
![Quartus](https://img.shields.io/badge/Quartus_Prime-25.x-blue)
![License](https://img.shields.io/badge/license-MIT-blue)

This repo allows running Altera Quartus on Linux with a dark theme,
providing a modern look while being easy on the eyes for Linux users.
There are some dark stylesheets for Windows, but those simply will
not work on Linux. 

So this is a dark mode for Altera Quartus Prime on Linux using [QDarkStyleSheet](https://github.com/ColinDuquesnoy/QDarkStyleSheet) with Quartus-specific patches.

Quartus's argument parser intercepts `-stylesheet` before Qt can process it on Linux. This project uses a small Rust `LD_PRELOAD` library to hook `QApplication::exec()` and inject the stylesheet directly via Qt's `setStyleSheet()` API.

Tested with Quartus Prime Pro 25.3.1 (Qt 6.5.7) on Linux Mint 22.

![alt text](assets/quartus_mint_demo2.gif)

## Requirements

- Altera Quartus Prime installed and run at least once
- Rust toolchain (`cargo`, `rustc`) — the inject library is compiled from source on first launch

### Installing Rust

If you don't have Rust installed, the recommended way is via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (the defaults are fine), then restart your shell or run `source ~/.cargo/env`.

## Installation

```bash
git clone https://github.com/saturn77/quartus-dark-linux.git
cd quartus-dark-linux

# Patch editor, RTL viewer, and pin planner colors (one-time, Quartus must be closed)
./install_linux.sh

# Launch Quartus with dark theme
./launch_quartus.sh
```

On first run, `launch_quartus.sh` will automatically build the Rust `LD_PRELOAD` library (`target/release/libqss_inject.so`). This only happens once — subsequent launches skip the build unless `src/lib.rs` has changed.

The launch script auto-detects your Quartus install path. To override:

```bash
QUARTUS_BIN=/path/to/quartus/bin/quartus ./launch_quartus.sh
```

## How it works

1. **`launch_quartus.sh`** resolves `:/dark_icons/` resource paths in the QSS to absolute filesystem paths, then launches Quartus with `LD_PRELOAD` set to the inject library.

2. **`libqss_inject.so`** (Rust) hooks `QApplication::exec()` via symbol interposition. Before the event loop starts, it resolves `QCoreApplication::self`, creates a `QString` from the QSS file contents via `QString::fromUtf8()`, and calls `QApplication::setStyleSheet()`.

3. **`install_linux.sh`** patches `~/.altera.quartus/quartus2.qreg` with dark colors for the Scintilla-based text editor, RTL viewer, and pin planner (these don't respond to QSS).

## Attribution

QSS based on [QDarkStyleSheet](https://github.com/ColinDuquesnoy/QDarkStyleSheet) — see `LICENSE.md`.

Windows version based on [Intel-Quartus-Dark-Mode-Windows](https://github.com/peter-tanner/Intel-Quartus-Dark-Mode-Windows).
