use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct Model {
    pub name: String,
    pub bin_path: PathBuf,
    pub param_path: PathBuf,
}

pub struct ModelsContainer {
    pub models: Vec<Model>,
    pub bad_models: Vec<String>,
}
impl ModelsContainer {
    pub fn new(path: &str) -> Self {
        let path_buf = Path::new(path);
        let files = fs::read_dir(path_buf).unwrap();
        let mut new_models: Vec<Model> = Vec::new();
        let mut new_bad_models: Vec<String> = Vec::new();
        for file in files {
            let file = file.unwrap().path();
            let new_path = path_buf.join(file);
            let name = new_path.file_stem().unwrap().to_str().unwrap();
            if new_models.iter().any(|m| m.name == name) {
                continue;
            }

            let new_bin_path = new_path.with_extension("bin");
            let new_param_path = new_path.with_extension("param");
            if !new_bin_path.exists() || !new_param_path.exists() {
                if !new_bad_models.contains(&name.to_string()) {
                    new_bad_models.push(name.to_string());
                    continue;
                }
                continue;
            }

            new_models.push(Model {
                name: name.to_string(),
                bin_path: new_bin_path,
                param_path: new_param_path,
            });
        }

        Self {
            models: new_models,
            bad_models: new_bad_models,
        }
    }
}
