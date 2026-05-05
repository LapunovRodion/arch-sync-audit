use serde::{Serialize,Deserialize};
#[derive(Debug,Serialize,Deserialize)]
pub struct SystemProfile
{
    pub pacman_packages: Vec<String>,
    pub aur_packages: Vec<String>,
    pub shell: Option<String>,
    pub config_paths: Vec <String>,
    pub system_services: Vec <String>,
    pub user_services: Vec <String>,
}
