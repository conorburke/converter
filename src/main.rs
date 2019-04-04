use calamine::{open_workbook, Reader, Xlsx};
use std::env;
use std::error::Error;
use std::fs::File;

mod lib;

fn main() -> Result<(), Box<Error>>{
    lib::convert::printconv();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 3 {
        panic!("I need a source file (xlsx) and a destination file (.csv)");
    }
    let input = lib::gets::Input::new(&args);
    println!("{:?}", input);

    let buffer = File::create(input.outputname)?;
    let mut wtr = csv::Writer::from_writer(buffer); 

    let mut excel: Xlsx<_> = open_workbook(input.filename).unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("{:?}", row);
            let mut vec: Vec<String> = Vec::new();
            for x in row {
              vec.push(x.to_string());
              println!("{}", x);
            }
            wtr.write_record(vec)?;
        }
    }
    wtr.flush()?;
    Ok(())
}
