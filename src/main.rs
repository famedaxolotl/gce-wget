pub mod arg_parser;
use arg_parser::arg_parser;

fn main() {
    let args = arg_parser().unwrap();
    println!("{}", args.sub_code);
    println!("{:?}", args.types);
    println!("{:?}", args.codes);
    println!("{:?}", args.years);
}
