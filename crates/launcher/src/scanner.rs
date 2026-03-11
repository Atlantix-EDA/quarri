//! Auto-detect Quartus installations on the filesystem.

use std::path::{Path, PathBuf};

/// A discovered Quartus installation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QuartusInstall {
    /// Display label, e.g. "Quartus Pro 25.3.1"
    pub label: String,
    /// Edition: Pro, Standard, or Lite
    pub edition: Edition,
    /// Version string parsed from the path
    pub version: String,
    /// Absolute path to the quartus binary
    pub bin_path: PathBuf,
    /// Root installation directory
    pub root_dir: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Edition {
    Pro,
    Standard,
    Lite,
}

impl std::fmt::Display for Edition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Edition::Pro => write!(f, "Pro"),
            Edition::Standard => write!(f, "Standard"),
            Edition::Lite => write!(f, "Lite"),
        }
    }
}

/// Scan standard locations for Quartus installations.
pub fn scan() -> Vec<QuartusInstall> {
    let home = dirs::home_dir().unwrap_or_default();
    let search_roots: Vec<PathBuf> = [
        "/opt/altera",
        "/opt/intel",
        "/opt/intelFPGA",
        "/opt/intelFPGA_pro",
        "/opt/intelFPGA_lite",
    ]
    .iter()
    .map(PathBuf::from)
    .chain(
        ["altera", "altera_pro", "altera_lite",
         "intel", "intelFPGA", "intelFPGA_pro", "intelFPGA_lite"]
        .iter()
        .map(|d| home.join(d)),
    )
    .collect();

    let mut installs = Vec::new();
    for root in &search_roots {
        collect_from(root, &mut installs);
    }
    installs.sort_by(|a, b| b.version.cmp(&a.version));
    installs
}

fn collect_from(root: &Path, out: &mut Vec<QuartusInstall>) {
    if !root.is_dir() {
        return;
    }
    // Structure: <root>/<version>/quartus/bin/quartus
    let Ok(entries) = std::fs::read_dir(root) else { return };
    for entry in entries.flatten() {
        let version_dir = entry.path();
        let bin = version_dir.join("quartus/bin/quartus");
        if bin.is_file() {
            let version = version_dir
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let edition = detect_edition(root, &version_dir);
            let label = format!("Quartus {} {}", edition, version);
            out.push(QuartusInstall {
                label,
                edition,
                version,
                bin_path: bin,
                root_dir: version_dir,
            });
        }
    }
}

fn detect_edition(search_root: &Path, _version_dir: &Path) -> Edition {
    let name = search_root.to_string_lossy().to_lowercase();
    if name.contains("pro") {
        Edition::Pro
    } else if name.contains("lite") {
        Edition::Lite
    } else {
        Edition::Standard
    }
}
