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
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Instant;

use hello_proto::MESSAGE_CHUNK_LENGTH;

static RESPONSE: &str = include_str!("response.raw");

fn handle_client(mut stream: TcpStream) {
    let mut recv_buf = [0u8; MESSAGE_CHUNK_LENGTH];

    'handle_client: loop {
        match stream.read(&mut recv_buf) {
            Ok(_) => {
                // echo everything!
                dbg!();
                send_default_response(&mut stream);
                // stream.write(&send_buf[0..size]).unwrap();
                // std::thread::sleep(std::time::Duration::from_millis(100));
                break 'handle_client;
                // true
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );

                break 'handle_client;
                // false
            }
        }
    }
    println!("Closing connection from {}",stream.peer_addr().unwrap());
    stream.shutdown(Shutdown::Both).unwrap();
}
fn send_default_response(stream: &mut TcpStream) {
    use std::io::Cursor;
    let tamanho_total_a_ser_enviado = RESPONSE.len();
    let numero_de_envios = tamanho_total_a_ser_enviado / MESSAGE_CHUNK_LENGTH;
    let tamanho_do_ultimo_envio = tamanho_total_a_ser_enviado % MESSAGE_CHUNK_LENGTH;
    dbg!(
        tamanho_total_a_ser_enviado,
        numero_de_envios,
        tamanho_do_ultimo_envio
    );
    let mut reponse_to_send = Cursor::new(RESPONSE);
    // * Pegar response e transformar num Cursor, para permitir envio segmentado via
    // * buffer de 16 Bytes. Deste modo o response será enviado em blocos do tamanho
    // * do buffer.
    let mut send_buf = [0u8; MESSAGE_CHUNK_LENGTH];
    // let mut buf = [0u8; MESSAGE_CHUNK_LENGTH];
    let mut counter: usize = 0;
    'send_default_response: loop {
        match reponse_to_send.read(&mut send_buf) {
            Ok(_) => {
                if send_buf[0] == 0 {
                    break 'send_default_response;
                }
                // println!("Foi lido da variárvel `reponse_to_send` {size} Bytes ");
                counter += 1;
                println!("{counter}: mensagem para enviar!{send_buf:?}");
                // echo everything!
                // stream.read_exact(&mut send_buf).unwrap();
                stream.write_all(&send_buf).unwrap();
                // println!("Foi enviado via rede um payload da variárvel `reponse_to_send` {size} Bytes ");
                println!("{counter}: mensagem enviada!{send_buf:?}");
                send_buf = [0u8; MESSAGE_CHUNK_LENGTH];
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                break 'send_default_response;
            }
        }
    }
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for res in listener.incoming() {
        // let start = Instant::now();

        match res {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(stream);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }

        // std::thread::sleep(std::time::Duration::from_millis(10));
        // println!("Another loop an {} at line {}", file!(), line!());

        // let duration = start.elapsed();
        // println!("Terminated at {} ms.", duration.as_millis());
    }
    // close the socket server
    drop(listener);
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
