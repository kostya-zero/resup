use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    model: String,
    models_path: String,
    executable: String,
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
    pub fn get_model(&self) -> String {
        self.model.to_string()
    }
    
    pub fn set_model(&mut self, model_name: &str) {
        self.model = String::from(model_name);
    }

    pub fn get_models_path(&self) -> String {
        self.models_path.to_string()
    }
    
    pub fn set_models_path(&mut self, models_path: &str) {
        self.models_path = String::from(models_path);
    }

    pub fn get_executable_path(&self) -> String {
        self.executable.to_string()
    }
    
    pub fn set_executable_path(&mut self, executable: &str) {
        self.executable = String::from(executable);
    }

    pub fn check_model_param_exists(&self) -> bool {
        let model_path = self.get_models_path() + "/" + &self.get_model() + ".param";
        Path::new(&model_path).exists()
    }

    pub fn check_model_bin_exists(&self) -> bool {
        let model_path = self.get_models_path() + "/" + &self.get_model() + ".bin";
        Path::new(&model_path).exists()
    }

    pub fn check_models_path_exists(&self) -> bool {
        Path::new(&self.get_models_path()).exists()
    }

    pub fn check_executable_exists(&self) -> bool {
        Path::new(&self.get_executable_path()).exists()
    }
}

pub struct Manager;
impl Manager {
    pub fn get_config_dir() -> String {
        home_dir().expect("Fail").display().to_string() + "/.config/resup"
    }

    pub fn get_config_path() -> String {
        home_dir().expect("Fail").display().to_string() + "/.config/resup/config.toml"
    }

    pub fn make_default() {
        let default_config: Config = Config::default();
        let toml_config: String =
            toml::to_string(&default_config).expect("Failed to format config.");
        let resup_path = Self::get_config_dir();
        if !Path::new(&resup_path).exists() {
            fs::create_dir(&resup_path).expect("Failed to create directory.");
        }

        let config_path =
            home_dir().expect("Fail").display().to_string() + "/.config/resup/config.toml";
        fs::write(config_path, toml_config).expect("Failed to write config data.");
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
