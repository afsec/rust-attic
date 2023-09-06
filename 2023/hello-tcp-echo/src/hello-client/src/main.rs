#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::type_complexity,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(
    elided_lifetimes_in_paths,
    clippy::new_ret_no_self,
    clippy::unused_self
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::time::{Duration, Instant};

use hello_proto::{MESSAGE_CHUNK_LENGTH, RESPONSE_MAX_LENGTH};

fn main() {
    let start = Instant::now();

    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write(msg).unwrap();

            println!("Sent Hello, awaiting reply...");

            let mut response: Vec<u8> = vec![];

            let mut recv_buf = [0u8; MESSAGE_CHUNK_LENGTH];

            'inner_read: loop {
                match stream.read(&mut recv_buf) {
                    Ok(recv_bytes_len) => {
                        // dbg!(recv_bytes, recv_buf);
                        if recv_bytes_len > 0 {
                            for byte in recv_buf {
                                if byte > 0 {
                                    response.push(byte);
                                }
                                recv_buf = [0u8; MESSAGE_CHUNK_LENGTH];
                            }
                            if response.len() > RESPONSE_MAX_LENGTH {
                                eprintln!("RESPONSE_MAX_LENGTH reached!");
                                break 'inner_read;
                            }
                        } else {
                            println!("No bytes received!");
                            break 'inner_read;
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                        break 'inner_read;
                    }
                }
            }
            let response_str = from_utf8(&response).unwrap();
            if response.ends_with(b"\r\n\r\n") {
                println!("Reponse finished successfully!");
                println!("{response_str}");
            } else {
                println!("Reponse corrupted!");

                println!("{response_str:#?}");
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    let duration = start.elapsed();
    println!("Terminated at {} ms.", duration.as_millis());
}

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
