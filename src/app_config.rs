use std::{collections::HashMap, path::PathBuf};

use config::Config;
use log::info;
use simple_home_dir::home_dir;

pub struct AppConfig {
    pub home_path: PathBuf,
    pub editor_command: String,
}

pub fn read_config() -> AppConfig {
    let mut config_path = home_dir().expect("No homedir found");
    config_path.push(".config/.zlogz");
    info!("config{:?}", config_path);

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build();

    let mut home_path = PathBuf::new();
    let mut editor_command = "nvim".to_string();

    if let Ok(settings) = settings {
        let config_map: HashMap<String, String> = settings
            .try_deserialize()
            .expect("Failed to deserialize settings");

        home_path = set_path(&config_map);
        editor_command = set_editor_command(&config_map);
    } else if let Err(e) = settings {
        info!("Settings don't exist, using default. Error: {}", e);
        home_path = home_dir().expect("No homedir found");
        home_path.push("zlogz");
    }

    info!("zlogz directory set {:?}", home_path);
    info!("zlogz editor set {:?}", editor_command);

    AppConfig {
        home_path,
        editor_command,
    }
}

fn set_path(settings: &HashMap<String, String>) -> PathBuf {
    let mut home_path = PathBuf::new();

    let path = settings.get("path");

    match path {
        Some(path) => home_path.push(path),
        None => {
            home_path = home_dir().expect("No homedir found");
            home_path.push("zlogz");
        }
    }

    home_path
}

fn set_editor_command(settings: &HashMap<String, String>) -> String {
    let editor = settings.get("editor");

    match editor {
        Some(editor) => editor.to_string(),
        None => "nvim".to_string(),
    }
}
