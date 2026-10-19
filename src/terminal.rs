use dialoguer::{theme::ColorfulTheme, Input};

#[macro_export]
macro_rules! print_error {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        eprintln!(" {}: {}", "Error".bright_red().bold(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_done {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!(" {} {}", "✓".bold().bright_green(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_title {
    ($($arg:tt)*) => {{
        use ::colored::Colorize;
        println!("{}:", format!($($arg)*).bold());
    }};
}

pub fn ask(msg: &str) -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .interact_text()
        .unwrap()
}
