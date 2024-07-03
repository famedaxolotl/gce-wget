use reqwest::blocking::get;
use super::arg_parser::Qual;

pub fn get_url(sub_code: &String, force_flag: &Qual) -> Result<String, &'static str>{

    let exam_type: String;
    match force_flag{
        // forced qualification logic
        Qual::Alevel => exam_type = String::from("A%20Levels"),
        Qual::IGCSE => exam_type = String::from("Cambridge%20IGCSE"),
        Qual::OLevel => exam_type = String::from("O%20Levels"),
        Qual::None => {
            // unwrap call here sufficient because arg_parser ensures Some
            // automatic qualification logic (no o-level)
            exam_type = if sub_code.get(0..1).unwrap() == "0"{
                String::from("Cambridge%20IGCSE")
            }else{
                String::from("A%20Levels")
            };
        }
    }

    let url_name_string = fetch_name(sub_code, &exam_type)?;    

    Ok(format!("https://papers.gceguide.net/{}/{}/", exam_type, url_name_string))
}

fn fetch_name(sub_code: &String, exam_type: &String) -> Result<String, &'static str>{
    let mut potential_names: Vec<&str> = Vec::new();
    
        // Fetch the HTML content
        let response = match get(format!("https://papers.gceguide.net/{}", exam_type)){
            Ok(res) => res,
            Err(_) => return Err("failed to get necessary info from papers.gceguide.net")
        };

        let body = match response.text(){
            Ok(bod) => bod,
            Err(_) => return Err("failed to read info from papers.gce-guide.net")
        };
    
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
        }
        // Takes the second of the output lines
        let sub_name_raw: &str = match potential_names.get(1){
            Some(sub_name) => sub_name,
            None => return Err("the entered subject couldn't be found on papers.gceguide.net")
        };
        let sub_name_final: String = sub_name_raw
        .replace(' ', "%20")
        .replace('(', "%28")
        .replace(')', "%29");

        // Removes residual HTML tag text
        Ok(sub_name_final[..sub_name_final.len() - 3].to_string())
}