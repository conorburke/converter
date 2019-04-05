pub mod derives {
    use serde::Serialize;
    #[derive(Debug, Serialize)]
    pub struct Record {
        pub name: String,
        pub universe: String
    }
}

pub mod gets {
    use calamine::{open_workbook, Reader, Xlsx};
    use std::error::Error;
    use std::fs::File;
    pub fn converter(args: Vec<String>) -> Result<(), Box<Error>>{
    let input = Input::new(&args);

        let buffer = File::create(input.outputname)?;
        let mut wtr = csv::Writer::from_writer(buffer); 

        let mut excel: Xlsx<_> = open_workbook(input.filename).unwrap();
        if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
            for row in r.rows() {
                wtr.serialize(super::derives::Record {
                name: row[0].to_string(),
                universe: row[1].to_string(),
                })?;
            }
        }
        wtr.flush()?;
        Ok(())
    }



    #[derive(Debug)]
    pub struct Input {
        pub filename: String,
        pub outputname: String
    }

    impl Input {
        pub fn new(args: &[String]) -> Input {
            let filename = args[1].clone();
            let outputname = args[2].clone();

            Input { filename, outputname }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
