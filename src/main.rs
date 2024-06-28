use clap::{command, Arg};

fn main() {
    let matches = command!()
            .about("Wget CLI tool to retrieve papers from papers.gceguide.com")
            .arg(
                Arg::new("subject_code")
                .help("Accepts ONE subject code, accepts IGCSE, AS/A Levels and O Levels")
                .required(true)
            )
            .arg(
                Arg::new("paper-types")
                .help("Comma separated list of paper types (e.g. --paper-types ms,qp,in")
                .short('t')
                .value_delimiter(',')
            )
            .arg(
                Arg::new("paper-codes")
                .help("Comma separated list of paper codes, incuding the variant (e.g. --paper-codes 11,21,31")
                .short('c')
                .value_delimiter(',')
            )
            .arg(
                Arg::new("years")
                .help("Comma seperated list of paper years; must specifiy at least one.")
                .value_delimiter(',')
                .required(true)
            ).get_matches();

            let sub_code = matches.get_one::<String>("subject_code").unwrap().clone();

            let mut codes_vec: Vec<String> = Vec::new();
            let mut years_vec: Vec<String> = Vec::new();

            let mut types_vec: Vec<String> = Vec::new();

            if let Some(values) = matches.get_many::<String>("paper-types") {
                for value in values {
                    types_vec.push(value.clone());
                }
            }

            if let Some(values) = matches.get_many::<String>("paper-codes") {
                for value in values {
                    codes_vec.push(value.clone());
                }
            }

            if let Some(values) = matches.get_many::<String>("years") {
                for value in values {
                    years_vec.push(value.clone());
                }
            }
            println!("{}", sub_code);
            println!("{:?}", types_vec);
            println!("{:?}", codes_vec);
            println!("{:?}", years_vec);

}
