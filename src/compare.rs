#[derive(Debug)]
pub struct ProfileDiff{
    pub missing_pacman_packages: Vec<String>,
    pub missing_aur_packages: Vec<String>,
    pub shell_diff: Option<ShellDiff>,
}

#[derive(Debug)]
pub struct ShellDiff{
    pub expected:String,
    pub current : Option<String>,
}

