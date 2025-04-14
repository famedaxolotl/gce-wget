use reqwest::blocking::get;
use super::arg_parser::Qual;

pub fn get_url(sub_code: &String, force_flag: &Qual) -> Result<String, Box<dyn std::error::Error>>{

    let exam_type: String = match force_flag{
        // forced qualification logic
        Qual::Alevel => String::from("a-levels"),
        Qual::Igcse => String::from("cambridge-IGCSE"),
        Qual::OLevel => String::from("o-levels"),
        Qual::None => {
            // unwrap call here sufficient because arg_parser ensures Some
            // automatic qualification logic (no o-level)
            if sub_code.get(0..1).unwrap() == "0"{
                String::from("cambridge-IGCSE")
            }else{
                String::from("a-levels")
            }
        }
    };

    println!("Searching qualification: {}", exam_type.to_uppercase());    
    let url_name_string = fetch_name(sub_code, &exam_type)?;

    Ok(format!("https://papers.gceguide.cc/{}/{}/", exam_type, url_name_string))
}

fn fetch_name(sub_code: &String, exam_type: &String) -> Result<String, Box<dyn std::error::Error>>{
    let mut potential_names: Vec<&str> = Vec::new();
    
        // Fetch the HTML content
        let response = get(format!("https://papers.gceguide.cc/{}", exam_type))
            .map_err(|err| format!("Failed to fetch info from papers.gceguide.cc: {}", err))?;

        let body = response.text()
            .map_err(|err| format!("Failed to read info from papers.gceguide.cc: {}", err))?;
    
        // Split the content into individual lines
        let lines: Vec<&str> = body.lines().collect();
    
        // Finds the big line with all subject list elements
        for line in lines {
            if line.contains(sub_code) {
                // Splits the big line
                let sub_lines: Vec<&str> = line.split('>').collect();
                // Find the 2 lines with the given sub_code
                for each_line in sub_lines{
                    if each_line.contains(sub_code){
                        // println!("{}", each_line);
                        potential_names.push(each_line);
                    }
                }
            }
        };

        // Takes the second of the output lines
        let sub_name_raw: &str = potential_names.get(1).ok_or_else(|| format!("Subject not found. Please check again or use qualification options (-i, -a or -o)."))?;

        // This prints the subject name before it is adjusted to url form
        println!("Subject found: {}", sub_name_raw[..sub_name_raw.len() - 3].to_string());

        let sub_name_final: String = sub_name_raw
        .replace(' ', "-")
        .replace('(', "%28")
        .replace(')', "%29")
        .to_lowercase()
        // these replace methods ensures "AS" and "A" remain capitalised when
        // subject names contain "(AS/A level only)"
        .replace("%28as-level-only%29", "%28AS-level-only%29")
        .replace("%28a-level-only%29", "%28A-level-only%29");

        // Removes residual HTML tag text and returns name in url-form
        Ok(sub_name_final[..sub_name_final.len() - 3].to_string())
}