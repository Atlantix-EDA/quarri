#!/bin/bash
# Launch Quartus Prime with dark theme on Linux.
# Uses LD_PRELOAD to inject the QSS stylesheet via Qt's setStyleSheet API.
#
# Usage: ./launch_quartus.sh [quartus args...]
# Override Quartus path: QUARTUS_BIN=/path/to/quartus ./launch_quartus.sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
QSS_SOURCE="$SCRIPT_DIR/darkstyle.qss"
QSS_RESOLVED="/tmp/quartus_darkstyle_resolved.qss"
INJECT_LIB="$SCRIPT_DIR/target/release/libqss_inject.so"
# Fallback for workspace layout
[ ! -f "$INJECT_LIB" ] && INJECT_LIB="$SCRIPT_DIR/inject/target/release/libqss_inject.so"

# Auto-detect Quartus if not set
if [ -z "$QUARTUS_BIN" ]; then
    QUARTUS_BIN="$(which quartus 2>/dev/null)"
    if [ -z "$QUARTUS_BIN" ]; then
        for d in /opt/altera* /opt/intel* "$HOME"/altera* "$HOME"/intel*; do
            found="$(find "$d" -path "*/quartus/bin/quartus" -type f 2>/dev/null | sort -V | tail -1)"
            [ -n "$found" ] && QUARTUS_BIN="$found" && break
        done
    fi
fi

if [ ! -f "$QUARTUS_BIN" ]; then
    echo "Error: Cannot find Quartus. Set QUARTUS_BIN=/path/to/quartus/bin/quartus"
    exit 1
fi

# Build inject library if missing or outdated
if [ ! -f "$INJECT_LIB" ] || [ "$SCRIPT_DIR/inject/src/lib.rs" -nt "$INJECT_LIB" ]; then
    echo "Building qss_inject..."
    cargo build --release --manifest-path="$SCRIPT_DIR/Cargo.toml" -p qss_inject
    if [ $? -ne 0 ]; then
        echo "Error: Build failed (is cargo/rustc installed?)"
        exit 1
    fi
fi

# Resolve icon paths in QSS (:/dark_icons/ -> absolute filesystem paths)
sed "s|:/dark_icons/|$SCRIPT_DIR/dark_icons/|g" "$QSS_SOURCE" > "$QSS_RESOLVED"

export QUARTUS_QSS="$QSS_RESOLVED"
# Auto-detect QSYS font size based on monitor DPI (overridable via QSYS_FONTSIZE)
if [ -z "$QSYS_FONTSIZE" ]; then
    DPI=$(xdpyinfo 2>/dev/null | grep -oP 'resolution:\s+\K[0-9]+' | head -1)
    if [ -n "$DPI" ] && [ "$DPI" -ge 144 ]; then
        QSYS_FONTSIZE=24
    elif [ -n "$DPI" ] && [ "$DPI" -gt 96 ]; then
        QSYS_FONTSIZE=18
    else
        QSYS_FONTSIZE=12
    fi
fi
export QSYS_FONTSIZE
export LD_PRELOAD="${LD_PRELOAD:+$LD_PRELOAD:}$INJECT_LIB"
exec "$QUARTUS_BIN" "$@"
