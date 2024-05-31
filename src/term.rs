use dialoguer::{theme::ColorfulTheme, Input};

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

    pub fn ask(msg: &str) -> String {
        let response = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(msg)
            .interact_text()
            .unwrap();
        response
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
