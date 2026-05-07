use crate::profile::SystemProfile;
#[derive(Debug)]
pub struct ProfileDiff {
    pub missing_pacman_packages: Vec<String>,
    pub missing_aur_packages: Vec<String>,
    pub shell_diff: Option<ShellDiff>,
}

#[derive(Debug)]
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
        shell_diff,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn profile(
        pacman_packages: Vec<&str>,
        aur_packages:Vec<&str>,
        shell: Option <&str>,
    ) -> SystemProfile {
        SystemProfile{
            pacman_packages:pacman_packages.into_iter().map(String::from).collect(),
            aur_packages: aur_packages.into_iter().map(String::from).collect(),
            shell:shell.map(String::from),
            config_paths:vec![],
            system_services:vec![],
            user_services:vec![],
        }
    }
    #[test]
    fn detects_missing_pacman_package()
    {
        let expected = profile(vec!["neovim", "ripgrep"], vec![],Some("/bin/zsh"));
        let current= profile(vec!["neovim" ], vec![],Some("/bin/zsh"));
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_pacman_packages,vec!["ripgrep"]);
    }

    #[test]
    fn detects_missing_aur_package()
    {
        let expected = profile(vec![], vec!["visual-studio-code-bin"],Some("/bin/zsh"));
        let current= profile(vec![], vec![],Some("/bin/zsh"));
        let diff = compare_profiles(&expected, &current);
        assert_eq!(diff.missing_aur_packages,vec!["visual-studio-code-bin"]);
    }
}



