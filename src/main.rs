use std::io::Read;

fn main() {
    println!("Hello, world!");

    let mut inputfile = std::fs::File::open("Keys.json").unwrap();
    let mut contents = String::from("");
    match inputfile.read_to_string(&mut contents) {
        Ok(n) => println!("Read {} lines", n),
        Err(_) => println!("Cannot read file Keys.json"),
    }

    println!("contents:\n{}", contents);
}


