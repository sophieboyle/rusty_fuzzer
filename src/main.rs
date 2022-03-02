use std::env;
use std::fs::{File, read};
use std::io::Write;

fn get_bytes(filename : String) -> Vec<u8>{
    let bytes_vector = read(filename).unwrap();
    return bytes_vector;
}

fn write_jpg(data : Vec<u8>){
    let mut f = File::create("output.jpg").unwrap();
    f.write_all(&data).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <jpg-path>");
    }

    let bytes = get_bytes(args[1].clone());
    write_jpg(bytes);
}
