//! Launch Quartus with the dark theme environment.

use std::path::{Path, PathBuf};
use std::process::Command;
use crate::scanner::QuartusInstall;

/// Baked-in source directory from build time.
const BUILD_SOURCE_DIR: &str = env!("CARGO_MANIFEST_DIR");

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

/// Find the project root by checking multiple locations.
fn find_project_root() -> Option<PathBuf> {
    // 1. Walk up from executable (works for cargo run / dev builds)
    if let Ok(exe) = std::env::current_exe() {
        let mut dir = exe.parent();
        while let Some(d) = dir {
            if d.join("darkstyle.qss").is_file() {
                return Some(d.to_path_buf());
            }
            dir = d.parent();
        }
    }
    // 2. Current working directory
    if let Ok(cwd) = std::env::current_dir() {
        if cwd.join("darkstyle.qss").is_file() {
            return Some(cwd);
        }
    }
    // 3. Build-time source directory
    //    CARGO_MANIFEST_DIR is either the root (cargo install) or crates/launcher/ (cargo run)
    let build_dir: &Path = Path::new(BUILD_SOURCE_DIR);
    let mut check = Some(build_dir);
    while let Some(d) = check {
        if d.join("darkstyle.qss").is_file() {
            return Some(d.to_path_buf());
        }
        check = d.parent();
    }
    // 4. Check ~/.config/quarri/project_root if user configured it
    if let Some(config_dir) = dirs::config_dir() {
        let config_file = config_dir.join("quarri/project_root");
        if let Ok(path) = std::fs::read_to_string(&config_file) {
            let root = PathBuf::from(path.trim());
            if root.join("darkstyle.qss").is_file() {
                return Some(root);
            }
        }
    }
    None
}

/// Spawn Quartus as a detached child process.
pub fn spawn(config: &LaunchConfig<'_>) -> Result<String, String> {
    let project_dir = find_project_root()
        .ok_or("Project root not found — run quarri from the project directory, or set ~/.config/quarri/project_root")?;

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
