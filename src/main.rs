#![allow(special_module_name)]
mod lib;
use lib::{arg_parser, get_url, parse_regex, create_link_file};
use std::process::{self, Command};
use std::io;

fn main() {
    let args =arg_parser::arg_parser().unwrap_or_else(|error| {
        eprintln!("Invalid arguement provided: {}", error);
        process::exit(1);
    });
    
    println!("Downloading the following subject: {}", args.sub_code);
    println!("Of the following types: {:?}", args.types);
    println!("The following paper codes: {:?}", args.codes);
    println!("From the following years: {:?}", args.years);

    let regex = parse_regex::parse(&args);

    let url = get_url::get_url(&args.sub_code).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        process::exit(1);
    });

    let link_file = create_link_file::create_link_file(&url, &args.years).unwrap_or_else(|error| {
        eprintln!("Error creating link-file: {}", error);
        process::exit(1);

    });

    runner(&regex, &link_file).unwrap_or_else(|error| {
        eprintln!("Error: wget execution failed: {}", error);
        process::exit(1);
    });

    println!("Please check your downloads folder!");

    match std::fs::remove_file(&link_file){
        Ok(_) => (),
        Err(_) => {
            eprintln!("Error: failed to delete link file, maybe do it manually")
        }
    };
}

fn runner(regex: &str, link_file: &String)-> io::Result<()>{

    // unwrap call, followed by tuple index access is verified safe
    let usr_downloads_dir = link_file.rsplit_once('/').unwrap().0;

    println!("Please wait, wget is running...");

    let output = Command::new("wget")
        .arg("-r")
        .arg("-l2")
        .args(["-A", ".pdf"])
        .args(["--accept-regex", regex])
        .arg("-nc")
        .args(["-e", "robots=off"])
        .args(["-P", usr_downloads_dir])
        .args(["-i", link_file])
        .output()?;


        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        };
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        };


        Ok(())
    
}

