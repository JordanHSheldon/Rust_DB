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
            match is_meta {
                '.' => {
                    // meta command
                    // prepare / parse first
                    // if okay then execute
                    print!("Performing meta command.");
                    parse_meta_commands();
                    meta_commands(buffer.to_lowercase().trim());
                }
                _ => {
                    // db command.
                    // prepare / parse first
                    // if okay then execute
                    parse_db_commands();
                    db_commands(buffer.to_lowercase().trim());
                }
            }
        
        }
    }
}

fn parse_meta_commands(){

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

fn parse_db_commands(){
    
}

fn db_commands(command: &str) -> u32 {
    match command {
        "select" => {
            // meta command
            print!("\nSelect not yet supported.");
        }
        "insert" => {
            // meta command
            print!("\nInsert not yet supported.");
        }
        "delete" => {
            // meta command
            print!("\ndelete not yet supported.");
        }
        "update" => {
            // meta command
            print!("\nUpdate not yet supported.");
        }
        _ => {
            print!("\nUnknown command.");
        }
    }

    return 1;
}