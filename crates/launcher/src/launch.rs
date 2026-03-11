//! Launch Quartus with the dark theme environment.

use std::path::Path;
use std::process::Command;
use crate::scanner::QuartusInstall;

/// Resolve the QSS stylesheet, substituting icon paths.
fn resolve_qss(project_dir: &Path) -> Option<String> {
    let source = project_dir.join("darkstyle.qss");
    let content = std::fs::read_to_string(&source).ok()?;
    let icons_dir = project_dir.join("dark_icons");
    let resolved = content.replace(
        ":/dark_icons/",
        &format!("{}/", icons_dir.display()),
    );
    let out = std::env::temp_dir().join("quartus_darkstyle_resolved.qss");
    std::fs::write(&out, &resolved).ok()?;
    Some(out.to_string_lossy().to_string())
}

/// Find the inject library.
fn inject_lib(project_dir: &Path) -> Option<String> {
    let candidates = [
        project_dir.join("target/release/libqss_inject.so"),
        project_dir.join("crates/inject/target/release/libqss_inject.so"),
    ];
    candidates
        .iter()
        .find(|p| p.is_file())
        .map(|p| p.to_string_lossy().to_string())
}

/// Launch parameters built from the UI state.
pub struct LaunchConfig<'a> {
    pub install: &'a QuartusInstall,
    pub dark_theme: bool,
    pub qsys_fontsize: u32,
}

/// Spawn Quartus as a detached child process.
pub fn spawn(config: &LaunchConfig<'_>) -> Result<String, String> {
    let project_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent()?.parent()?.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

    let mut cmd = Command::new(&config.install.bin_path);

    if config.dark_theme {
        let qss = resolve_qss(&project_dir)
            .ok_or("Failed to resolve QSS stylesheet")?;
        let lib = inject_lib(&project_dir)
            .ok_or("libqss_inject.so not found — run `cargo build --release -p qss_inject`")?;

        cmd.env("QUARTUS_QSS", &qss);

        // Append to existing LD_PRELOAD if set
        let preload = match std::env::var("LD_PRELOAD") {
            Ok(existing) if !existing.is_empty() => format!("{existing}:{lib}"),
            _ => lib,
        };
        cmd.env("LD_PRELOAD", preload);
    }

    if config.qsys_fontsize > 0 {
        cmd.env("QSYS_FONTSIZE", config.qsys_fontsize.to_string());
    }

    cmd.spawn()
        .map(|child| format!("Launched {} (pid {})", config.install.label, child.id()))
        .map_err(|e| format!("Failed to launch: {e}"))
}
