use std::io;

fn main () {
    let mut input = String::new();
    loop {
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        print!("{}", input);
        input.clear();
    }
}
