use std::env;
use std::fs::{File, read};
use std::io::Write;
use rand::seq::{SliceRandom};
use std::thread;

fn get_bytes(filename : String) -> Vec<u8>{
    let bytes_vector = read(filename).unwrap();
    return bytes_vector;
}

fn write_jpg(data : Vec<u8>, filename : String){
    let mut f = File::create(filename).unwrap();
    f.write_all(&data).unwrap();
}

fn select_indexes(n_indexes : i32, n_selections: i32) -> Vec<i32>{
    // Excludes start of image and end of image markers from index range
    let index_range : Vec<i32> = (2..(n_indexes - 2)).collect();
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

fn get_magic_number() -> (i32, i32){
    // Format: (length in bytes, value of leading byte)
    let magic = [
        (1, 255),
        (1, 255),
        (1, 127),
        (1, 0),
        (2, 255),
        (2, 0),
        (4, 255),
        (4, 0),
        (4, 128),
        (4, 64),
        (4, 127)
    ];

    let magic_opt = magic.choose(&mut rand::thread_rng());
    match magic_opt{
        Some(num) => {
            return *num;
        },
        None => {
            panic!("Could not select magic number");
        }
    }
}

fn overwrite_with_magic(mut bytes : Vec<u8>, magic_n : (i32, i32)) -> Vec<u8>{
    match magic_n.0 {
        1 => {
            let indexes : Vec<i32> = (2..(bytes.len()) as i32 -2).collect();
            let index_opt = indexes.choose(&mut rand::thread_rng());
            match index_opt {
                Some(i) => {
                    bytes[*i as usize] = magic_n.1 as u8;
                },
                None => {
                    panic!("Cannot choose index");
                }
            }
        },
        2 => {
            let indexes : Vec<i32> = (2..(bytes.len()) as i32 -3).collect();
            let index_opt = indexes.choose(&mut rand::thread_rng());
            match index_opt {
                Some(i) => {
                    bytes[*i as usize] = magic_n.1 as u8;
                    bytes[(*i + 1) as usize] = magic_n.1 as u8;
                },
                None => {
                    panic!("Cannot choose index");
                }
            }
        },
        4 => {
            let indexes : Vec<i32> = (2..(bytes.len()) as i32 -6).collect();
            let index_opt = indexes.choose(&mut rand::thread_rng());
            match index_opt {
                Some(i) => {
                    match magic_n.1 {
                        255 => {
                            bytes[*i as usize] = 255;
                            bytes[(*i + 1) as usize] = 225;
                            bytes[(*i + 1) as usize] = 225;
                            bytes[(*i + 1) as usize] = 225;
                        },
                        0 => {
                            bytes[*i as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                        },
                        128 => {
                            bytes[*i as usize] = 128;
                            bytes[(*i + 1) as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                        },
                        64 => {
                            bytes[*i as usize] = 64;
                            bytes[(*i + 1) as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                            bytes[(*i + 1) as usize] = 0;
                        },
                        127 => {
                            bytes[*i as usize] = 127;
                            bytes[(*i + 1) as usize] = 255;
                            bytes[(*i + 1) as usize] = 255;
                            bytes[(*i + 1) as usize] = 255;
                        },
                        _ => {
                            panic!("Invalid magic byte {} {}", magic_n.0, magic_n.1);
                        }
                    }
                },
                None => {
                    panic!("Cannot choose index");
                }
            }
        },
        _ => {
            panic!("Invalid magic number length");
        }
    }
    return bytes;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: cargo run <jpg-path> <fuzzing-target-path");
        return;
    }

    let bytes = get_bytes(args[1].clone());

    // Spawn 20 threads, each of which will create a mutated image 500 times
    for i in 0..20{
        let mut bytes_clone = bytes.clone();
        let i_clone = i.clone();
        thread::spawn(move || {
            let original_bytes = bytes_clone.clone();
            for j in 0..500{
                bytes_clone = original_bytes.clone();
                // Randomly choose between bit-flipping or magic number mutation
                let options : Vec<i32> = (0..2).collect();
                match options.choose(&mut rand::thread_rng()){
                    Some(0) => {
                        // Only perform flips on 1% of bytes
                        let n_flips = ((bytes_clone.len() as i32) as f64 * 0.01).floor() as i32;
                        let selected_i = select_indexes(bytes_clone.len() as i32, n_flips);
                        bytes_clone = flip_bits(bytes_clone, selected_i);
                    },
                    Some(1) => {
                        let magic_n = get_magic_number();
                        bytes_clone = overwrite_with_magic(bytes_clone, magic_n);
                    },
                    _ => {
                        panic!("Could not choose between functions");
                    }
                }
                write_jpg(bytes_clone, String::from(format!("output/{}-{}-output.jpg", i_clone, j)));
            }
        });
    }


}
