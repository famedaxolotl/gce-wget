use reqwest::blocking::get;

pub fn get_url(sub_code: &String) -> Result<String, &'static str>{
    let exam_type: String = if sub_code.get(0..1).unwrap() == "0"{
        String::from("Cambridge%20IGCSE")
    }else{
        String::from("A%20Levels")
    };
    let url_name_string = fetch_name(sub_code, &exam_type);
    

    Ok(format!("https://papers.gceguide.net/{}/{}/", exam_type, url_name_string))
}

fn fetch_name(sub_code: &String, exam_type: &String) -> String{
    let mut potential_names: Vec<&str> = Vec::new();
    
        // Fetch the HTML content
        let response = get(format!("https://papers.gceguide.net/{}", exam_type)).expect("Failed to fetch URL");
        let body = response.text().expect("Failed to read response body");
    
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
    let sub_name_raw: &str = potential_names.get(1).expect("No subject found with given subject code");
        let sub_name_final: String = sub_name_raw
        .replace(' ', "%20")
        .replace('(', "%28")
        .replace(')', "%29");

        // Removes residual HTML tag text
        sub_name_final[..sub_name_final.len() - 3].to_string()
}