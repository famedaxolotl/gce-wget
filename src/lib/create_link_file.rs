use std::fs::File;
use std::io;
use std::env;
use std::io::Write;

pub fn create_link_file(url: &String, years: &[String]) -> Result<String, io::Error> {
    let link_file_path = format!("{}/Downloads/.temp-link-file", env::var("HOME").expect("Home env var not found"));

    let mut link_file = File::create(&link_file_path)?;

    for year in years {
        writeln!(link_file, "{}", format!("{}{}", url, year))?;
    };

    Ok(link_file_path)
}