mod app_config;

use app_config::{read_config, AppConfig};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::{
    env::args,
    fs::{self, File},
    io::Write,
};
use subprocess::Exec;
use time::OffsetDateTime;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let app_config = read_config();

    let mut args_added = false;

    args().for_each(|arg| match arg.to_lowercase().as_str() {
        "f" => {
            info!("Opening search files");

            if app_config.editor_command == "nvim" {
                let _ = Exec::cmd("nvim")
                    .arg(&app_config.home_path)
                    .arg("+lua require('telescope.builtin').find_files()")
                    .join();
            } else {
                info!("No search for other editors yet")
            }
            args_added = true;
        }

        "s" => {
            info!("Opening search all");

            if app_config.editor_command == "nvim" {
                let _ = Exec::cmd("nvim")
                    .arg(&app_config.home_path)
                    .arg("+lua require('telescope.builtin').live_grep()")
                    .join();
            } else {
                info!("No search for other editors yet")
            }
            args_added = true;
        }

        _ => (),
    });

    if !args_added {
        create_log(app_config)?;
    }

    Ok(())
}

fn create_log(mut app_config: AppConfig) -> anyhow::Result<()> {
    app_config
        .home_path
        .push(OffsetDateTime::now_local()?.year().to_string());
    app_config
        .home_path
        .push(OffsetDateTime::now_local()?.month().to_string());

    match fs::create_dir_all(&app_config.home_path) {
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

    app_config.home_path.push(file_name);

    let file = File::open(&app_config.home_path);

    match file {
        Ok(_) => {
            info!("Opening todays zlogz");
            let _ = Exec::cmd(app_config.editor_command)
                .arg(&app_config.home_path)
                .join();

            info!("zlog opened")
        }
        Err(_) => {
            info!("A file with todays date dont exist, creating new file");
            let mut file = File::create(&app_config.home_path)?;

            file.write_all(format!("# {}", today).as_bytes())?;

            let _ = Exec::cmd(app_config.editor_command)
                .arg(&app_config.home_path)
                .join();

            info!("zlog opened")
        }
    }

    Ok(())
}
