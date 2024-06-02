use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;

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
    pub fn get_config_dir() -> String {
        home_dir()
            .unwrap()
            .join(".config")
            .join("resup")
            .display()
            .to_string()
    }

    pub fn get_config_path() -> String {
        home_dir()
            .unwrap()
            .join(".config")
            .join("resup")
            .join("config.toml")
            .display()
            .to_string()
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
