pub mod convert {
    pub fn printconv() -> i32 {
        println!("in convert!");
        2
    }
}

pub mod gets {
    #[derive(Debug)]
    pub struct Input {
        filename: String,
    }

    // fn read_inputs(args: &[String]) -> Input {
    //     let filename = args[1].clone();

    //     Input { ilename }
    // }

    impl Input {
        pub fn new(args: &[String]) -> Input {
            let filename = args[1].clone();

            Input { filename }
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
