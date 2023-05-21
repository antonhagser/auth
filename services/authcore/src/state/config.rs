use crypto::input::password::PasswordRequirements;

pub static CONFIG: once_cell::sync::Lazy<Config> = once_cell::sync::Lazy::new(Config::default);

#[derive(Debug, Default)]
pub struct Config {
    default_password_requirements: PasswordRequirements,
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

    pub fn default_password_requirements(&self) -> PasswordRequirements {
        self.default_password_requirements
    }
}
