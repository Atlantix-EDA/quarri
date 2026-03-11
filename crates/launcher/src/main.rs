//! QuarRI™ — a Tokyo Night themed egui_mobius launcher
//! for managing multiple Quartus installations with dark theme support.

mod launch;
mod platform;
mod scanner;
mod state;
mod theme;
mod ui;

use state::LauncherState;
use ui::{installs_panel, log_panel, logger_panel, settings_panel, terminal_panel};

use eframe::egui;
use egui_dock::{DockArea, DockState, NodeIndex};

// ---------------------------------------------------------------------------
// Tabs
// ---------------------------------------------------------------------------

enum TabKind {
    Installs,
    Settings,
    Shell,
    Terminal,
    Logger,
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
            TabKind::Shell => "Shell",
            TabKind::Terminal => "Terminal",
            TabKind::Logger => "Logger",
        }
    }
}

// ---------------------------------------------------------------------------
// Tab viewer bridge for egui_dock
// ---------------------------------------------------------------------------

struct TabViewer<'a> {
    state: &'a LauncherState,
    log: &'a mut Vec<String>,
    shell_log: &'a mut Vec<String>,
    shell_cmd: &'a mut String,
    term_output: &'a mut Vec<String>,
    term_cmd: &'a mut String,
}

impl egui_dock::TabViewer for TabViewer<'_> {
    type Tab = Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title().into()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        match tab.kind {
            TabKind::Installs => installs_panel::InstallsPanel::render(ui, self.state, self.log),
            TabKind::Settings => settings_panel::SettingsPanel::render(ui, self.state, self.log),
            TabKind::Shell => {
                log_panel::LogPanel::render(ui, self.shell_log, self.shell_cmd);
            }
            TabKind::Terminal => {
                terminal_panel::TerminalPanel::render(ui, self.term_output, self.term_cmd);
            }
            TabKind::Logger => logger_panel::LoggerPanel::render(ui, self.log),
        }
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
    shell_log: Vec<String>,
    shell_cmd: String,
    term_output: Vec<String>,
    term_cmd: String,
}

impl LauncherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_visuals(&cc.egui_ctx);

        // System banner goes into the shell
        let mut shell_log = platform::system_banner();

        // Scan for installations
        let installs = scanner::scan();
        let mut log = Vec::new();
        match installs.len() {
            0 => {
                log.push("[WARN] No Quartus installations detected".into());
                shell_log.push("No Quartus installations detected".into());
            }
            n => {
                log.push(format!("[INFO] Found {n} Quartus installation(s)"));
                shell_log.push(format!("Found {n} Quartus installation(s)"));
                for inst in &installs {
                    let line = format!("  {} — {}", inst.label, inst.bin_path.display());
                    log.push(format!("[INFO] {line}"));
                    shell_log.push(line);
                }
            }
        }

        // Dock layout:
        // ┌──────────────┬───────────────┐
        // │ Installations│   Settings    │
        // │              ├───────────────┤
        // ├──────────────┤Logger│Terminal │
        // │    Shell     │               │
        // └──────────────┴───────────────┘
        let mut dock_state = DockState::new(vec![Tab::new(TabKind::Installs)]);

        let [left, right] = dock_state.main_surface_mut().split_right(
            NodeIndex::root(),
            0.47,
            vec![Tab::new(TabKind::Settings)],
        );
        // Split left column: Installations over Shell
        let [_, _shell] = dock_state.main_surface_mut().split_below(
            left,
            0.50,
            vec![Tab::new(TabKind::Shell)],
        );
        // Split right column: Settings over Logger/Terminal
        let [_, _bottom_right] = dock_state.main_surface_mut().split_below(
            right,
            0.50,
            vec![Tab::new(TabKind::Logger), Tab::new(TabKind::Terminal)],
        );

        let start_time = cc.egui_ctx.input(|i| i.time);
        Self {
            splash: ui::splash::Splash::new(start_time),
            show_about: false,
            dock_state,
            state: LauncherState::new(installs),
            log,
            shell_log,
            shell_cmd: String::new(),
            term_output: Vec::new(),
            term_cmd: String::new(),
        }
    }
}

impl eframe::App for LauncherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply font scale from state
        theme::apply_font_scale(ctx, self.state.ui_scale.get());

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
                shell_log: &mut self.shell_log,
                shell_cmd: &mut self.shell_cmd,
                term_output: &mut self.term_output,
                term_cmd: &mut self.term_cmd,
            },
        );
    }
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "QuarRI™",
        eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([900.0, 620.0])
                .with_min_inner_size([600.0, 400.0]),
            ..Default::default()
        },
        Box::new(|cc| Ok(Box::new(LauncherApp::new(cc)))),
    )
}
