use simple_home_dir::home_dir;
use std::fs;

fn main() -> std::io::Result<()> {
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
            return Err(e);
        }
    }

    Ok(())
}
