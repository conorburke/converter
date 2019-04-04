use calamine::{open_workbook, Reader, Xlsx};
use std::env;

mod lib;

fn main() {
    lib::convert::printconv();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() < 2 {
        panic!("not enough arguments");
    }
    let input = lib::gets::Input::new(&args);
    println!("{:?}", input);

    let mut excel: Xlsx<_> = open_workbook("heroes.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }
}
