use crate::compare::ProfileDiff;

pub fn render_report(diff: &ProfileDiff) -> String {
    let mut lines = vec!["System audit report".to_string(), String::new()];

    push_list_section(&mut lines, "Pacman packages", &diff.missing_pacman_packages);
    push_list_section(&mut lines, "AUR packages", &diff.missing_aur_packages);
    push_list_section(&mut lines, "Config paths", &diff.missing_config_paths);
    push_list_section(&mut lines, "System services", &diff.missing_system_services);
    push_list_section(&mut lines, "User services", &diff.missing_user_services);
    push_shell_section(&mut lines, diff);

    if is_empty_diff(diff) {
        lines.push("No differences found.".to_string());
    }

    lines.join("\n")
}

fn push_list_section(lines: &mut Vec<String>, title: &str, missing: &[String]) {
    lines.push(format!("{title}:"));
    lines.push(format!("  missing: {}", missing.len()));

    for item in missing {
        lines.push(format!("  - {item}"));
    }

    lines.push(String::new());
}

fn push_shell_section(lines: &mut Vec<String>, diff: &ProfileDiff) {
    lines.push("Shell:".to_string());

    if let Some(shell_diff) = &diff.shell_diff {
        lines.push(format!("  expected: {}", shell_diff.expected));
        lines.push(format!(
            "  current:  {}",
            shell_diff.current.as_deref().unwrap_or("unknown")
        ));
    } else {
        lines.push("  status: ok".to_string());
    }

    lines.push(String::new());
}

fn is_empty_diff(diff: &ProfileDiff) -> bool {
    diff.missing_pacman_packages.is_empty()
        && diff.missing_aur_packages.is_empty()
        && diff.missing_config_paths.is_empty()
        && diff.missing_system_services.is_empty()
        && diff.missing_user_services.is_empty()
        && diff.shell_diff.is_none()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compare::ShellDiff;

    #[test]
    fn renders_report_with_counts_and_shell_diff() {
        let diff = ProfileDiff {
            missing_pacman_packages: vec!["ripgrep".to_string()],
            missing_aur_packages: vec![],
            missing_config_paths: vec!["~/.config/nvim".to_string()],
            missing_system_services: vec![],
            missing_user_services: vec![],
            shell_diff: Some(ShellDiff {
                expected: "/bin/zsh".to_string(),
                current: Some("/bin/bash".to_string()),
            }),
        };

        let report = render_report(&diff);

        assert!(report.contains("System audit report"));
        assert!(report.contains("Pacman packages:\n  missing: 1\n  - ripgrep"));
        assert!(report.contains("Config paths:\n  missing: 1\n  - ~/.config/nvim"));
        assert!(report.contains("Shell:\n  expected: /bin/zsh\n  current:  /bin/bash"));
    }
}
