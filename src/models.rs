use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub struct Model {
    pub name: String,
    pub is_pth: bool,
    pub bin_path: Option<PathBuf>,
    pub param_path: Option<PathBuf>,
}

pub struct ModelsContainer {
    pub models: Vec<Model>,
    pub bad_models: Vec<String>,
}

impl ModelsContainer {
    pub fn new(path: &str) -> Result<Self, io::Error> {
        let path_buf = Path::new(path);
        let files = fs::read_dir(path_buf)?;

        let mut new_models: Vec<Model> = Vec::new();
        let mut new_bad_models: Vec<String> = Vec::new();

        for entry in files {
            let file = entry?.path();
            let name = file
                .file_stem()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or_default()
                .to_string();

            if file.extension().and_then(|ext| ext.to_str()) == Some("pth") {
                new_models.push(Model {
                    name,
                    is_pth: true,
                    bin_path: None,
                    param_path: None,
                });
                continue;
            }

            if new_models.iter().any(|m| m.name == name) {
                continue;
            }

            let bin_path = file.with_extension("bin");
            let param_path = file.with_extension("param");

            if bin_path.exists() && param_path.exists() {
                new_models.push(Model {
                    name,
                    is_pth: false,
                    bin_path: Some(bin_path),
                    param_path: Some(param_path),
                });
            } else if !new_bad_models.contains(&name) {
                new_bad_models.push(name);
            }
        }

        Ok(Self {
            models: new_models,
            bad_models: new_bad_models,
        })
    }
}
