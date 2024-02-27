use std::process::{Command, Stdio};

use crate::config::Config;

pub enum UpscaleError {
    ExecutableNotFound,
    ProcessInterrupted,
    ModelsDirectoryNotFound,
    ModelParamNotFound,
    ModelBinNotFound,
    UnknownError,
}

pub fn run_upscale(
    config: Config,
    input: &str,
    output: &str,
    quite: bool,
) -> Result<(), UpscaleError> {
    let mut proc: Command = Command::new(&config.executable);
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
    if !quite {
        proc.stdout(Stdio::inherit());
        proc.stdin(Stdio::inherit());
        proc.stderr(Stdio::inherit());
    } else {
        proc.stdout(Stdio::piped());
        proc.stdin(Stdio::piped());
        proc.stderr(Stdio::piped());
    }
    if !config.check_executable_exists() {
        return Err(UpscaleError::ExecutableNotFound);
    }
    if !config.check_models_path_exists() {
        return Err(UpscaleError::ModelsDirectoryNotFound);
    }
    if !config.check_model_bin_exists() {
        return Err(UpscaleError::ModelBinNotFound);
    }
    if !config.check_model_param_exists() {
        return Err(UpscaleError::ModelParamNotFound);
    }
    let result = proc.output();
    match result {
        Ok(_) => Ok(()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Err(UpscaleError::ExecutableNotFound),
            std::io::ErrorKind::Interrupted => Err(UpscaleError::ProcessInterrupted),
            _ => Err(UpscaleError::UnknownError),
        },
    }
}
