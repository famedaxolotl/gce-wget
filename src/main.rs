mod arg_parser;
mod parse_regex;
mod get_url;
use arg_parser::arg_parser;

fn main() {
    let args = arg_parser().unwrap();
    println!("{}", args.sub_code);
    println!("{:?}", args.types);
    println!("{:?}", args.codes);
    println!("{:?}", args.years);

    let regex = parse_regex::parse(&args).unwrap();
    println!("{}", regex);
    let url = get_url::get_url(&args.sub_code).unwrap();
    println!("{}", url);
}
