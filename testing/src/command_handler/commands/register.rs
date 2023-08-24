use std::collections::HashMap;

use tracing::{error, info};

use crate::command_handler::{
    command::{Command, CommandOwner, CommandResult},
    context::Context,
};

pub struct RegisterCommandOwner {}

impl RegisterCommandOwner {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RegisterCommandOwner {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandOwner for RegisterCommandOwner {
    fn get_command_name(&self) -> &'static str {
        "register"
    }

    fn get_command_description(&self) -> &'static str {
        "Register to an account."
    }

    fn get_command_usage(&self) -> &'static str {
        "register <email> <password> <application_id>"
    }

    fn parse(
        &self,
        args: Vec<String>,
        _variables: HashMap<String, String>,
    ) -> Option<Box<dyn Command>> {
        if args.len() != 3 {
            return None;
        }

        let email = args[0].clone();
        let password = args[1].clone();
        let application_id = args[2].clone();

        Some(Box::new(RegisterCommand {
            email,
            password,
            application_id,
        }))
    }
}

#[derive(Default)]
pub struct RegisterCommand {
    pub email: String,
    pub password: String,
    pub application_id: String,
}

impl Command for RegisterCommand {
    fn assign_additional_context(
        &mut self,
        _args: Vec<String>,
        _variables: HashMap<String, String>,
        id: &'static str,
    ) {
        #[allow(clippy::match_single_binding)]
        match id {
            _ => panic!("Invalid context ID."),
        }
    }

    fn execute(&self, _ctx: Context) -> CommandResult {
        println!("Register command executing...");

        let client = reqwest::blocking::Client::new();
        let response = client
            .post("http://localhost:8080/basic/register")
            .json(&serde_json::json!({
                "email": self.email,
                "password": self.password,
                "application_id": self.application_id,
            }))
            .send();

        if let Err(e) = response {
            error!(?e);
            return CommandResult::Failure;
        }

        let response = response.unwrap();
        if !response.status().is_success() {
            error!("could not register: {}", response.status());
            return CommandResult::Failure;
        }

        let response = response.json::<serde_json::Value>();
        if let Err(e) = response {
            error!(?e);
            return CommandResult::Failure;
        }

        let response = response.unwrap();
        info!(?response);

        CommandResult::Success
    }
}
