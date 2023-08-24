use std::collections::HashMap;

use crate::command_handler::{
    command::{Command, CommandOwner, CommandResult},
    context::Context,
};

pub struct TOTPCommandOwner {}

impl TOTPCommandOwner {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TOTPCommandOwner {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandOwner for TOTPCommandOwner {
    fn get_command_name(&self) -> &'static str {
        "totp"
    }

    fn get_command_description(&self) -> &'static str {
        "Get the code for totp."
    }

    fn get_command_usage(&self) -> &'static str {
        "totp <secret>"
    }

    fn parse(
        &self,
        args: Vec<String>,
        _variables: HashMap<String, String>,
    ) -> Option<Box<dyn Command>> {
        if args.len() != 1 {
            return None;
        }

        let secret = args[0].clone();

        Some(Box::new(TOTPCommand { secret }))
    }
}

#[derive(Default)]
pub struct TOTPCommand {
    pub secret: String,
}

impl Command for TOTPCommand {
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
        let code = crypto::totp::generate_totp(self.secret.as_bytes(), 30).unwrap();
        println!("TOTP code: {}", code);

        CommandResult::Success
    }
}
