# QuarRI™ — Quartus Dark Mode for Linux

![Platform](https://img.shields.io/badge/platform-Linux-blue)
![Rust](https://img.shields.io/badge/rust-%E2%9C%93-orange?logo=rust)
![Qt](https://img.shields.io/badge/Qt_6.5-compatible-blue?logo=qt)
![Quartus](https://img.shields.io/badge/Quartus_Prime-25.x-blue)
![License](https://img.shields.io/badge/license-MPL--2.0-blue)

**QuarRI™** (QUARtus launcheR guI — pronounced "quarry") is a dark-themed launcher and configuration tool for Intel Quartus Prime on Linux. It provides:

- **Dark QSS Theme** — `LD_PRELOAD` injection for full Quartus UI dark mode
- **Editor Colors** — QScintilla text editor, RTL viewer, and Pin Planner dark themes via qreg
- **Multi-Install Support** — auto-detects Pro, Standard, and Lite editions
- **Tokyo Night Storm UI** — built with [egui_mobius](https://github.com/saturn77/egui_mobius) + [egui_dock](https://docs.rs/egui_dock)

Tested with Quartus Prime Pro 25.3.1 (Qt 6.5.7) on **Linux Mint 22.3 Cinnamon.**

![alt text](assets/quartus_mint_demo2.gif)

## Requirements

- Altera Quartus Prime installed and run at least once
- Rust toolchain (`cargo`, `rustc`)

### Installing Rust

If you don't have Rust installed, the recommended way is via [rustup](https://rustup.rs):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts (the defaults are fine), then restart your shell or run `source ~/.cargo/env`.

## Installation

### QuarRI™ Launcher (recommended)

```bash
git clone https://github.com/saturn77/quartus-dark-linux.git
cd quartus-dark-linux
cargo install --path .
```

This installs the `quarri` binary to `~/.cargo/bin/`. Run it from anywhere:

```bash
quarri
```

The launcher auto-detects Quartus installations, lets you configure dark theme options, and launches Quartus with the correct environment.

### Manual (shell scripts)

```bash
git clone https://github.com/saturn77/quartus-dark-linux.git
cd quartus-dark-linux

# Patch editor, RTL viewer, and pin planner colors (one-time, Quartus must be closed)
./install_linux.sh

# Launch Quartus with dark theme
./launch_quartus.sh
```

On first run, `launch_quartus.sh` will automatically build the Rust `LD_PRELOAD` library (`inject/target/release/libqss_inject.so`). Subsequent launches skip the build unless source has changed.

### Building the inject library separately

```bash
cargo build --release -p qss_inject
```

## How it works

1. **`quarri` launcher** (or `launch_quartus.sh`) resolves `:/dark_icons/` resource paths in the QSS to absolute filesystem paths, sets `LD_PRELOAD` to the inject library, and spawns Quartus.

2. **`libqss_inject.so`** (Rust cdylib) hooks `QApplication::exec()` via symbol interposition. Before the event loop starts, it resolves Qt 6.5 symbols (`QCoreApplication::self`, `QString::fromUtf8`, `QApplication::setStyleSheet`) and injects the dark stylesheet.

3. **`install_linux.sh`** patches `~/.altera.quartus/quartus2.qreg` with dark colors for the QScintilla-based text editor, RTL viewer, messages window, and Pin Planner. Quartus 25.3+ uses plain `#RRGGBB` hex format with `Color_version=12` and `_DARK_MODE` variants (older `@Variant` binary format is no longer supported).

## Project structure

```
quartus-dark-linux/
├── launcher/          # QuarRI™ egui launcher (quarri binary)
│   └── src/
│       ├── main.rs    # Entry point, splash screen, dock layout
│       ├── state.rs   # Reactive state (egui_mobius_reactive)
│       ├── scanner.rs # Auto-detect Quartus installations
│       ├── launch.rs  # Spawn Quartus with LD_PRELOAD
│       ├── theme.rs   # Tokyo Night Storm palette
│       └── ui/        # Panels: installs, settings, log, splash, top_bar
├── inject/            # LD_PRELOAD cdylib (qss_inject)
│   └── src/lib.rs
├── assets/            # QSS stylesheet and dark icons
├── install_linux.sh   # One-time qreg color patch
└── launch_quartus.sh  # Manual launch script
```

## Attribution

QSS stylesheet derived from [QDarkStyleSheet](https://github.com/ColinDuquesnoy/QDarkStyleSheet) (MIT licensed).

Windows approach inspired by [Intel-Quartus-Dark-Mode-Windows](https://github.com/peter-tanner/Intel-Quartus-Dark-Mode-Windows).

## License

[Mozilla Public License 2.0](LICENSE)
