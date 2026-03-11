//! Shell panel — interactive QuarRI command shell.
//!
//! Full-panel monospace buffer with integrated prompt, like a real terminal.

use egui::RichText;
use crate::theme::TokyoNight;

const PROMPT: &str = "quarri> ";
const INPUT_ID: &str = "shell_input";

pub struct LogPanel;

impl LogPanel {
    pub fn render(ui: &mut egui::Ui, log: &mut Vec<String>, cmd_buf: &mut String) {
        let frame = egui::Frame::new()
            .fill(TokyoNight::BG_DARK)
            .inner_margin(8.0);

        frame.show(ui, |ui| {
            ui.style_mut().visuals.extreme_bg_color = TokyoNight::BG_DARK;

            let text_id = ui.make_persistent_id(INPUT_ID);

            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .stick_to_bottom(true)
                .show(ui, |ui| {
                    // Render history
                    for line in log.iter() {
                        render_line(ui, line);
                    }

                    // Live prompt + input
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label(
                            RichText::new(PROMPT)
                                .color(TokyoNight::CYAN)
                                .monospace()
                                .strong(),
                        );
                        let response = ui.add(
                            egui::TextEdit::singleline(cmd_buf)
                                .id(text_id)
                                .desired_width(ui.available_width())
                                .font(egui::TextStyle::Monospace)
                                .frame(false)
                                .text_color(TokyoNight::FG),
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            let input = cmd_buf.trim().to_string();
                            if !input.is_empty() {
                                if input == "clear" || input == "cls" {
                                    log.clear();
                                } else {
                                    log.push(format!("{PROMPT}{input}"));
                                    let output = execute_command(&input);
                                    for line in output {
                                        log.push(line);
                                    }
                                }
                            }
                            cmd_buf.clear();
                            ui.memory_mut(|m| m.request_focus(text_id));
                        }

                        // Always keep focus on the input when the panel is visible
                        if !response.has_focus() && !ui.ctx().wants_keyboard_input() {
                            ui.memory_mut(|m| m.request_focus(text_id));
                        }
                    });
                });
        });
    }
}

fn render_line(ui: &mut egui::Ui, line: &str) {
    let color = line_color(line);
    ui.label(
        RichText::new(line)
            .color(color)
            .monospace(),
    );
}

fn execute_command(input: &str) -> Vec<String> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let cmd = parts.first().map(|s| s.to_lowercase()).unwrap_or_default();

    match cmd.as_str() {
        "help" | "?" => vec![
            "Available commands:".into(),
            "  help        Show this help".into(),
            "  ver         Show QuarRI version".into(),
            "  info        Show system information".into(),
            "  clear       Clear the shell".into(),
            "  scan        Re-scan for Quartus installations".into(),
            "  env         Show relevant environment variables".into(),
        ],
        "ver" | "version" => vec![
            format!("QuarRI™ v{}", env!("CARGO_PKG_VERSION")),
            "QUARtus launcheR guI — pronounced \"quarry\"".into(),
        ],
        "info" | "system" | "sysinfo" => crate::platform::system_banner(),
        "scan" | "rescan" => {
            let installs = crate::scanner::scan();
            let mut lines = Vec::new();
            match installs.len() {
                0 => lines.push("No Quartus installations detected".into()),
                n => {
                    lines.push(format!("Found {n} Quartus installation(s)"));
                    for inst in &installs {
                        lines.push(format!("  {} — {}", inst.label, inst.bin_path.display()));
                    }
                }
            }
            lines
        }
        "env" => {
            let vars = ["LD_PRELOAD", "QUARTUS_QSS", "QSYS_FONTSIZE", "QUARTUS_ROOTDIR"];
            let mut lines = vec!["Environment:".into()];
            for var in &vars {
                let val = std::env::var(var).unwrap_or_else(|_| "(not set)".into());
                lines.push(format!("  {var}={val}"));
            }
            lines
        }
        "" => vec![],
        other => vec![format!("Unknown command: {other}. Type 'help' for available commands.")],
    }
}

fn line_color(line: &str) -> egui::Color32 {
    if line.starts_with("ERROR") {
        TokyoNight::RED
    } else if line.starts_with("quarri>") {
        TokyoNight::CYAN
    } else if line.starts_with("───") {
        TokyoNight::COMMENT
    } else if line.starts_with("Launched") {
        TokyoNight::GREEN
    } else if line.starts_with("Selected") {
        TokyoNight::MAGENTA
    } else if line.starts_with("Found") {
        TokyoNight::GREEN
    } else if line.contains("enabled") {
        TokyoNight::TEAL
    } else if line.contains("disabled") {
        TokyoNight::ORANGE
    } else if line.starts_with("  ") {
        TokyoNight::FG_DIM
    } else {
        TokyoNight::FG
    }
}
