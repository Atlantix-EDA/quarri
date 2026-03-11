//! Quartus Dark Launcher — a Tokyo Night themed egui_mobius launcher
//! for managing multiple Quartus installations with dark theme support.

mod launch;
mod scanner;
mod state;
mod theme;
mod ui;

use state::LauncherState;
use ui::{installs_panel, log_panel, settings_panel};

use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex};

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

enum TabKind {
    Installs,
    Settings,
    Log,
}

struct Tab {
    kind: TabKind,
}

impl Tab {
    fn new(kind: TabKind) -> Self {
        Self { kind }
    }

    fn title(&self) -> &str {
        match self.kind {
            TabKind::Installs => "Installations",
            TabKind::Settings => "Settings",
            TabKind::Log => "Events",
        }
    }

    fn content(&self, ui: &mut egui::Ui, state: &LauncherState, log: &mut Vec<String>) {
        match self.kind {
            TabKind::Installs => installs_panel::InstallsPanel::render(ui, state, log),
            TabKind::Settings => settings_panel::SettingsPanel::render(ui, state, log),
            TabKind::Log => log_panel::LogPanel::render(ui, log),
        }
    }
}

// ---------------------------------------------------------------------------
// Tab viewer bridge for egui_dock
// ---------------------------------------------------------------------------

struct TabViewer<'a> {
    state: &'a LauncherState,
    log: &'a mut Vec<String>,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.content(ui, self.state, self.log);
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

struct LauncherApp {
    splash: ui::splash::Splash,
    show_about: bool,
    dock_state: DockState<Tab>,
    state: LauncherState,
    log: Vec<String>,
}

impl LauncherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_visuals(&cc.egui_ctx);

        // Scan for installations
        let installs = scanner::scan();
        let mut log = Vec::new();
        match installs.len() {
            0 => log.push("No Quartus installations detected".into()),
            n => {
                log.push(format!("Found {n} Quartus installation(s)"));
                for inst in &installs {
                    log.push(format!("  {} — {}", inst.label, inst.bin_path.display()));
                }
            }
        }

        // Dock layout: [Installs | Settings] over [Log]
        let mut dock_state = DockState::new(vec![Tab::new(TabKind::Installs)]);

        let [main, _right] = dock_state.main_surface_mut().split_right(
            NodeIndex::root(),
            0.55,
            vec![Tab::new(TabKind::Settings)],
        );
        let [_, _bottom] = dock_state.main_surface_mut().split_below(
            main,
            0.70,
            vec![Tab::new(TabKind::Log)],
        );

        let start_time = cc.egui_ctx.input(|i| i.time);
        Self {
            splash: ui::splash::Splash::new(start_time),
            show_about: false,
            dock_state,
            state: LauncherState::new(installs),
            log,
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = ctx.input(|i| i.time);

        if self.splash.is_active(now) {
            self.splash.render(ctx, now);
            return;
        }

        ui::top_bar::TopBar::render(ctx, &mut self.show_about);

        DockArea::new(&mut self.dock_state).show(
            ctx,
            &mut TabViewer {
                state: &self.state,
                log: &mut self.log,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Quartus Dark Launcher",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([820.0, 560.0])
                .with_min_inner_size([600.0, 400.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(LauncherApp::new(cc)))),
    )
}
