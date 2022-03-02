use std::env;
use std::fs::{File, read};
use std::io::Write;
use rand::seq::{SliceRandom};

fn get_bytes(filename : String) -> Vec<u8>{
    let bytes_vector = read(filename).unwrap();
    return bytes_vector;
}

fn write_jpg(data : Vec<u8>){
    let mut f = File::create("output.jpg").unwrap();
    f.write_all(&data).unwrap();
}

fn select_indexes(n_indexes : i32, n_selections: i32) -> Vec<i32>{
    let index_range : Vec<i32> = (0..n_indexes).collect();
    let mut selected_indexes = Vec::new();

    while selected_indexes.len() != n_selections as usize {
        let chosen_i = index_range.choose(&mut rand::thread_rng());
        match chosen_i {
            Some(i) => {
                selected_indexes.push(*i);
            },
            None => {
                panic!("Not enough indexes to choose given number of flips");
            }
        }
    }
    return selected_indexes;
}

fn flip_bits(mut bytes : Vec<u8>, byte_indexes : Vec<i32>) -> Vec<u8>{
    for i in byte_indexes{
        // println!("{:#010b}", bytes[i as usize]);
        let index_range : Vec<i32> = (0..8).collect();
        let bit_i_opt = index_range.choose(&mut rand::thread_rng());
        match bit_i_opt {
            Some(bit_i) => {
                bytes[i as usize] ^= (1 as u8) << (*bit_i as u8);
            },
            None => {
                panic!("Could not randomly select bit index of chosen byte");
            }
        }
        // println!("{:#010b}", bytes[i as usize]);
    }
    return bytes;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <jpg-path>");
        return;
    }

    let mut bytes = get_bytes(args[1].clone());

    // Only perform flips on 1% of bytes (excluding markers)
    let markers = 2;
    let marker_len = 2;
    let n_flips = ((bytes.len() as i32 - (markers * marker_len)) as f64 * 0.01).floor() as i32;
    let selected_i = select_indexes(bytes.len() as i32, n_flips);

    bytes = flip_bits(bytes, selected_i);

    write_jpg(bytes);
}
