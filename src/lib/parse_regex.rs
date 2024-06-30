use super::arg_parser::Args;

pub fn parse(args: &Args) -> Result<String, &'static str>{
    let Args {sub_code, types, codes, years: _years} = args;

    let mut types_str: String = String::new();
    let mut codes_str: String = String::new();

    for sub_type in types.clone(){
        types_str.push_str(&sub_type);
        types_str += "|";
        
    } 

    for code in codes.clone(){
        codes_str.push_str(&code);
        codes_str += "|";
        
    }
    types_str.pop();
    codes_str.pop();

    if codes.len() == 0 && types.len() == 0{
        Ok(format!("{}_.{{3}}", sub_code))
    }else if codes.len() == 0{
        Ok(format!("{}_.{{3}}_({})", sub_code, types_str))
    }else if types.len() ==0{
        Ok(format!("{}_.{{3}}_.{{2}}_({})", sub_code, codes_str))
    }else{

        Ok(format!("{}_.{{3}}_({})_({})", sub_code, types_str, codes_str))
    }
}