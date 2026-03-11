//! Reactive application state using `Dynamic<T>`.

use egui_mobius_reactive::Dynamic;
use crate::scanner::QuartusInstall;

/// Central launcher state — every field is reactive.
pub struct LauncherState {
    /// All detected Quartus installations.
    pub installs: Dynamic<Vec<QuartusInstall>>,
    /// Index of the currently selected installation.
    pub selected: Dynamic<Option<usize>>,
    /// Whether to apply the dark QSS theme on launch.
    pub dark_theme: Dynamic<bool>,
    /// Whether to apply editor/RTL colors via qreg.
    pub editor_colors: Dynamic<bool>,
    /// QSYS font size override (0 = auto-detect from DPI).
    pub qsys_fontsize: Dynamic<u32>,
    /// Global UI font scale percentage (100 = default).
    pub ui_scale: Dynamic<u32>,
}

impl LauncherState {
    pub fn new(installs: Vec<QuartusInstall>) -> Self {
        let has_installs = !installs.is_empty();
        Self {
            installs: Dynamic::new(installs),
            selected: Dynamic::new(if has_installs { Some(0) } else { None }),
            dark_theme: Dynamic::new(true),
            editor_colors: Dynamic::new(true),
            qsys_fontsize: Dynamic::new(0),
            ui_scale: Dynamic::new(115),
        }
    }

    /// Get the currently selected install, if any.
    pub fn selected_install(&self) -> Option<QuartusInstall> {
        let idx = self.selected.get()?;
        let list = self.installs.get();
        list.get(idx).cloned()
    }
}
