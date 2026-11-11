use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    loop {
        // Print prompt and ensure output is displayed immediately
        print!("> ");
        io::stdout().flush().unwrap();

        // Get input from the user
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // Parse the input into command and arguments
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        match command {
            "exit" => {
                return Ok(());
            }
            "cd" => {
                let new_dir = args.first().unwrap_or(&"/");
                let root = Path::new(new_dir);
                if let Err(error) = env::set_current_dir(root) {
                    eprintln!("cd: {}", error);
                }
            }
            command => {
                let mut cmd = Command::new(command);
                cmd.args(&args);

                match cmd.spawn() {
                    Ok(mut child) => {
                        // Wait for the child process to finish
                        match child.wait() {
                            Ok(status) => {
                                if !status.success() {
                                    eprintln!(
                                        "Command '{}' failed with exit code: {:?}",
                                        command,
                                        status.code()
                                    );
                                }
                            }
                            Err(error) => {
                                eprintln!("Failed to wait for command '{}': {}", command, error);
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("Failed to execute command '{}': {}", command, error);
                    }
                }
            }
        }
    }
}
