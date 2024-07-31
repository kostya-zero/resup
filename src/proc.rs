use crate::config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum UpscaleError {
    #[error("Failed to run executable file because it's not found.")]
    ExecutableNotFound,

    #[error("Process interrupted.")]
    ProcessInterrupted,

    #[error("Upscale failed for an unknown reason.")]
    UnknownError,
}

pub fn run_upscale(
    config: Config,
    input: &str,
    output: &str,
    verbose: bool,
) -> Result<(), UpscaleError> {
    let mut proc = Command::new(config.executable);
    if !verbose {
        proc.stdout(Stdio::piped());
        proc.stdin(Stdio::piped());
        proc.stderr(Stdio::piped());
    }

    proc.args(vec![
        "-i",
        input,
        "-o",
        output,
        // "-s",
        // "4",
        "-f",
        "png",
        "-m",
        &config.models_path,
        "-n",
        &config.model,
    ]);

    match proc.spawn() {
        Ok(status) => {
            if verbose {
                status.wait_with_output().unwrap();
            } else {
                let pb = ProgressBar::new(100);
                pb.set_style(
                    ProgressStyle::with_template(
                        "[{elapsed_precise}] [{wide_bar:.white/gray}] {pos}%",
                    )
                    .unwrap()
                    .progress_chars("##-"),
                );
                let stdout = status.stderr.unwrap();
                let reader = BufReader::new(stdout);
                reader.lines().map_while(Result::ok).for_each(|line| {
                    if line.contains('%') {
                        let split = line
                            .split('%')
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>();
                        let progress = split[0].clone();
                        let position: f32 = progress.parse().unwrap();
                        let integer_part: i16 = position.trunc() as i16;
                        pb.set_position(integer_part as u64);
                    }
                });
                pb.finish_and_clear();
            }
            Ok(())
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Err(UpscaleError::ExecutableNotFound),
            std::io::ErrorKind::Interrupted => Err(UpscaleError::ProcessInterrupted),
            _ => Err(UpscaleError::UnknownError),
        },
    }
}
