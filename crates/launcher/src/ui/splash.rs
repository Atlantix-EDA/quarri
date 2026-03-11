//! Splash screen — "QuarRI™" branding on launch.

use egui::{Align2, Color32, FontId};
use crate::theme::TokyoNight;

const SPLASH_DURATION: f64 = 2.5; // seconds

pub struct Splash {
    start_time: f64,
}

impl Splash {
    pub fn new(start_time: f64) -> Self {
        Self { start_time }
    }

    /// Returns true while the splash should still be shown.
    pub fn is_active(&self, current_time: f64) -> bool {
        current_time - self.start_time < SPLASH_DURATION
    }

    pub fn render(&self, ctx: &egui::Context, current_time: f64) {
        let elapsed = current_time - self.start_time;
        let progress = (elapsed / SPLASH_DURATION).min(1.0);

        // Fade out in the last 0.5s
        let alpha = if elapsed > SPLASH_DURATION - 0.5 {
            ((SPLASH_DURATION - elapsed) / 0.5).max(0.0)
        } else {
            // Fade in over first 0.3s
            (elapsed / 0.3).min(1.0)
        };

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(TokyoNight::BG_DARK))
            .show(ctx, |ui| {
                let rect = ui.available_rect_before_wrap();
                let center = rect.center();
                let painter = ui.painter();

                // "QuarRI™" title
                let title_color = color_with_alpha(TokyoNight::BLUE, alpha);
                painter.text(
                    center + egui::vec2(0.0, -40.0),
                    Align2::CENTER_CENTER,
                    "QuarRI™",
                    FontId::proportional(72.0),
                    title_color,
                );

                // Subtitle
                let sub_color = color_with_alpha(TokyoNight::FG_DIM, alpha);
                painter.text(
                    center + egui::vec2(0.0, 20.0),
                    Align2::CENTER_CENTER,
                    "QUARtus launcheR guI",
                    FontId::proportional(16.0),
                    sub_color,
                );

                // Version
                let ver_color = color_with_alpha(TokyoNight::COMMENT, alpha);
                painter.text(
                    center + egui::vec2(0.0, 50.0),
                    Align2::CENTER_CENTER,
                    concat!("v", env!("CARGO_PKG_VERSION")),
                    FontId::proportional(13.0),
                    ver_color,
                );

                // Loading bar
                let bar_width = 200.0;
                let bar_height = 3.0;
                let bar_y = center.y + 75.0;
                let bar_left = center.x - bar_width / 2.0;

                // Track background
                let track = egui::Rect::from_min_size(
                    egui::pos2(bar_left, bar_y),
                    egui::vec2(bar_width, bar_height),
                );
                painter.rect_filled(
                    track,
                    bar_height / 2.0,
                    color_with_alpha(TokyoNight::BG_HIGHLIGHT, alpha),
                );

                // Fill
                let fill_width = bar_width * progress as f32;
                let fill = egui::Rect::from_min_size(
                    egui::pos2(bar_left, bar_y),
                    egui::vec2(fill_width, bar_height),
                );
                painter.rect_filled(
                    fill,
                    bar_height / 2.0,
                    color_with_alpha(TokyoNight::CYAN, alpha),
                );
            });

        // Keep repainting during splash
        ctx.request_repaint();
    }
}

fn color_with_alpha(color: Color32, alpha: f64) -> Color32 {
    let a = (alpha * 255.0) as u8;
    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a)
}
