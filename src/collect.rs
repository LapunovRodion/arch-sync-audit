use crate::profile::SystemProfile;
use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const CONFIG_PATH_CANDIDATES: &[&str] = &["~/.zshrc", "~/.gitconfig", "~/.config/nvim"];

pub fn command_output_lines(command: &str, args: &[&str]) -> Result<Vec<String>> {
    let output = Command::new(command)
        .args(args)
        .output()
        .with_context(|| format!("failed to run command: {command}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("command failed: {command}: {}", stderr.trim());
    }

    let stdout = String::from_utf8(output.stdout)
        .with_context(|| format!("command output is not valid UTF-8: {command}"))?;

    Ok(stdout.lines().map(String::from).collect())
}

fn parse_enabled_services(lines: Vec<String>) -> Vec<String> {
    let mut services: Vec<String> = lines
        .into_iter()
        .filter_map(|line| {
            let mut columns = line.split_whitespace();
            let unit = columns.next()?;
            let state = columns.next()?;

            (unit.ends_with(".service") && state == "enabled").then(|| unit.to_string())
        })
        .collect();

    services.sort();
    services
}

pub fn current_shell() -> Option<String> {
    env::var("SHELL").ok()
}

pub fn aur_packages() -> Result<Vec<String>> {
    let mut packages = command_output_lines("pacman", &["-Qqm"])?;
    packages.sort();
    Ok(packages)
}

pub fn pacman_packages(aur_packages: &[String]) -> Result<Vec<String>> {
    let explicit_packages = command_output_lines("pacman", &["-Qqe"])?;
    let aur_package_set: HashSet<&str> = aur_packages.iter().map(String::as_str).collect();

    let mut packages: Vec<String> = explicit_packages
        .into_iter()
        .filter(|package| !aur_package_set.contains(package.as_str()))
        .collect();

    packages.sort();
    Ok(packages)
}

pub fn config_paths() -> Vec<String> {
    CONFIG_PATH_CANDIDATES
        .iter()
        .filter(|path| expand_home(path).is_some_and(|real_path| real_path.exists()))
        .map(|path| path.to_string())
        .collect()
}

fn expand_home(path: &str) -> Option<PathBuf> {
    if path == "~" {
        return env::var_os("HOME").map(PathBuf::from);
    }

    if let Some(rest) = path.strip_prefix("~/") {
        return env::var_os("HOME").map(|home| PathBuf::from(home).join(rest));
    }

    Some(PathBuf::from(path))
}

pub fn system_services() -> Result<Vec<String>> {
    let lines = command_output_lines(
        "systemctl",
        &["list-unit-files", "--state=enabled", "--type=service"],
    )?;

    Ok(parse_enabled_services(lines))
}

pub fn user_services() -> Vec<String> {
    let lines = match command_output_lines(
        "systemctl",
        &[
            "--user",
            "list-unit-files",
            "--state=enabled",
            "--type=service",
        ],
    ) {
        Ok(lines) => lines,
        Err(error) => {
            eprintln!("warning: failed to collect user services: {error}");
            return vec![];
        }
    };

    parse_enabled_services(lines)
}

pub fn current_profile() -> Result<SystemProfile> {
    let aur_packages = aur_packages()?;
    let pacman_packages = pacman_packages(&aur_packages)?;

    Ok(SystemProfile {
        pacman_packages,
        aur_packages,
        shell: current_shell(),
        config_paths: config_paths(),
        system_services: system_services()?,
        user_services: user_services(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_enabled_services() {
        let services = parse_enabled_services(vec![
            "UNIT FILE STATE PRESET".to_string(),
            "bluetooth.service enabled disabled".to_string(),
            "cups.service disabled disabled".to_string(),
            "getty@.service enabled enabled".to_string(),
            "3 unit files listed.".to_string(),
        ]);

        assert_eq!(services, vec!["bluetooth.service", "getty@.service"]);
    }
}
