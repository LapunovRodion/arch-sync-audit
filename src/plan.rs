use crate::compare::ProfileDiff;

pub fn render_plan(diff: &ProfileDiff) -> String {
    let mut lines = Vec::new();

    if !diff.missing_pacman_packages.is_empty() {
        lines.push(format!(
            "sudo pacman -S {}",
            diff.missing_pacman_packages.join(" ")
        ));
    }

    if !diff.missing_aur_packages.is_empty() {
        lines.push("# Install AUR packages manually:".to_string());
        for package in &diff.missing_aur_packages {
            lines.push(format!("# {package}"));
        }
    }

    if !diff.missing_config_paths.is_empty() {
        lines.push("# Create or restore config paths manually:".to_string());
        for path in &diff.missing_config_paths {
            lines.push(format!("# {path}"));
        }
    }

    if !diff.missing_system_services.is_empty() {
        lines.push("# Enable system services manually:".to_string());
        for service in &diff.missing_system_services {
            lines.push(format!("# sudo systemctl enable {service}"));
        }
    }

    if !diff.missing_user_services.is_empty() {
        lines.push("# Enable user services manually:".to_string());
        for service in &diff.missing_user_services {
            lines.push(format!("# systemctl --user enable {service}"));
        }
    }

    if let Some(shell_diff) = &diff.shell_diff {
        lines.push(format!("chsh -s {}", shell_diff.expected));
    }

    if lines.is_empty() {
        "No actions needed.".to_string()
    } else {
        lines.join("\n")
    }
}
