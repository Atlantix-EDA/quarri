#!/bin/bash
# Install dark mode editor/RTL colors into Quartus settings on Linux.
# Run once per machine. Quartus must be closed.
#
# Quartus 25.3+ uses plain hex colors (#RRGGBB) instead of QVariant encoding.
# A Color_version entry is required to trigger custom color loading.

QREG="$HOME/.altera.quartus/quartus2.qreg"

if [ ! -f "$QREG" ]; then
    echo "Error: $QREG not found. Run Quartus at least once first."
    exit 1
fi

# Check if already patched (new format)
if grep -q "Color_version=" "$QREG"; then
    echo "Editor colors already configured in $QREG"
    exit 0
fi

cp "$QREG" "$QREG.bak"
echo "Backup saved to $QREG.bak"

# Find the [*_quartus] section header
SECTION=$(grep -n '^\[.*_quartus\]' "$QREG" | tail -1 | cut -d: -f1)
if [ -z "$SECTION" ]; then
    echo "Error: Cannot find quartus settings section in $QREG"
    exit 1
fi

# Remove any old-format @Variant color entries we may have previously inserted
sed -i '/^Altera_Foundation_Class\\AFCQ_.*=@Variant/d' "$QREG"
# Re-find section line after removals
SECTION=$(grep -n '^\[.*_quartus\]' "$QREG" | tail -1 | cut -d: -f1)

