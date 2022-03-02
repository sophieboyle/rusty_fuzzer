use std::env;
use std::fs::read;

fn get_bytes(filename : String) -> Vec<u8>{
    let bytes_vector = read(filename).unwrap();
    return bytes_vector;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <jpg-path>");
    }

    let bytes = get_bytes(args[1].clone());
    print!("Test");
}
