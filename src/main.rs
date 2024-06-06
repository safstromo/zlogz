use simple_home_dir::home_dir;
use simple_logger::SimpleLogger;
use std::{
    fs::{self, File},
    io::Write,
};
use time::OffsetDateTime;

fn main() -> anyhow::Result<()> {
    SimpleLogger::new().init().unwrap();

    let mut home_path = home_dir().expect("No homedir found");

    home_path.push("zlogz");

    match fs::create_dir(&home_path) {
        Ok(_) => {
            log::info!("zlogz directory created");
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            log::info!("zlogz directory already exists, using existing directory");
        }
        Err(e) => {
            log::error!("Failed to create zlogz directory: {}", e);
            return Err(e.into());
        }
    }

    let today = OffsetDateTime::now_local()?.date();

    let file_name = format!("{}.md", &today);

    home_path.push(file_name);

    let mut file = File::create(&home_path)?;

    file.write_all(format!("# {}", today).as_bytes())?;

    Ok(())
}
