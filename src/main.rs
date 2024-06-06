use simple_home_dir::home_dir;
use std::{
    fmt::format,
    fs::{self, File},
    io::Write,
};
use time::{macros::date, OffsetDateTime};

fn main() -> anyhow::Result<()> {
    let mut home_path = home_dir().expect("No homedir found");

    home_path.push("zlogz");

    println!("{:?}", &home_path);

    match fs::create_dir(&home_path) {
        Ok(_) => {
            println!("zlogz directory created");
        }
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            println!("zlogz directory already exists");
        }
        Err(e) => {
            println!("Failed to create zlogz directory: {}", e);
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
