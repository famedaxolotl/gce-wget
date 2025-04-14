use std::{fs::File, env, io::Write};

pub fn create_link_file(url: &String, years: &[String]) -> Result<String, Box<dyn std::error::Error>> {
    let link_file_path = format!("{}/Downloads/.temp-link-file", env::var("HOME")
        .map_err(|e| format!("Error: {}", e))?);

    let mut link_file = File::create(&link_file_path)?;

    for year in years {
        writeln!(link_file, "{}{}", url, year)?;
    };

    Ok(link_file_path)
}