use std::time::Duration;
use std::time::Instant;

const TRIALS: u32 = 10000000;

fn add( a:i32, b:i32 ) -> i32 {
    return a+b;
}

fn main () {
    let x = 10;
    println!("My Value: {x}");
    let x = 20;
    println!("My Value: {x}");
    let mut y = 20;
    println!("My Value: {y}");
    y = 10;
    println!("My Value: {y}");

    let start = Instant::now();
    for _ in 0..TRIALS {
        add(x, y);
    }
	let end = Instant::now();
    let runtime = end.duration_since(start);
	println!("Running time: {:?}", runtime/TRIALS);

    let start = Instant::now();
    for _ in 0..5 {
        println!("{x}");
    }
	let end = Instant::now();
    let runtime = end.duration_since(start);
	println!("Running time: {:?}", runtime/TRIALS);

}