//! Platform details — OS, CPU, memory gathered at startup.

use sysinfo::System;

/// Gather system details and return as log lines for the Events panel.
pub fn system_banner() -> Vec<String> {
    let mut lines = Vec::new();

    lines.push("─── System ───".into());

    // OS from /etc/os-release (more accurate than sysinfo on Linux)
    let os_name = read_os_pretty_name()
        .or_else(|| System::name())
        .unwrap_or_else(|| "Unknown".into());
    lines.push(format!("  OS             : {os_name}"));

    if let Some(kernel) = System::kernel_version() {
        lines.push(format!("  Kernel         : {kernel}"));
    }
    if let Some(host) = System::host_name() {
        lines.push(format!("  Host           : {host}"));
    }

    lines.push("─── CPU ───".into());

    let mut sys = System::new();
    sys.refresh_cpu_all();

    if let Some(cpu) = sys.cpus().first() {
        lines.push(format!("  CPU            : {}", cpu.brand()));
        lines.push(format!("  Frequency      : {:.2} GHz", cpu.frequency() as f64 / 1000.0));
    }
    if let Some(phys) = System::physical_core_count() {
        lines.push(format!("  Physical cores : {phys}"));
    }
    lines.push(format!("  Logical cores  : {}", sys.cpus().len()));

    lines.push("─── Memory ───".into());

    sys.refresh_memory();
    let gb = |bytes: u64| bytes as f64 / 1024.0 / 1024.0 / 1024.0;
    lines.push(format!("  Total          : {:.1} GB", gb(sys.total_memory())));
    lines.push(format!("  Available      : {:.1} GB", gb(sys.available_memory())));
    lines.push(format!("  Used           : {:.1} GB", gb(sys.used_memory())));

    lines.push("───────────────".into());

    lines
}

/// Read PRETTY_NAME from /etc/os-release for accurate distro detection.
fn read_os_pretty_name() -> Option<String> {
    let content = std::fs::read_to_string("/etc/os-release").ok()?;
    for line in content.lines() {
        if line.starts_with("PRETTY_NAME=") {
            return Some(
                line.trim_start_matches("PRETTY_NAME=")
                    .trim_matches('"')
                    .to_string(),
            );
        }
    }
    None
}
