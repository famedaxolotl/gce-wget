use super::arg_parser::Config;

// upon further consideration this function
// has no need to return Result<T, E>
pub fn parse(config: &Config) -> String{
    let Config {sub_code, types, codes, years: _years, force_flag: _force_flag} = config;

    let mut types_str: String = String::new();
    let mut codes_str: String = String::new();

    // adds "|" after every item for the regex OR operator
    for sub_type in types.clone(){
        types_str.push_str(&sub_type);
        types_str += "|";
        
    } 

    for code in codes.clone(){
        codes_str.push_str(&code);
        codes_str += "|";
        
    }

    // removes trailing "|"
    types_str.pop();
    codes_str.pop();

    // constructs regex
    if codes.is_empty() && types.is_empty(){
        format!("{}_.{{3}}", sub_code)
    }else if codes.is_empty(){
        format!("{}_.{{3}}_({})", sub_code, types_str)
    }else if types.is_empty(){
        format!("{}_.{{3}}_.{{2}}_({})", sub_code, codes_str)
    }else{

        format!("{}_.{{3}}_({})_({})", sub_code, types_str, codes_str)
    }
}