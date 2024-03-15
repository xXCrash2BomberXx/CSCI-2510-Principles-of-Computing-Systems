use std::io;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input).unwrap() {
            0 => {
                break;
            },
            _ => {
                match File::open(input.trim()) {
                    Ok(mut x) => {
                        input.clear();
                        match x.read_to_string(&mut input) {
                            Ok(..) => println!("{}", input),
                            Err(y) => println!("{:?}", y),
                        }
                    },
                    Err(..) => {
                        println!("{:?}", std::result::Result::<i32, &str>::Err("Could not open file"));
                    },
                };
                input.clear();
            },
        };
    }
}
