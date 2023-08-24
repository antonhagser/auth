use std::collections::HashMap;

use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};
use thiserror::Error;
use tracing::{error, info, warn};

use crate::command_handler::command::CommandResult;

use self::context::Context;

pub mod command;
pub mod commands;
pub mod context;

#[derive(Error, Debug)]
pub enum QuitCode {
    #[error("Command ran successfully")]
    Success,
    #[error("Command encountered an error")]
    Failure,

    #[error("Command needs more context")]
    MoreContextNeeded,
}

pub struct CommandHandler {
    context: Context,
    handler: Option<Box<dyn command::Command>>,
    more_context_id: Option<&'static str>,
}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {
            context: Context::new(),
            handler: None,
            more_context_id: None,
        }
    }

    pub fn handle(&mut self, cmd: String, arguments: String, line: String) -> QuitCode {
        // Parse arguments, argument should be split into parts
        // The following rules apply:
        //  - Arguments are split by spaces
        //  - Arguments can be quoted with double quotes
        //  - Arguments can be escaped with a backslash
        //  - Commands can have variables which are set with -variable_name=value
        let parseable = if self.handler.is_some() {
            line
        } else {
            arguments
        };

        let (args, variables) = match parse_arguments(&parseable) {
            Ok((args, variables)) => (args, variables),
            Err(err) => {
                error!("error parsing arguments: {}", err);
                return QuitCode::Failure;
            }
        };

        if let (Some(handler), Some(more_context_id)) =
            (self.handler.take(), self.more_context_id.take())
        {
            return self.execute_command(cmd, args, variables, Some((handler, more_context_id)));
        }

        match Command::from(cmd.clone()) {
            Command::Help => {
                info!("Available commands:");
                for cmd in Command::iter() {
                    if cmd == Command::Unknown {
                        continue;
                    }

                    let cmd = cmd.as_ref().to_lowercase();
                    info!("\t{}: fix later", cmd);
                }

                QuitCode::Success
            }
            Command::Unknown => {
                warn!("Unknown command");
                QuitCode::Failure
            }
            Command::Quit => QuitCode::Failure,
            _ => self.execute_command(cmd, args, variables, None),
        }
    }

    fn execute_command(
        &mut self,
        cmd: String,
        args: Vec<String>,
        variables: HashMap<String, String>,
        mut handler: Option<(Box<dyn command::Command>, &'static str)>,
    ) -> QuitCode {
        // If command has an active handler, continue with it
        // else try to parse the command
        let handler = if let Some((mut handler, context_id)) = handler.take() {
            handler.assign_additional_context(args, variables, context_id);
            handler
        } else {
            // Find the command in the command list
            let mut commands = self.context.commands().borrow_mut();
            let command = match commands.get_mut(&cmd) {
                Some(command) => command,
                None => {
                    println!("Unknown command");
                    return QuitCode::Failure;
                }
            };

            if let Some(cmd) = command.parse(args, variables) {
                cmd
            } else {
                println!("Error parsing command");
                return QuitCode::Failure;
            }
        };

        // Execute the command handler
        match handler.execute(self.context.clone()) {
            CommandResult::MoreContextNeeded(id) => {
                // If the command needs more context, set the handler
                self.handler = Some(handler);
                self.more_context_id = Some(id);
                QuitCode::MoreContextNeeded
            }
            CommandResult::Success => QuitCode::Success,
            CommandResult::Failure => QuitCode::Failure,
        }
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, EnumIter, AsRefStr, PartialEq)]
pub enum Command {
    Help,
    Login,
    Register,
    Quit,
    Unknown,
}

impl From<String> for Command {
    fn from(cmd: String) -> Self {
        match &*cmd {
            "help" => Command::Help,
            "login" => Command::Login,
            "register" => Command::Register,
            "quit" => Command::Quit,
            _ => Command::Unknown,
        }
    }
}

fn parse_arguments(
    arguments: &str,
) -> Result<(Vec<String>, std::collections::HashMap<String, String>), QuitCode> {
    let mut args = Vec::new();
    let mut variables = std::collections::HashMap::new();

    let mut in_quotes = false;
    let mut escape_next = false;
    let mut current_arg = String::new();

    for ch in arguments.chars() {
        if escape_next {
            current_arg.push(ch);
            escape_next = false;
        } else {
            match ch {
                '\\' => escape_next = true,
                '"' => in_quotes = !in_quotes,
                ' ' if !in_quotes => {
                    if !current_arg.is_empty() {
                        if current_arg.starts_with('-') {
                            let parts: Vec<&str> = current_arg.splitn(2, '=').collect();
                            if parts.len() == 2 {
                                variables.insert(
                                    parts[0].to_string().replace('-', ""),
                                    parts[1].to_string(),
                                );
                            } else {
                                // Handle error: variable assignment is malformed
                                return Err(QuitCode::Failure);
                            }
                        } else {
                            args.push(current_arg.clone());
                        }
                        current_arg.clear();
                    }
                }
                _ => current_arg.push(ch),
            }
        }
    }

    if !current_arg.is_empty() {
        if current_arg.starts_with('-') {
            let parts: Vec<&str> = current_arg.splitn(2, '=').collect();
            if parts.len() == 2 {
                variables.insert(parts[0].to_string().replace('-', ""), parts[1].to_string());
            } else {
                // Handle error: variable assignment is malformed
                return Err(QuitCode::Failure);
            }
        } else {
            args.push(current_arg);
        }
    }

    if in_quotes {
        // Handle error: unmatched quote in arguments
        return Err(QuitCode::Failure);
    }

    Ok((args, variables))
}
