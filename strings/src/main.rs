//use std::str;

fn main() {

    //"SLU" followed by a Grinning face emoji encoded in UTF-8
    let bytes = vec![ 0x53, 0x4C, 0x55, 0xF0, 0x9F, 0x98, 0x80 ];
    let view_into_bytes = &bytes[0..7]; //normally called a slice
    let result = String::from_utf8(view_into_bytes.to_vec());
    if result.is_ok(){
        println!("Found UTF characters {}", result.unwrap());  // Found UTF characters SLU😀
        //println!("Found UTF characters {}", result.unwrap().chars().count());  // 4
        //println!("Found UTF characters {}", result.unwrap().bytes().count());  // 7
        println!("{}, {}", String::from("Z").chars().count(), String::from("F").bytes().count());
    } else if result.is_err() {
        println!("Provided bytes were not valid UTF-8");
    }
    let mut s: String = String::from("Hello");
    println!("{s}");
    s += ", World!";
    println!("{s}");
    let index: usize = s.find("W").unwrap();
    println!("W at {index}");
    let s2 = s.replace("World", "Dave");
    println!("{s2}");
    println!("{:?}", s.split(", ").collect::<Vec<&str>>());
}
