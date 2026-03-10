//! Log panel — displays launch events with Tokyo Night colors.

use egui::RichText;
use crate::theme::TokyoNight;

pub struct LogPanel;

impl LogPanel {
    pub fn render(ui: &mut egui::Ui, events: &[String]) {
        ui.vertical(|ui| {
            ui.heading(
                RichText::new(format!("Events ({})", events.len()))
                    .color(TokyoNight::BLUE)
                    .strong(),
            );
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for (i, event) in events.iter().enumerate().rev() {
                        let color = if event.starts_with("ERROR") {
                            TokyoNight::RED
                        } else if event.starts_with("Selected") {
                            TokyoNight::MAGENTA
                        } else if event.starts_with("Launched") {
                            TokyoNight::GREEN
                        } else if event.contains("enabled") {
                            TokyoNight::TEAL
                        } else if event.contains("disabled") {
                            TokyoNight::ORANGE
                        } else {
                            TokyoNight::FG_DIM
                        };

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{:>3}", i + 1))
                                    .color(TokyoNight::COMMENT)
                                    .monospace(),
                            );
                            ui.label(
                                RichText::new(event)
                                    .color(color)
                                    .monospace(),
                            );
                        });
                    }
                });
        });
    }
}
