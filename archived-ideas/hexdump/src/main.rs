use std::env;
use std::fs::File;
use std::io::{Cursor, Read};

// Reference: https://trentmay.medium.com/writing-a-hex-dump-utility-in-rust-e98b3355e530

const GLOBAL_BUFFER_LENGTH: usize = 16;

const FILE_BUFFER_LENGTH: usize = GLOBAL_BUFFER_LENGTH * 1_00_000; // 16_000_000 Bytes

fn get_file(path_to_file: String) -> File {
    match File::open(path_to_file) {
        Ok(f) => File::from(f),
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn get_hex_rep(byte_array: &mut [u8]) -> String {
    let to_fill = GLOBAL_BUFFER_LENGTH - byte_array.len();
    let build_string_vec: Vec<String> = byte_array
        .chunks(2)
        .enumerate()
        .map(|(idx, c)| {
            let mut hex_rep = if idx == 4 {
                " ".to_string()
            } else {
                "".to_string()
            };

            if c.len() == 2 {
                hex_rep.push_str(format!("{:02x} {:02x}", c[0], c[1]).as_str());
            } else {
                hex_rep.push_str(format!("{:02x}", c[0]).as_str());
            };
            hex_rep
        })
        .collect();

    let mut hex_rep = build_string_vec.join(" ");
    for _ in 0..to_fill {
        hex_rep.push_str("   ");
    }
    hex_rep
}

pub fn get_ascii_representation(byte_array: &mut [u8]) -> String {
    byte_array
        .iter()
        .map(|num| {
            if *num >= 32 && *num <= 126 {
                *num as char
            } else {
                '.'
            }
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not enough arguments!");
    }

    let mut file_to_read = get_file(String::from(&args[1]));

    let mut file_buffer: [u8; FILE_BUFFER_LENGTH] = [0; FILE_BUFFER_LENGTH];

    let mut buff = [0; GLOBAL_BUFFER_LENGTH];
    let mut offset: usize = 0;

    'file: loop {
        // let bytes_read = file_to_read.read(&mut buff);

        match file_to_read.read(&mut file_buffer) {
            Ok(bytes_read_from_file) => {
                if bytes_read_from_file == 0 {
                    break;
                } else {
                    let mut file_cursor: Cursor<Vec<u8>> =
                        Cursor::new(Vec::from(&mut file_buffer[0..bytes_read_from_file]));
                    'cursor: loop {
                        match file_cursor.read(&mut buff) {
                            Ok(bytes_read_from_cursor) => {
                                if bytes_read_from_cursor == 0 {
                                    break 'cursor;
                                } else {
                                    println!(
                                        "{:08x}  {:40}  |{:10}|",
                                        offset,
                                        get_hex_rep(&mut buff[0..bytes_read_from_cursor]),
                                        get_ascii_representation(
                                            &mut buff[0..bytes_read_from_cursor]
                                        )
                                    );
                                    offset += bytes_read_from_cursor;
                                }
                            }
                            Err(error) => {
                                eprintln!("{}  {}", env!("CARGO_PKG_NAME"), error);
                                break 'file;
                            }
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("{}: {}", env!("CARGO_PKG_NAME"), error);
                break 'file;
            }
        }
        // Offset
        println!("{:08x}", offset);
    }
}
