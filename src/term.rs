pub struct Term;
impl Term {
    pub fn message(msg: &str) {
        println!("\x1b[1m 󰍡 {}\x1b[0m", msg);
    }

    pub fn display_data(name: &str, data: &str) {
        println!("\x1b[1m 󰆼 {}:\x1b[0m {}", name, data);
    }

    pub fn error(msg: &str) {
        println!("\x1b[1m\x1b[91m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn done(msg: &str) {
        println!("\x1b[1m\x1b[92m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }

    pub fn warn(msg: &str) {
        println!("\x1b[1m\x1b[93m \x1b[0m\x1b[1m {}\x1b[0m", msg);
    }
}
