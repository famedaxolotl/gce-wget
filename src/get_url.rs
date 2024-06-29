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
    let sub_name_raw: &str;

    let mut potent_name: Vec<&str> = Vec::new();
    
        // Fetch the HTML content
        let response = get(format!("https://papers.gceguide.net/{}", exam_type)).expect("Failed to fetch URL");
        let body = response.text().expect("Failed to read response body");
    
        // Split the content into individual lines
        let lines: Vec<&str> = body.lines().collect();
    
        for line in lines {
            if line.contains(sub_code) {
                let sub_lines: Vec<&str> = line.split(">").collect();

                for each_line in sub_lines{
                    if each_line.contains(sub_code){
                        // println!("{}", each_line);
                        potent_name.push(each_line);
                    }
                }
            }
        }

        sub_name_raw = &potent_name.get(1).unwrap();
        let sub_name_final: String = sub_name_raw
        .replace(" ", "%20")
        .replace("(", "%28")
        .replace(")", "%29");

        sub_name_final[..sub_name_final.len() - 3].to_string()
}