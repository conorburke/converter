use std::env;

mod lib;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("I need a source file (xlsx) and a destination file (.csv)");
    }
    lib::gets::converter(args).expect("shit broke");
}
