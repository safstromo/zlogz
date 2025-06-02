mod app_config;

use app_config::{read_config, AppConfig};
use log::{error, info};
use simple_logger::SimpleLogger;
use std::{
    env::{self, args},
    fs::{self, File},
    io::Write,
};
use subprocess::Exec;
use time::OffsetDateTime;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut app_config = read_config();

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
        "t" => {
            info!("Opening todo");
            let _ = create_zlog(&mut app_config, Zlog::Todo);
            args_added = true;
        }

        _ => (),
    });

    if !args_added {
        let _ = create_zlog(&mut app_config, Zlog::Today);
    }

    Ok(())
}

#[derive(Debug)]
enum Zlog {
    Today,
    Todo,
}

fn create_zlog(app_config: &mut AppConfig, zlog: Zlog) -> anyhow::Result<()> {
    check_dir(app_config)?;

    let now = OffsetDateTime::now_local()?;
    let today = now.date();

    let file_name = match zlog {
        Zlog::Today => {
            format!("{}.md", &today)
        }
        Zlog::Todo => {
            let year = now.year();
            let month = now.month();

            format!("{}-{}-todo.md", year, month)
        }
    };

    app_config.home_path.push(file_name);

    let file = File::open(&app_config.home_path);

    match file {
        Ok(_) => {
            info!("Opening {:?}", zlog);
            let _ = Exec::cmd(&app_config.editor_command)
                .arg(&app_config.home_path)
                .join();

            info!("zlog {:?} opened", zlog)
        }
        Err(_) => {
            info!("A {:?} dont exist, creating new file", zlog);
            let mut file = File::create(&app_config.home_path)?;

            match zlog {
                Zlog::Today => file.write_all(format!("# {}", today).as_bytes())?,
                Zlog::Todo => file.write_all("# TODO:".as_bytes())?,
            }

            let _ = Exec::cmd(&app_config.editor_command)
                .arg(&app_config.home_path)
                .join();

            info!("zlog {:?} opened", zlog)
        }
    }

    Ok(())
}

fn check_dir(app_config: &mut AppConfig) -> anyhow::Result<()> {
    env::set_current_dir(&app_config.home_path)?;
    app_config
        .home_path
        .push(OffsetDateTime::now_local()?.year().to_string());
    app_config
        .home_path
        .push(OffsetDateTime::now_local()?.month().to_string());

    match fs::create_dir_all(&app_config.home_path) {
        Ok(_) => {
            info!("zlogz directory created");
            Ok(())
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            info!("zlogz directory already exists, using existing directory");
            Ok(())
        }
        Err(e) => {
            error!("Failed to create zlogz directory: {}", e);
            Err(e.into())
        }
    }
}
