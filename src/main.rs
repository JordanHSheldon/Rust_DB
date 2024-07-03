use std::io::stdin;
use std::io::Write;
use std::process::exit;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    loop {
        // print the shell interface so the user knows the program is ready for input.
        print!("db >");
        std::io::stdout().flush().unwrap();
        let mut buffer: String = String::new();
        stdin().read_line(&mut buffer)?;

        if let Some(is_meta) = buffer.chars().next() {
            // meta command
            match is_meta {
                '.' => {
                    // meta command
                    print!("Performing meta command.");
                    meta_commands(buffer.trim());
                    exit(0);
                }
                _ => {
                    // db command.
                    println!("{}",buffer);
                    println!("Unreckognized command.")
                }
            }
        } else {
            // db commands
            match buffer.trim().to_lowercase().as_str() {
                "select" => {
                    // meta command
                    print!("Performing meta command.");
                    meta_commands(buffer.trim());
                    exit(0);
                }
                _ => {
                    // db command.
                    println!("{}",buffer);
                    println!("Unreckognized command.")
                }
            }
        }
        
    }
}

fn meta_commands(command: &str) -> u32 {
    match command {
        ".exit" => {
            exit(0);
        },
        _ => {
            println!("Unreckognized meta command.");
            return 1;
        }
    }
}