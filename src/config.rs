use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub model: String,
    pub models_path: String,
    pub executable: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            model: String::from("realesrgan-x4plus"),
            models_path: String::from("/usr/share/realesrgan-ncnn-vulkan/models"),
            executable: String::from("/usr/bin/realesrgan-ncnn-vulkan"),
        }
    }
}

impl Config {

    pub fn check_model_param_exists(&self) -> bool {
        Path::new(&self.models_path).join(self.model.clone() + ".param").exists()
    }

    pub fn check_model_bin_exists(&self) -> bool {
        Path::new(&self.models_path).join(self.model.clone() + ".bin").exists()
    }

    pub fn check_models_path_exists(&self) -> bool {
        Path::new(&self.models_path).exists()
    }

    pub fn check_executable_exists(&self) -> bool {
        Path::new(&self.executable).exists()
    }
}

pub struct Manager;
impl Manager {
    pub fn get_config_dir() -> String {
        home_dir().unwrap().join(".config").join("resup").display().to_string()
    }

    pub fn get_config_path() -> String {
        home_dir().unwrap().join(".config").join("resup").join("config.toml").display().to_string()
    }

    pub fn exists() -> bool {
        Path::new(&Self::get_config_path()).exists()
    }

    pub fn load() -> Config {
        let config_string: String =
            fs::read_to_string(Self::get_config_path()).expect("Failed to read config.");
        let config_struct: Config =
            toml::from_str(&config_string).expect("Failed to format config.");
        config_struct
    }

    pub fn write(config: Config) {
        let config_string: String = toml::to_string(&config).expect("Failed to format config.");
        fs::write(Self::get_config_path(), config_string)
            .expect("Failed to write content to file.");
    }
}
