use command_handler::CommandHandler;
use rustyline::{error::ReadlineError, DefaultEditor};
use tracing::{error, info};

use crate::command_handler::QuitCode;

pub mod command_handler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_target(false).init();
    info!("Testing CLI");

    let mut rl = DefaultEditor::new()?;
    let mut handler = CommandHandler::new();

    // Load history
    let _ = rl.load_history("history.txt");

    // Make user press CTRL+C twice
    let mut ctrl_c_count = 0;
    let mut ctrl_last_time = None;

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
                    continue;
                };

                // Handle with command handler
                let quitcode = handler.handle(cmd.into(), args.into(), line.clone());
                match quitcode {
                    QuitCode::Failure => {
                        continue;
                    }
                    QuitCode::Success => {
                        let last = rl.history().iter().last();
                        if last != Some(&line) {
                            let _ = rl.add_history_entry(line.as_str());
                        }
                    }
                    QuitCode::MoreContextNeeded => {
                        continue;
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                if ctrl_c_count == 0 {
                    ctrl_c_count += 1;
                    ctrl_last_time = Some(std::time::Instant::now());
                    println!("(To exit, press ^C again)");
                    continue;
                } else if ctrl_c_count == 1 {
                    if let Some(last_time) = ctrl_last_time {
                        if last_time.elapsed().as_secs() < 2 {
                            break;
                        } else {
                            ctrl_c_count = 0;
                            ctrl_last_time = None;
                            continue;
                        }
                    }
                }
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                error!(?err);
                break;
            }
        }
    }

    // Store history
    rl.save_history("history.txt")?;

    Ok(())
}
