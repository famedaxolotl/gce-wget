use clap::{command, Arg, ArgAction, ArgGroup};

pub struct Config{
    pub sub_code: String,
    pub types: Vec<String>,
    pub codes: Vec<String>,
    pub years: Vec<String>,
    pub force_flag: Qual,
}
#[derive(Debug)]
pub enum Qual{
    IGCSE,
    OLevel,
    Alevel,
    None
}

impl Config{
        pub fn new() -> Result<Config, &'static str>{

            let matches = command!()
            .about("Wget CLI tool to retrieve papers from papers.gceguide.net")
            .version("1.0.2")
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
            )
            .arg(
                Arg::new("igcse")
                .help("Force IGCSE search")
                .short('i')
                .long("igcse")
                .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("a-level")
                .help("Force A Levels search")
                .short('a')
                .long("a-level")
                .action(ArgAction::SetTrue)
            )
            .arg(
                Arg::new("o-level")
                .help("Force O Levels search")
                .short('o')
                .long("o-level")
                .action(ArgAction::SetTrue)
                
            )
            .group(ArgGroup::new("force-qualification")
            .args(&["igcse", "o-level", "a-level"])
            .required(false)).get_matches();
        
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
                    let types_list = ["qp", "ms", "in", "er", "gt", "tr", "ci"];
        
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

            let force_flag: Qual;

            if matches.get_flag("igcse") {
                force_flag = Qual::IGCSE;
            }else if matches.get_flag("a-level") {
                force_flag = Qual::Alevel;
            }else if matches.get_flag("o-level") {
                force_flag = Qual::OLevel;
            }else{
                force_flag = Qual::None;
            }
        
            Ok(Config{
                sub_code,
                types: types_vec,
                codes: codes_vec,
                years: years_vec,
                force_flag
            })
        }
    }