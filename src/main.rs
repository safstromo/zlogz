use config::Config;
use log::{error, info};
use simple_home_dir::home_dir;
use simple_logger::SimpleLogger;
use std::{
    collections::HashMap,
    env::args,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};
use subprocess::Exec;
use time::OffsetDateTime;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let home_path = read_config();

    let mut args_added = false;

    args().for_each(|arg| {
        if arg.to_lowercase() == "s" {
            info!("Opening search");
            let _ = Exec::cmd("nvim")
                .arg(&home_path)
                .arg("+lua require('telescope.builtin').find_files()")
                .join();
            args_added = true;
        }
    });

    if !args_added {
        create_log(home_path)?;
    }

    Ok(())
}

fn read_config() -> PathBuf {
    let mut config_path = home_dir().expect("No homedir found");
    config_path.push(".config/.zlogz");
    info!("config{:?}", config_path);

    let settings = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build();

    let mut home_path = PathBuf::new();

    if let Ok(settings) = settings {
        let config_map: HashMap<String, String> = settings
            .try_deserialize()
            .expect("Failed to deserialize settings");

        home_path.push(config_map.get("path").unwrap())
    } else if let Err(e) = settings {
        info!("Settings don't exist, using default. Error: {}", e);
        home_path = home_dir().expect("No homedir found");

        home_path.push("zlogz");
    }

    info!("zlogz directory set {:?}", home_path);

    home_path
}

fn create_log(mut home_path: std::path::PathBuf) -> anyhow::Result<()> {
    home_path.push(OffsetDateTime::now_local()?.year().to_string());
    home_path.push(OffsetDateTime::now_local()?.month().to_string());

    match fs::create_dir_all(&home_path) {
        Ok(_) => {
            info!("zlogz directory created");
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            info!("zlogz directory already exists, using existing directory");
        }
        Err(e) => {
            error!("Failed to create zlogz directory: {}", e);
            return Err(e.into());
        }
    }

    let today = OffsetDateTime::now_local()?.date();

    let file_name = format!("{}.md", &today);

    home_path.push(file_name);

    let file = File::open(&home_path);

    match file {
        Ok(_) => {
            info!("Opening todays zlogz");
            let _ = Exec::cmd("nvim").arg(&home_path).join();

            info!("zlog opened")
        }
        Err(_) => {
            info!("A file with todays date dont exist, creating new file");
            let mut file = File::create(&home_path)?;

            file.write_all(format!("# {}", today).as_bytes())?;

            let _ = Exec::cmd("nvim").arg(&home_path).join();

            info!("zlog opened")
        }
    }

    Ok(())
}
