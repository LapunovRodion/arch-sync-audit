use crate::profile::SystemProfile;
#[derive(Debug, PartialEq)]
pub struct ProfileDiff {
    pub missing_pacman_packages: Vec<String>,
    pub missing_aur_packages: Vec<String>,
    pub missing_config_paths: Vec<String>,
    pub missing_system_services: Vec<String>,
    pub missing_user_services: Vec<String>,
    pub shell_diff: Option<ShellDiff>,
}

#[derive(Debug, PartialEq)]
pub struct ShellDiff {
    pub expected: String,
    pub current: Option<String>,
}
pub fn compare_profiles(expected: &SystemProfile, current: &SystemProfile) -> ProfileDiff {
    let missing_pacman_packages = expected
        .pacman_packages
        .iter()
        .filter(|package| !current.pacman_packages.contains(package))
        .cloned()
        .collect();
    let missing_aur_packages = expected
        .aur_packages
        .iter()
        .filter(|package| !current.aur_packages.contains(package))
        .cloned()
        .collect();
    let missing_config_paths = expected
        .config_paths
        .iter()
        .filter(|path| !current.config_paths.contains(path))
        .cloned()
        .collect();
    let missing_system_services = expected
        .system_services
        .iter()
        .filter(|service| !current.system_services.contains(service))
        .cloned()
        .collect();
    let missing_user_services = expected
        .user_services
        .iter()
        .filter(|service| !current.user_services.contains(service))
        .cloned()
        .collect();
    let shell_diff = if expected.shell != current.shell {
        expected.shell.as_ref().map(|exprected_shell| ShellDiff {
            expected: exprected_shell.clone(),
            current: current.shell.clone(),
        })
    } else {
        None
    };

    ProfileDiff {
        missing_pacman_packages,
        missing_aur_packages,
        missing_config_paths,
        missing_system_services,
        missing_user_services,
        shell_diff,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn profile(
        pacman_packages: Vec<&str>,
        aur_packages: Vec<&str>,
        shell: Option<&str>,
        config_paths: Vec<&str>,
        system_services: Vec<&str>,
        user_services: Vec<&str>,
    ) -> SystemProfile {
        SystemProfile {
            pacman_packages: pacman_packages.into_iter().map(String::from).collect(),
            aur_packages: aur_packages.into_iter().map(String::from).collect(),
            shell: shell.map(String::from),
            config_paths: config_paths.into_iter().map(String::from).collect(),
            system_services: system_services.into_iter().map(String::from).collect(),
            user_services: user_services.into_iter().map(String::from).collect(),
        }
    }
    #[test]
    fn detects_missing_pacman_package() {
        let expected = profile(
            vec!["neovim", "ripgrep"],
            vec![],
            Some("/bin/zsh"),
            vec![],
            vec![],
            vec![],
        );
        let current = profile(
            vec!["neovim"],
            vec![],
            Some("/bin/zsh"),
            vec![],
            vec![],
            vec![],
        );
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_pacman_packages, vec!["ripgrep"]);
    }

    #[test]
    fn detects_missing_aur_package() {
        let expected = profile(
            vec![],
            vec!["visual-studio-code-bin"],
            Some("/bin/zsh"),
            vec![],
            vec![],
            vec![],
        );
        let current = profile(vec![], vec![], Some("/bin/zsh"), vec![], vec![], vec![]);
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_aur_packages, vec!["visual-studio-code-bin"]);
    }

    #[test]
    fn detects_different_shell() {
        let expected = profile(vec![], vec![], Some("/bin/zsh"), vec![], vec![], vec![]);
        let current = profile(vec![], vec![], Some("/bin/bash"), vec![], vec![], vec![]);
        let diff = compare_profiles(&expected, &current);
        assert_eq!(
            diff.shell_diff,
            Some(ShellDiff {
                expected: "/bin/zsh".to_string(),
                current: Some("/bin/bash".to_string()),
            })
        );
    }

    #[test]
    fn detects_missing_config_path() {
        let expected = profile(
            vec![],
            vec![],
            Some("/bin/zsh"),
            vec!["~/.config/nvim"],
            vec![],
            vec![],
        );
        let current = profile(vec![], vec![], Some("/bin/zsh"), vec![], vec![], vec![]);
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_config_paths, vec!["~/.config/nvim"]);
    }

    #[test]
    fn detects_missing_system_service() {
        let expected = profile(
            vec![],
            vec![],
            Some("/bin/zsh"),
            vec![],
            vec!["bluetooth.service"],
            vec![],
        );
        let current = profile(vec![], vec![], Some("/bin/zsh"), vec![], vec![], vec![]);
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_system_services, vec!["bluetooth.service"]);
    }

    #[test]
    fn detects_missing_user_service() {
        let expected = profile(
            vec![],
            vec![],
            Some("/bin/zsh"),
            vec![],
            vec![],
            vec!["pipewire.service"],
        );
        let current = profile(vec![], vec![], Some("/bin/zsh"), vec![], vec![], vec![]);
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_user_services, vec!["pipewire.service"]);
    }

    #[test]
    fn returns_empty_diff_when_profiles_match() {
        let expected = profile(
            vec!["neovim", "ripgrep"],
            vec!["visual-studio-code-bin"],
            Some("/bin/zsh"),
            vec!["~/.zshrc"],
            vec!["bluetooth.service"],
            vec!["pipewire.service"],
        );
        let current = profile(
            vec!["neovim", "ripgrep"],
            vec!["visual-studio-code-bin"],
            Some("/bin/zsh"),
            vec!["~/.zshrc"],
            vec!["bluetooth.service"],
            vec!["pipewire.service"],
        );
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_pacman_packages, Vec::<String>::new());
        assert_eq!(diff.missing_aur_packages, Vec::<String>::new());
        assert_eq!(diff.missing_config_paths, Vec::<String>::new());
        assert_eq!(diff.missing_system_services, Vec::<String>::new());
        assert_eq!(diff.missing_user_services, Vec::<String>::new());
        assert_eq!(diff.shell_diff, None);
    }
}
