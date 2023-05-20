use crate::core::registration::basic_auth::password::PasswordRequirements;

pub static CONFIG: once_cell::sync::Lazy<Config> = once_cell::sync::Lazy::new(Config::default);

#[derive(Debug, Default)]
pub struct Config {
    pub default_password_requirements: PasswordRequirements,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn set_default_password_requirements(&mut self, requirements: PasswordRequirements) {
        self.default_password_requirements = requirements;
    }
}
