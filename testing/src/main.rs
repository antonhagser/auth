use command_handler::CommandHandler;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::command_handler::QuitCode;

pub mod command_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rl = DefaultEditor::new()?;
    let mut handler = CommandHandler::new();

    // Load history
    let _ = rl.load_history("history.txt");

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                // Find command
                let command = line.split_once(' ');
                let (cmd, args) = if let Some((cmd, args)) = command {
                    (cmd, args)
                } else if !line.is_empty() {
                    (line.as_str(), "")
                } else {
                    println!("No command found");
                    continue;
                };

                let quitcode = handler.handle(cmd.into(), args.into());
                if let QuitCode::Failure = quitcode {
                    println!("Critical error reached, quitting.");
                    break;
                } else {
                    let _ = rl.add_history_entry(line.as_str());
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // Store history
    rl.save_history("history.txt")?;

    Ok(())
}
