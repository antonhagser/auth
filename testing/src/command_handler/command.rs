use std::collections::HashMap;

use super::context::Context;

pub trait CommandOwner {
    fn get_command_name(&self) -> &'static str;
    fn get_command_description(&self) -> &'static str;
    fn get_command_usage(&self) -> &'static str;

    fn parse(
        &self,
        args: Vec<String>,
        variables: HashMap<String, String>,
    ) -> Option<Box<dyn Command>>;
}

pub trait Command {
    fn execute(&self, ctx: Context) -> CommandResult;
    fn assign_additional_context(
        &mut self,
        args: Vec<String>,
        variables: HashMap<String, String>,
        context_id: &'static str,
    );
}

#[derive(Debug)]
pub enum CommandResult {
    // Identifies that the command is not finished and needs more context, the user has to answer some more questions
    MoreContextNeeded(&'static str),

    // Identifies that the command is finished and the user can continue with the next command
    Success,

    // Identifies that the command failed and the user has to start over
    Failure,
}
