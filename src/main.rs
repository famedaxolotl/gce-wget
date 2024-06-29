mod arg_parser;
mod parse_regex;
use arg_parser::arg_parser;

fn main() {
    let args = arg_parser().unwrap();
    println!("{}", args.sub_code);
    println!("{:?}", args.types);
    println!("{:?}", args.codes);
    println!("{:?}", args.years);

    let regex = parse_regex::parse(args).unwrap();
    println!("{}", regex)
}
