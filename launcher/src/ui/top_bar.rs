//! Top bar — clock, timezone, about button, and branding.

use egui::RichText;
use crate::theme::TokyoNight;

pub struct TopBar;

impl TopBar {
    pub fn render(ctx: &egui::Context, show_about: &mut bool) {
        egui::TopBottomPanel::top("top_bar")
            .frame(
                egui::Frame::new()
                    .fill(TokyoNight::BG_DARK)
                    .inner_margin(egui::Margin::symmetric(12, 6))
                    .stroke(egui::Stroke::new(0.5, TokyoNight::BORDER)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Left: branding
                    ui.label(
                        RichText::new("QuarRI™")
                            .color(TokyoNight::BLUE)
                            .strong()
                            .size(16.0),
                    );
                    ui.label(
                        RichText::new("QUARtus launcheR guI")
                            .color(TokyoNight::FG_DIM)
                            .size(14.0),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Right: clock + timezone
                        let now = chrono::Local::now();
                        let time_str = now.format("%H:%M:%S").to_string();
                        let date_str = now.format("%a %d %b %Y").to_string();
                        let tz_str = now.format("%Z").to_string();

                        ui.label(
                            RichText::new(&tz_str)
                                .color(TokyoNight::COMMENT)
                                .monospace()
                                .size(11.0),
                        );
                        ui.label(
                            RichText::new(&time_str)
                                .color(TokyoNight::CYAN)
                                .monospace()
                                .strong()
                                .size(14.0),
                        );
                        ui.separator();
                        ui.label(
                            RichText::new(&date_str)
                                .color(TokyoNight::FG_DIM)
                                .size(12.0),
                        );
                        ui.separator();

                        // About "i" button
                        let about_btn = ui.add(
                            egui::Button::new(
                                RichText::new(" i ")
                                    .color(TokyoNight::BG_DARK)
                                    .strong()
                                    .size(13.0),
                            )
                            .fill(TokyoNight::BLUE)
                            .corner_radius(10.0),
                        );
                        if about_btn.clicked() {
                            *show_about = true;
                        }
                        about_btn.on_hover_text("About QuarRI");

                        // Version next to "i" button
                        ui.label(
                            RichText::new(concat!("v", env!("CARGO_PKG_VERSION")))
                                .color(TokyoNight::FG_DIM)
                                .size(12.0),
                        );
                    });
                });
            });

        // About modal
        if *show_about {
            Self::show_about_modal(ctx, show_about);
        }

        ctx.request_repaint_after(std::time::Duration::from_secs(1));
    }

    fn show_about_modal(ctx: &egui::Context, open: &mut bool) {
        egui::Window::new("About QuarRI")
            .collapsible(false)
            .resizable(true)
            .title_bar(false)
            .min_size([400.0, 300.0])
            .default_size([480.0, 560.0])
            .frame(
                egui::Frame::new()
                    .fill(TokyoNight::BG_DARK)
                    .stroke(egui::Stroke::new(1.0, TokyoNight::BLUE))
                    .corner_radius(8.0)
                    .inner_margin(32.0),
            )
            .show(ctx, |ui| {
                // Custom close button in top-right corner
                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                    let close_btn = ui.add(
                        egui::Button::new(
                            RichText::new(" ✕ ")
                                .color(TokyoNight::FG_DIM)
                                .size(16.0),
                        )
                        .frame(false),
                    );
                    if close_btn.clicked() {
                        *open = false;
                    }
                });

                ui.vertical_centered(|ui| {
                    ui.label(
                        RichText::new("QuarRI™")
                            .color(TokyoNight::BLUE)
                            .strong()
                            .size(48.0),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("QUARtus launcheR guI")
                            .color(TokyoNight::FG_DIM)
                            .size(18.0),
                    );
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("pronounced \"quarry\" — dig into your FPGA designs")
                            .color(TokyoNight::COMMENT)
                            .italics()
                            .size(13.0),
                    );
                    ui.add_space(16.0);

                    ui.label(
                        RichText::new(concat!("Version ", env!("CARGO_PKG_VERSION")))
                            .color(TokyoNight::BLUE)
                            .strong()
                            .size(16.0),
                    );
                    ui.add_space(20.0);

                    ui.label(
                        RichText::new(
                            "A dark-themed launcher and configuration tool\n\
                             for Intel Quartus Prime on Linux."
                        )
                        .color(TokyoNight::FG)
                        .size(15.0),
                    );
                    ui.add_space(16.0);

                    ui.separator();
                    ui.add_space(12.0);

                    let features = [
                        ("Dark QSS Theme", "LD_PRELOAD injection for full UI dark mode"),
                        ("Editor Colors", "QScintilla / RTL / Pin Planner via qreg"),
                        ("Multi-Install", "Auto-detect Pro, Standard, and Lite editions"),
                    ];

                    egui::Grid::new("about_features")
                        .num_columns(2)
                        .spacing([16.0, 10.0])
                        .show(ui, |ui| {
                            for (name, desc) in &features {
                                ui.label(
                                    RichText::new(*name)
                                        .color(TokyoNight::BLUE)
                                        .strong()
                                        .size(14.0),
                                );
                                ui.label(
                                    RichText::new(*desc)
                                        .color(TokyoNight::FG_DIM)
                                        .size(14.0),
                                );
                                ui.end_row();
                            }
                        });

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(12.0);

                    ui.label(
                        RichText::new("James Bonanno — Atlantix EDA")
                            .color(TokyoNight::CYAN)
                            .strong()
                            .size(15.0),
                    );
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("atlantix-eda@proton.me")
                            .color(TokyoNight::FG_DIM)
                            .size(13.0),
                    );
                    ui.add_space(12.0);

                    ui.label(
                        RichText::new("Licensed under the Mozilla Public License 2.0")
                            .color(TokyoNight::BLUE)
                            .strong()
                            .size(14.0),
                    );
                    ui.add_space(12.0);

                    ui.separator();
                    ui.add_space(8.0);

                    ui.label(
                        RichText::new("Built with egui_mobius + Tokyo Night Storm")
                            .color(TokyoNight::COMMENT)
                            .size(13.0),
                    );
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("github.com/saturn77/quartus-dark-linux")
                            .color(TokyoNight::COMMENT)
                            .size(13.0),
                    );
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("github.com/saturn77/egui_mobius")
                            .color(TokyoNight::COMMENT)
                            .size(13.0),
                    );
                    ui.add_space(12.0);
                });
            });
    }
}
