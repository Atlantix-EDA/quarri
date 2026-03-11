//! Logger panel — read-only structured event log, egui_lens-inspired.
//!
//! Log entries are prefixed with [INFO], [WARN], or [ERROR].

use egui::RichText;
use crate::theme::TokyoNight;

pub struct LoggerPanel;

impl LoggerPanel {
    pub fn render(ui: &mut egui::Ui, events: &[String]) {
        let frame = egui::Frame::new()
            .fill(TokyoNight::BG_DARK)
            .inner_margin(8.0);

        frame.show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(
                    RichText::new("Logger")
                        .color(TokyoNight::BLUE)
                        .strong(),
                );
                ui.label(
                    RichText::new(format!("({})", events.len()))
                        .color(TokyoNight::COMMENT)
                        .monospace(),
                );
            });
            ui.add_space(4.0);

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    for (i, event) in events.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{:>3}", i + 1))
                                    .color(TokyoNight::COMMENT)
                                    .monospace(),
                            );

                            let (prefix, prefix_color, msg_color) = classify(event);
                            ui.label(
                                RichText::new(prefix)
                                    .color(prefix_color)
                                    .monospace()
                                    .strong(),
                            );
                            // Strip existing prefix from message text
                            let msg = event
                                .strip_prefix("[INFO] ")
                                .or_else(|| event.strip_prefix("[WARN] "))
                                .or_else(|| event.strip_prefix("[ERROR] "))
                                .unwrap_or(event);
                            ui.label(
                                RichText::new(msg)
                                    .color(msg_color)
                                    .monospace(),
                            );
                        });
                    }
                });
        });
    }
}

/// Classify a log line and return (prefix, prefix_color, message_color).
fn classify(event: &str) -> (&'static str, egui::Color32, egui::Color32) {
    if event.starts_with("[ERROR]") || event.starts_with("ERROR") {
        ("[ERROR] ", TokyoNight::RED, TokyoNight::RED)
    } else if event.starts_with("[WARN]") || event.contains("disabled") {
        ("[WARN]  ", TokyoNight::ORANGE, TokyoNight::ORANGE)
    } else if event.starts_with("[INFO]") {
        ("[INFO]  ", TokyoNight::CYAN, TokyoNight::FG_DIM)
    } else if event.starts_with("Launched") {
        ("[INFO]  ", TokyoNight::GREEN, TokyoNight::GREEN)
    } else if event.starts_with("Found") {
        ("[INFO]  ", TokyoNight::CYAN, TokyoNight::GREEN)
    } else if event.starts_with("Selected") {
        ("[INFO]  ", TokyoNight::CYAN, TokyoNight::MAGENTA)
    } else if event.contains("enabled") {
        ("[INFO]  ", TokyoNight::CYAN, TokyoNight::TEAL)
    } else {
        ("[INFO]  ", TokyoNight::CYAN, TokyoNight::FG_DIM)
    }
}
