//! Installations panel — lists detected Quartus installs with a Launch button.

use egui::RichText;
use crate::state::LauncherState;
use crate::theme::TokyoNight;

pub struct InstallsPanel;

impl InstallsPanel {
    pub fn render(ui: &mut egui::Ui, state: &LauncherState, log: &mut Vec<String>) {
        ui.vertical(|ui| {
            ui.heading(
                RichText::new("Quartus Installations")
                    .color(TokyoNight::BLUE)
                    .strong(),
            );
            ui.add_space(8.0);

            let installs = state.installs.get();
            let mut selected = state.selected.get();

            if installs.is_empty() {
                ui.label(
                    RichText::new("No Quartus installations found.")
                        .color(TokyoNight::ORANGE)
                        .italics(),
                );
                ui.add_space(4.0);
                ui.label("Searched: ~/altera*, ~/intel*, /opt/altera*, /opt/intel*");
                return;
            }

            for (i, install) in installs.iter().enumerate() {
                let is_selected = selected == Some(i);
                let bg = if is_selected {
                    TokyoNight::SELECTION
                } else {
                    TokyoNight::BG_DARK
                };

                egui::Frame::new()
                    .fill(bg)
                    .corner_radius(4.0)
                    .inner_margin(8.0)
                    .outer_margin(egui::Margin::symmetric(0, 2))
                    .stroke(egui::Stroke::new(
                        if is_selected { 1.0 } else { 0.5 },
                        if is_selected { TokyoNight::BLUE } else { TokyoNight::BORDER },
                    ))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            let badge_color = match install.edition {
                                crate::scanner::Edition::Pro => TokyoNight::MAGENTA,
                                crate::scanner::Edition::Standard => TokyoNight::CYAN,
                                crate::scanner::Edition::Lite => TokyoNight::GREEN,
                            };
                            ui.label(
                                RichText::new(format!(" {} ", install.edition))
                                    .color(TokyoNight::BG_DARK)
                                    .background_color(badge_color)
                                    .strong()
                                    .monospace(),
                            );
                            ui.add_space(8.0);

                            if ui
                                .selectable_label(
                                    is_selected,
                                    RichText::new(&install.label)
                                        .color(if is_selected {
                                            TokyoNight::CYAN
                                        } else {
                                            TokyoNight::FG
                                        }),
                                )
                                .clicked()
                            {
                                selected = Some(i);
                                state.selected.set(selected);
                                log.push(format!("[INFO] Selected: {}", install.label));
                            }
                        });

                        ui.label(
                            RichText::new(install.bin_path.display().to_string())
                                .color(TokyoNight::FG_DIM)
                                .small()
                                .monospace(),
                        );
                    });
            }

            ui.add_space(12.0);

            let can_launch = selected.is_some();
            ui.add_enabled_ui(can_launch, |ui| {
                let btn = egui::Button::new(
                    RichText::new("  Launch Quartus  ")
                        .color(TokyoNight::BG_DARK)
                        .strong()
                        .size(16.0),
                )
                .fill(TokyoNight::GREEN)
                .corner_radius(6.0);

                if ui.add(btn).clicked() {
                    if let Some(install) = state.selected_install() {
                        let config = crate::launch::LaunchConfig {
                            install: &install,
                            dark_theme: state.dark_theme.get(),
                            qsys_fontsize: state.qsys_fontsize.get(),
                        };
                        match crate::launch::spawn(&config) {
                            Ok(msg) => log.push(format!("[INFO] {msg}")),
                            Err(e) => log.push(format!("[ERROR] {e}")),
                        }
                    }
                }
            });
        });
    }
}
