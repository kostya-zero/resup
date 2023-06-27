use std::{
    process::{exit, Command, Stdio},
    vec,
};

use crate::term::Term;

pub struct Proc;
impl Proc {
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
        let result = esrgan.output();
        if result.is_err() {
            Term::fatal("Failed to launch real-esrgan executable. Check if it exists in PATH or by given path.");
            exit(1);
        }

        let status = result.unwrap();
        if status.status.success() {
            return true;
        }

        false
    }
}
