use std::{
    process::{Command, Stdio},
    vec,
};

use crate::term::Term;

pub fn upscale(
    input: String,
    output: String,
    model: String,
    executable: String,
    models: String,
) -> bool {
    let mut esrgan = Command::new(executable);
    let mut args: Vec<&str> = vec![];
    args.append(&mut vec![
        "-i", &input, "-o", &output, "-s", "4", "-n", &model,
    ]);
    if !models.is_empty() {
        args.append(&mut vec!["-m", &models]);
    }
    esrgan.args(args);
    esrgan.stdout(Stdio::inherit());
    esrgan.stdin(Stdio::inherit());
    esrgan.stderr(Stdio::inherit());
    match esrgan.output() {
        Ok(_) => {
            Term::info("ESRGAN finished his job successfully");
            return true;
        },
        Err(e) => {
            Term::fatal(&format!("ESRGAN encountered an error: {}", e));
            return false;
        }
    }
}
