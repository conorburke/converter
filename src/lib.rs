pub mod convert {
    pub fn printconv() -> i32 {
        println!("in convert!");
        2
    }
}

pub mod gets {
    #[derive(Debug)]
    pub struct Input {
        pub filename: String,
        pub outputname: String
    }

    // fn read_inputs(args: &[String]) -> Input {
    //     let filename = args[1].clone();

    //     Input { ilename }
    // }

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

    #[test]
    fn it_prints() {
        assert_eq!(super::convert::printconv(), 2);
    }
}
