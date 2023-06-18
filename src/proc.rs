use std::process::{Command, Stdio};

pub struct Proc;
impl Proc {
    pub fn upscale(input: String, output: String, model: String, executable: String) -> bool {
        let model_name = match model.as_str() {
            "anime" => "realesrgan-x4plus-anime",
            "photo" => "realesrgan-x4plus",
            _ => panic!("Unknown model type passed!")

        };

        let status = Command::new(executable)
            //.args([format!("-i {} -o {} -s {} -m {}", input, output, scale.to_string(), model_name)])
            .args(["-i", input.as_str(), "-o", output.as_str(), "-s", "4", "-n", model_name])
            .stdout(Stdio::inherit())
            .stdin(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("Faield to start realesrgan-ncnn-vulkan. Check if binary exists in PATH.");

        if status.status.success() {
            return true;
        }

        false
    }
    
}
