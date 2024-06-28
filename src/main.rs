pub mod arg_parser;
use arg_parser::arg_parser;

fn main() {
    let (sub_code, types_vec, codes_vec, years_vec) = arg_parser();
    println!("{sub_code}");
    println!("{:?}", types_vec);
    println!("{:?}", codes_vec);
    println!("{:?}", years_vec);
}
