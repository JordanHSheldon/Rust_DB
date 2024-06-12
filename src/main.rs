use std::io::stdin;
use std::io::Write;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    loop {
        // print the shell interface so the user knows the program is ready for input.
        print!("db >");
        std::io::stdout().flush().unwrap(); // print stores but does not flush like pribntln
        let mut buffer: String = String::new();
        stdin().read_line(&mut buffer)?;
        print!("You said {}\n", buffer);
    }
}