# Dark color palette (Quartus 25.3+ hex format)
# Colors inspired by QDarkStyle / Tokyo Night
read -r -d '' COLORS << 'COLORBLOCK'
Altera_Foundation_Class\Color_version=12
Altera_Foundation_Class\AFCQ_TED_BACKGROUND_COLOR=#19232d
Altera_Foundation_Class\AFCQ_TED_BACKGROUND_COLOR_DARK_MODE=#19232d
Altera_Foundation_Class\AFCQ_TED_NORMAL_COLOR=#dfe1e2
Altera_Foundation_Class\AFCQ_TED_NORMAL_COLOR_DARK_MODE=#dfe1e2
Altera_Foundation_Class\AFCQ_TED_KEYWORD_COLOR=#ffffff
Altera_Foundation_Class\AFCQ_TED_KEYWORD_COLOR_DARK_MODE=#ffffff
Altera_Foundation_Class\AFCQ_TED_LINE_NUMBER_COLOR=#dfe1e2
Altera_Foundation_Class\AFCQ_TED_LINE_NUMBER_COLOR_DARK_MODE=#dfe1e2
Altera_Foundation_Class\AFCQ_TED_LINE_BACKGROUND_COLOR=#19232d
Altera_Foundation_Class\AFCQ_TED_LINE_BACKGROUND_COLOR_DARK_MODE=#19232d
Altera_Foundation_Class\AFCQ_TED_SELECTION_FG_COLOR=#dfe1e2
Altera_Foundation_Class\AFCQ_TED_SELECTION_FG_COLOR_DARK_MODE=#dfe1e2
Altera_Foundation_Class\AFCQ_TED_SELECTION_HIGHLIGHT_COLOR=#2e3c64
Altera_Foundation_Class\AFCQ_TED_SELECTION_HIGHLIGHT_COLOR_DARK_MODE=#2e3c64
Altera_Foundation_Class\AFCQ_TED_ADDITIONAL_HIGHLIGHT_COLOR=#37414f
Altera_Foundation_Class\AFCQ_TED_ADDITIONAL_HIGHLIGHT_COLOR_DARK_MODE=#37414f
Altera_Foundation_Class\AFCQ_TED_SINGLE_COLOR=#00c000
Altera_Foundation_Class\AFCQ_TED_SINGLE_COLOR_DARK_MODE=#00c000
Altera_Foundation_Class\AFCQ_TED_MULTI_COLOR=#00c000
Altera_Foundation_Class\AFCQ_TED_MULTI_COLOR_DARK_MODE=#00c000
Altera_Foundation_Class\AFCQ_TED_STRING_COLOR=#e800e8
Altera_Foundation_Class\AFCQ_TED_STRING_COLOR_DARK_MODE=#e800e8
Altera_Foundation_Class\AFCQ_TED_IDENTIFIER_COLOR=#e800e8
Altera_Foundation_Class\AFCQ_TED_IDENTIFIER_COLOR_DARK_MODE=#e800e8
Altera_Foundation_Class\AFCQ_TED_BOOKMARK_COLOR=#00bfff
Altera_Foundation_Class\AFCQ_TED_BOOKMARK_COLOR_DARK_MODE=#00bfff
Altera_Foundation_Class\AFCQ_TED_NUMBER_COLOR=#ff6347
Altera_Foundation_Class\AFCQ_TED_NUMBER_COLOR_DARK_MODE=#ff6347
Altera_Foundation_Class\AFCQ_TED_VHDL_KEYWORDS_COLOR=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_VHDL_KEYWORDS_COLOR_DARK_MODE=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_VHDL_STD_KEYWORDS_COLOR=#ff6eb4
Altera_Foundation_Class\AFCQ_TED_VHDL_STD_KEYWORDS_COLOR_DARK_MODE=#ff6eb4
Altera_Foundation_Class\AFCQ_TED_VERILOG_KEYWORDS_COLOR=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_VERILOG_KEYWORDS_COLOR_DARK_MODE=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_TCL_KEYWORDS_COLOR=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_TCL_KEYWORDS_COLOR_DARK_MODE=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_TCL_MODIFIER_KEYWORDS_COLOR=#ff6eb4
Altera_Foundation_Class\AFCQ_TED_TCL_MODIFIER_KEYWORDS_COLOR_DARK_MODE=#ff6eb4
Altera_Foundation_Class\AFCQ_TED_TCL_VARIABLE_SUB_KEYWORDS_COLOR=#ff6eb4
Altera_Foundation_Class\AFCQ_TED_TCL_VARIABLE_SUB_KEYWORDS_COLOR_DARK_MODE=#ff6eb4
Altera_Foundation_Class\AFCQ_TED_SYS_VERILOG_KEYWORDS_COLOR=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_SYS_VERILOG_KEYWORDS_COLOR_DARK_MODE=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_AHDL_KEYWORDS_COLOR=#7aaaf7
Altera_Foundation_Class\AFCQ_TED_AHDL_KEYWORDS_COLOR_DARK_MODE=#7aaaf7
Altera_Foundation_Class\AFCQ_MSW_WARNING_COLOR=#e0af68
Altera_Foundation_Class\AFCQ_MSW_WARNING_COLOR_DARK_MODE=#e0af68
Altera_Foundation_Class\AFCQ_MSW_CRITICAL_WARNING_COLOR=#ff9e64
Altera_Foundation_Class\AFCQ_MSW_CRITICAL_WARNING_COLOR_DARK_MODE=#ff9e64
Altera_Foundation_Class\AFCQ_MSW_INFO_COLOR=#9ece6a
Altera_Foundation_Class\AFCQ_MSW_INFO_COLOR_DARK_MODE=#9ece6a
Altera_Foundation_Class\AFCQ_NUI_BACKGROUND_COLOR=#19232d
Altera_Foundation_Class\AFCQ_NUI_BACKGROUND_COLOR_DARK_MODE=#19232d
Altera_Foundation_Class\AFCQ_NUI_INSTANE_FONT_COLOR=#ff00ff
Altera_Foundation_Class\AFCQ_NUI_NET_COLOR=#ffff00
Altera_Foundation_Class\AFCQ_NUI_PIN_COLOR=#00ffff
Altera_Foundation_Class\AFCQ_NUI_PORT_COLOR=#00ffff
Altera_Foundation_Class\AFCQ_NUI_RIPPER_COLOR=#00ffff
Altera_Foundation_Class\AFCQ_NUI_PRIMITIVE_COLOR=#1a72bb
Altera_Foundation_Class\AFCQ_NUI_SELECTION_COLOR=#ff0000
Altera_Foundation_Class\AFCQ_NUI_INSTANCE_COLOR=#008000
Altera_Foundation_Class\AFCQ_NUI_INSTANCE_REGION_COLOR=#37414f
Altera_Foundation_Class\AFCQ_NUI_INSTANCE_ATOM_COLOR=#1a72bb
Altera_Foundation_Class\AFCQ_NUI_ENCRYPTED_INSTANCE_COLOR=#455364
Altera_Foundation_Class\AFCQ_PPLQ_BACKGROUND_COLOR=#54687a
COLORBLOCK

# Insert dark colors after the [*_quartus] section header
sed -i "${SECTION}r /dev/stdin" "$QREG" <<< "$COLORS"

echo "Dark editor, RTL viewer, and pin planner colors installed."
echo "Restart Quartus to see changes."
