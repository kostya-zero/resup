use std::io::{self, Write};

pub struct Term;
impl Term {
    pub fn message(msg: &str) {
        println!(" 󰍡 {}", msg);
    }

    pub fn title(msg: &str) {
        println!("\x1b[1m 󰍡 {}\x1b[0m", msg);
    }

    pub fn no_icon_message(msg: &str) {
        println!("   {}", msg);
    }

    pub fn display_data(name: &str, data: &str) {
        println!("\x1b[1m 󰆼 {}:\x1b[0m {}", name, data);
    }

    pub fn ask(msg: &str, default: &str) -> String {
        print!(" 󰍡 {} \x1b[90m({})\x1b[0m: ", msg, default);
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.is_empty() {
            return default.to_string();
        }
        input.strip_suffix("\r\n").unwrap().to_string()
    }

    pub fn error(msg: &str) {
        println!("\x1b[91m  {}\x1b[0m", msg);
    }

    pub fn done(msg: &str) {
        println!("\x1b[92m  {}\x1b[0m", msg);
    }

    pub fn warn(msg: &str) {
        println!("\x1b[93m  {}\x1b[0m", msg);
    }
}
