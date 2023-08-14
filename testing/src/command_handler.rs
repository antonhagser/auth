use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuitCode {
    #[error("Command ran successfully")]
    Success,
    #[error("Command encountered an error")]
    Failure,
}

pub struct CommandHandler {}

impl CommandHandler {
    pub fn new() -> Self {
        CommandHandler {}
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

    pub fn handle(&mut self, cmd: String, arguments: String) -> QuitCode {
        // Parse arguments, argument should be split into parts
        // The following rules apply:
        //  - Arguments are split by spaces
        //  - Arguments can be quoted with double quotes
        //  - Arguments can be escaped with a backslash
        //  - Commands can have variables which are set with -variable_name=value
        let (args, variables) = match Self::parse_arguments(&arguments) {
            Ok((args, variables)) => (args, variables),
            Err(err) => {
                println!("Error parsing arguments: {}", err);
                return QuitCode::Failure;
            }
        };

        println!("Arguments: {:?}", args);
        println!("Variables: {:?}", variables);

        match Command::from(cmd) {
            Command::Help => {
                println!("Available commands:");
                for cmd in Command::iter() {
                    if cmd == Command::Unknown {
                        continue;
                    }

                    let cmd = cmd.as_ref().to_lowercase();
                    println!("\t{}", cmd);
                }
            }
            Command::Login => todo!(),
            Command::Register => todo!(),
            Command::Unknown => {
                println!("Unknown command");
                return QuitCode::Failure;
            }
            Command::Quit => return QuitCode::Failure,
        }

        QuitCode::Success
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
