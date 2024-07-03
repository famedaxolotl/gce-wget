use clap::{command, Arg};

pub struct Config{
    pub sub_code: String,
    pub types: Vec<String>,
    pub codes: Vec<String>,
    pub years: Vec<String>,
}

impl Config{
        pub fn new() -> Result<Config, &'static str>{

            let matches = command!()
            .about("Wget CLI tool to retrieve papers from papers.gceguide.net")
            .arg(
                Arg::new("subject_code")
                .help("Accepts ONE subject code, accepts IGCSE and AS/A Levels")
                .required(true)
            )
            .arg(
                Arg::new("paper-types")
                .help("Comma delimited list of paper types (e.g. --paper-types ms,qp,in)")
                .short('t')
                .long("types")
                .value_delimiter(',')
            )
            .arg(
                Arg::new("paper-codes")
                .help("Comma delimited list of paper codes, incuding the variant (e.g. --paper-codes 11,21,31)")
                .short('c')
                .long("codes")
                .value_delimiter(',')
            )
            .arg(
                Arg::new("years")
                .help("Comma delimited list of paper years; must specifiy at least one.")
                .value_delimiter(',')
                .required(true)
            ).get_matches();
        
            // Getting matches and inserting them into vecs
        
            let mut sub_code: String = String::new();
        
            if let Some(code) = matches.get_one::<String>("subject_code"){
                if code.len() == 4 && code.parse::<i32>().is_ok(){
                    sub_code.push_str(code);
                }else{
                    return Err("Enter valid 4 digit subject code")
                }
            }
        
            let mut codes_vec: Vec<String> = Vec::new();
            let mut years_vec: Vec<String> = Vec::new();
        
            let mut types_vec: Vec<String> = Vec::new();
        
            if let Some(values) = matches.get_many::<String>("paper-types") {
                for value in values {
                    let types_list: [&str; 6] = ["qp", "ms", "in", "er", "gt", "tr"];
        
                    if !types_list.contains(&value.as_str()){
                        return Err("You have passed an invalid paper type");
                    }else{
                        types_vec.push(value.clone());
                    }
        
                }
            }
        
            if let Some(values) = matches.get_many::<String>("paper-codes") {
                for value in values {
        
                    if (value.len() == 2 || value.len() == 1) && value.parse::<i32>().is_ok(){
                        codes_vec.push(value.clone())
                    }else{
                        return Err("Enter valid 2-digit paper codes")
                    }
                }
            }
        
            if let Some(values) = matches.get_many::<String>("years") {
                for value in values {
                    
                    if value.len() == 4 && value.parse::<i32>().is_ok(){
                        years_vec.push(value.clone())
                    }else{
                        return Err("Enter valid 4-digit years")
                    }
                }
            }
        
            Ok(Config{
                sub_code,
                types: types_vec,
                codes: codes_vec,
                years: years_vec
            })
        }
    }

