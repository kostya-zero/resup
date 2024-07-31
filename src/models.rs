use std::{fs, io, path::Path};

pub struct ModelsContainer {
    models: Vec<String>,
    bad_models: Vec<String>,
}

impl ModelsContainer {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let path_buf = Path::new(path);
        let files = fs::read_dir(path_buf)?;

        let mut new_models: Vec<String> = Vec::new();
        let mut new_bad_models: Vec<String> = Vec::new();

        for entry in files {
            let file = entry?.path();
            let name = file
                .file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or_default()
                .to_string();

            if file.extension().and_then(|ext| ext.to_str()) == Some("pth") {
                new_models.push(name);
                continue;
            }

            if new_models.iter().any(|m| *m == name) {
                continue;
            }

            let bin_path = file.with_extension("bin");
            let param_path = file.with_extension("param");

            if bin_path.exists() && param_path.exists() {
                new_models.push(name);
            } else if !new_bad_models.contains(&name) {
                new_bad_models.push(name);
            }
        }

        Ok(Self {
            models: new_models,
            bad_models: new_bad_models,
        })
    }

    pub fn get_models(&self) -> Vec<String> {
        self.models.clone()
    }

    pub fn get_bad_models(&self) -> Vec<String> {
        self.bad_models.clone()
    }
}
