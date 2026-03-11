//! Settings panel — theme toggles and font size.

use egui::RichText;
use crate::state::LauncherState;
use crate::theme::TokyoNight;

pub struct SettingsPanel;

impl SettingsPanel {
    pub fn render(ui: &mut egui::Ui, state: &LauncherState, log: &mut Vec<String>) {
        ui.vertical(|ui| {
            ui.heading(
                RichText::new("Launch Settings")
                    .color(TokyoNight::BLUE)
                    .strong(),
            );
            ui.add_space(12.0);

            // Dark theme toggle
            let mut dark = state.dark_theme.get();
            if ui
                .checkbox(
                    &mut dark,
                    RichText::new("Dark QSS Theme (UI chrome)")
                        .color(TokyoNight::FG),
                )
                .changed()
            {
                state.dark_theme.set(dark);
                if dark {
                    log.push("[INFO] Dark QSS theme enabled".into());
                } else {
                    log.push("[WARN] Dark QSS theme disabled".into());
                }
            }
            ui.label(
                RichText::new("  Injects dark stylesheet via LD_PRELOAD")
                    .color(TokyoNight::COMMENT)
                    .small(),
            );
            ui.add_space(8.0);

            // Editor colors toggle
            let mut editor = state.editor_colors.get();
            if ui
                .checkbox(
                    &mut editor,
                    RichText::new("Editor / RTL Dark Colors (qreg)")
                        .color(TokyoNight::FG),
                )
                .changed()
            {
                state.editor_colors.set(editor);
                if editor {
                    log.push("[INFO] Editor colors enabled".into());
                } else {
                    log.push("[WARN] Editor colors disabled".into());
                }
            }
            ui.label(
                RichText::new("  QScintilla editor, RTL viewer, Pin Planner")
                    .color(TokyoNight::COMMENT)
                    .small(),
            );
            ui.add_space(12.0);

            // Font size
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("QSYS Font Size:")
                        .color(TokyoNight::FG),
                );
                let mut size = state.qsys_fontsize.get();
                let slider = egui::Slider::new(&mut size, 0..=36)
                    .text("pt")
                    .custom_formatter(|v, _| {
                        if v == 0.0 {
                            "Auto".to_string()
                        } else {
                            format!("{:.0}", v)
                        }
                    });
                let response = ui.add(slider);
                if response.changed() {
                    state.qsys_fontsize.set(size);
                }
                if response.drag_stopped() || response.lost_focus() {
                    let label = if size == 0 {
                        "auto (DPI-based)".to_string()
                    } else {
                        format!("{size}pt")
                    };
                    log.push(format!("[INFO] QSYS font size: {label}"));
                }
            });

            ui.add_space(8.0);

            // UI font scale
            ui.horizontal(|ui| {
                ui.label(
                    RichText::new("UI Font Scale:")
                        .color(TokyoNight::FG),
                );
                let mut scale = state.ui_scale.get();
                let slider = egui::Slider::new(&mut scale, 80..=160)
                    .text("%")
                    .custom_formatter(|v, _| format!("{:.0}%", v));
                let response = ui.add(slider);
                if response.drag_stopped() || (!response.dragged() && response.changed()) {
                    state.ui_scale.set(scale);
                }
            });

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);

            // Install editor colors button
            let btn = egui::Button::new(
                RichText::new("Install Editor Colors to qreg")
                    .color(TokyoNight::BG_DARK)
                    .strong(),
            )
            .fill(TokyoNight::YELLOW)
            .corner_radius(4.0);

            if ui.add(btn).clicked() {
                match run_install_script() {
                    Ok(msg) => log.push(format!("[INFO] {msg}")),
                    Err(e) => log.push(format!("[ERROR] {e}")),
                }
            }
            ui.label(
                RichText::new("  Patches ~/.altera.quartus/quartus2.qreg")
                    .color(TokyoNight::COMMENT)
                    .small(),
            );
        });
    }
}

/// Baked-in source directory from build time.
const BUILD_SOURCE_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn find_project_root() -> Option<std::path::PathBuf> {
    let marker = "install_linux.sh";
    // 1. Walk up from executable (dev builds)
    if let Ok(exe) = std::env::current_exe() {
        let mut dir = exe.parent();
        while let Some(d) = dir {
            if d.join(marker).is_file() {
                return Some(d.to_path_buf());
            }
            dir = d.parent();
        }
    }
    // 2. Current working directory
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join(marker).is_file() {
            return Some(cwd);
        }
    }
    // 3. Build-time source directory
    //    CARGO_MANIFEST_DIR is either the root (cargo install) or crates/launcher/ (cargo run)
    let build_dir = std::path::Path::new(BUILD_SOURCE_DIR);
    // Try the dir itself, then walk up to find the marker
    let mut dir = Some(build_dir);
    while let Some(d) = dir {
        if d.join(marker).is_file() {
            return Some(d.to_path_buf());
        }
        dir = d.parent();
    }
    None
}

fn run_install_script() -> Result<String, String> {
    let project_root = find_project_root().ok_or(
        "install_linux.sh not found — run quarri from the project directory, \
         or run ./install_linux.sh manually"
    )?;
    let script = project_root.join("install_linux.sh");

    if !script.is_file() {
        return Err(format!("install_linux.sh not found at {}", script.display()));
    }

    let output = std::process::Command::new("bash")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to run install script: {e}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() { "Editor colors installed".into() } else { stdout })
    } else {
        Err(if stderr.is_empty() { stdout } else { stderr })
    }
}
