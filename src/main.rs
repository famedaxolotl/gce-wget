#![allow(special_module_name)]
mod lib;
use lib::{arg_parser, get_url, parse_regex, create_link_file};
use std::process::Command;
use std::env;
// use std::process;

fn main() {
    let args = arg_parser::arg_parser().unwrap();
    println!("Downloading the following subject: {}", args.sub_code);
    println!("Of the following types: {:?}", args.types);
    println!("The following paper codes: {:?}", args.codes);
    println!("From the following years: {:?}", args.years);

    let regex = parse_regex::parse(&args).unwrap();
    // println!("{}", regex);
    let url = get_url::get_url(&args.sub_code).unwrap();
    // println!("{}", url);

    let link_file = match create_link_file::create_link_file(&url, &args.years){
        Ok(val) => val,
        Err(e) => e.to_string()
    };



    runner(&regex, &link_file).expect("Something went wrong");

    println!("Please check your downloads folder!");

}

fn runner(regex: &str, link_file: &String)-> Result<(), &'static str>{

    // let target_year = years.first().unwrap();

    let usr_downloads_dir = format!("{}/Downloads", env::var("HOME").expect("Home env var not found"));

    println!("Please wait, wget is running...");

    let output = Command::new("wget")
        .arg("-r")
        .arg("-l2")
        .args(["-A", ".pdf"])
        .args(["--accept-regex", regex])
        .arg("-nc")
        .args(["-e", "robots=off"])
        .args(["-P", usr_downloads_dir.as_str()])
        // .arg(format!("{}{}", url, target_year.as_str()))
        .args(["-i", link_file])
        .output()
        .expect("Wget execution failed");


        if !output.stdout.is_empty() {
            println!("{}", String::from_utf8_lossy(&output.stdout));
        };
        if !output.stderr.is_empty() {
            eprintln!("{}", String::from_utf8_lossy(&output.stderr));
        };

        std::fs::remove_file(format!("{}/.temp-link-file", usr_downloads_dir)).expect("Couldn't delete link file");

        Ok(())
    
}

