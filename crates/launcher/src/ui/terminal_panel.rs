//! Terminal panel — OS shell escape for running external commands.
//!
//! Full-panel monospace buffer with bash prompt, like a real terminal.

use egui::RichText;
use crate::theme::TokyoNight;

const PROMPT: &str = "$ ";
const INPUT_ID: &str = "terminal_input";

pub struct TerminalPanel;

impl TerminalPanel {
    pub fn render(ui: &mut egui::Ui, output: &mut Vec<String>, cmd_buf: &mut String) {
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
                    for line in output.iter() {
                        render_line(ui, line);
                    }

                    // Live prompt + input
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label(
                            RichText::new(PROMPT)
                                .color(TokyoNight::GREEN)
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
                                    output.clear();
                                } else {
                                    output.push(format!("{PROMPT}{input}"));
                                    let result = run_shell_command(&input);
                                    for line in result {
                                        output.push(line);
                                    }
                                }
                            }
                            cmd_buf.clear();
                            ui.memory_mut(|m| m.request_focus(text_id));
                        }

                        // Keep focus on input when panel is visible
                        if !response.has_focus() && !ui.ctx().wants_keyboard_input() {
                            ui.memory_mut(|m| m.request_focus(text_id));
                        }
                    });
                });
        });
    }
}

fn render_line(ui: &mut egui::Ui, line: &str) {
    let color = if line.starts_with("$ ") {
        TokyoNight::GREEN
    } else if line.starts_with("error:") || line.starts_with("ERROR") {
        TokyoNight::RED
    } else {
        TokyoNight::FG_DIM
    };
    ui.label(
        RichText::new(line)
            .color(color)
            .monospace(),
    );
}

fn run_shell_command(input: &str) -> Vec<String> {
    match std::process::Command::new("bash")
        .arg("-c")
        .arg(input)
        .output()
    {
        Ok(output) => {
            let mut lines = Vec::new();
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            for line in stdout.lines() {
                lines.push(line.to_string());
            }
            for line in stderr.lines() {
                lines.push(format!("error: {line}"));
            }
            if lines.is_empty() && !output.status.success() {
                lines.push(format!("error: exit code {}", output.status));
            }
            lines
        }
        Err(e) => vec![format!("error: {e}")],
    }
}
