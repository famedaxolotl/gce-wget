use std::fs::File;
use std::io;
use std::env;
use std::io::Write;

pub fn create_link_file(url: &String, years: &[String]) -> Result<(), io::Error> {
    let usr_downloads_dir = format!("{}/Downloads", env::var("HOME").expect("Home env var not found"));

    let mut link_file = File::create(format!("{}/.temp-link-file", usr_downloads_dir)).expect("Failed to create necessary link file");

    for year in years {
        writeln!(link_file, "{}", format!("{}{}", url, year)).expect("Could not create link file");
    };

    Ok(())
}