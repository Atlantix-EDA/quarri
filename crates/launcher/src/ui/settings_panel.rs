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
                let status = if dark { "enabled" } else { "disabled" };
                log.push(format!("Dark QSS theme {status}"));
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
                let status = if editor { "enabled" } else { "disabled" };
                log.push(format!("Editor colors {status}"));
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
                if ui.add(slider).changed() {
                    state.qsys_fontsize.set(size);
                    let label = if size == 0 {
                        "auto (DPI-based)".to_string()
                    } else {
                        format!("{size}pt")
                    };
                    log.push(format!("QSYS font size: {label}"));
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
                    Ok(msg) => log.push(msg),
                    Err(e) => log.push(format!("ERROR: {e}")),
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

fn run_install_script() -> Result<String, String> {
    let script = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent()?.parent()?.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        .join("install_linux.sh");

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
