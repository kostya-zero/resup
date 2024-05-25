use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use indicatif::{ProgressBar, ProgressStyle};

use crate::config::Config;

pub enum UpscaleError {
    ExecutableNotFound,
    ProcessInterrupted,
    ModelFilesNotFound,
    UnknownError,
}

pub fn run_upscale(
    config: Config,
    input: &str,
    output: &str,
) -> Result<(), UpscaleError> {
    if !config.check_model() {
        return Err(UpscaleError::ModelFilesNotFound);
    }
    let mut proc = Command::new(config.executable);
    proc.stdout(Stdio::piped());
    proc.stdin(Stdio::piped());
    proc.stderr(Stdio::piped());
    proc.args(vec![
        "-i",
        input,
        "-o",
        output,
        "-s",
        "4",
        "-f",
        "png",
        "-m",
        &config.models_path,
        "-n",
        &config.model,
    ]);

    match proc.spawn() {
        Ok(status) => {
            let pb = ProgressBar::new(100);
            pb.set_style(ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.white/gray}] {pos}%").unwrap().progress_chars("##-"));
            let stdout = status.stderr.unwrap();
            let reader = BufReader::new(stdout);
            reader.lines().map_while(Result::ok).for_each(
                |line| {
                    if line.contains('%') {
                        let split = line.split('%').map(|i| i.to_string()).collect::<Vec<String>>();
                        let progress = split[0].clone();
                        let position: f32 = progress.parse().unwrap();
                        let integer_part: i16 = position.trunc() as i16;
                        pb.set_position(integer_part as u64);
                    }
                }
            );
            pb.finish_and_clear();
            Ok(())
        },
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Err(UpscaleError::ExecutableNotFound),
            std::io::ErrorKind::Interrupted => Err(UpscaleError::ProcessInterrupted),
            _ => Err(UpscaleError::UnknownError),
        },
    }
}
