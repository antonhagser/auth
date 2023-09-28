use crypto::input::password::PasswordRequirements;

pub static CONFIG: once_cell::sync::Lazy<Config> = once_cell::sync::Lazy::new(Config::new);

#[derive(Debug, Default)]
pub struct Config {
    default_password_requirements: PasswordRequirements,
    authcore_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            authcore_url: std::env::var("AUTHCORE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            ..Default::default()
        }
    }

    pub fn set_default_password_requirements(&mut self, requirements: PasswordRequirements) {
        self.default_password_requirements = requirements;
    }

    pub fn default_password_requirements(&self) -> PasswordRequirements {
        self.default_password_requirements
    }

    pub fn authcore_url(&self) -> &str {
        self.authcore_url.as_ref()
    }
}
