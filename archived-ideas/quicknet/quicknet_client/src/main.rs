#![warn(clippy::all)]

use std::net::UdpSocket;
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use md5::compute as md5_compute;

use quicknet_core::client::QuicknetClient;
use quicknet_core::errors::TrickError;

const UDP_DATAGRAM_MAX_LENGTH: usize = 65507;

// * IP Header: "Total Length" has 16 bits
// Total Length is the length of the datagram, measured in octets,
//  including internet header and data.  This field allows the length of
//  a datagram to be up to 65,535 octets.

// * Set buffer to UDP field Length where max lentgh is 65507. Because:
//  - UDP Headers uses 8 octects
//  - IP Headers uses 20 bytes
// Calculations:
//                     65535 (IPv4 packet max length)
//                      - 20 (IPv4 Headers)
//                         8 (UDP Headers)
//                   --------
//                     65507 (UDP Datagram max length)

fn main() {
    let result = start();
    if let Err(error) = result {
        eprintln!("ERROR: {:?}", error);
    }
}
struct Client;

impl QuicknetClient for Client {}

pub fn start() -> Result<(), TrickError> {
    let mut main_buffer = [0; UDP_DATAGRAM_MAX_LENGTH];
    // let mut main_buffer = Vec::with_capacity(UDP_DATAGRAM_MAX_LENGTH);

    // * Get host from env::args or use default host
    //let server_host = "127.0.0.1:8000";
    // let server_host = "quicknet-server:8000";
    let server_host = "192.168.0.253:8000";
    print!("      TIMESTAMP       |");
    print!("                 FLOW                 |");
    print!(" ELAPSED (ms) |");
    print!(" BYTES |");
    print!("         HASH Content(MD5)        ");
    println!();

    // * Threadpool?
    // loop {

    // * Define Data to be Send

    // * Populate buffer with 'AAA...'
    let data_length = 50;
    let mut pos = 0;
    for item in main_buffer.iter_mut() {
        if pos < data_length {
            pos += 1;
            *item = b'A';
        } else {
            break;
        }
    }

    // dbg!(&main_buffer.len());
    // dbg!(&first_eight_bytes);
    for message in 0..10000 {
        let bytes_to_be_sent = main_buffer;

        // * Generate MD5 hash
        let hash_sent = format!("{:x}", md5_compute(&bytes_to_be_sent));

        // * Start time measurement
        let time_measure_startpoint = Instant::now();

        // * Get system datetime of Send Event
        let current_datetime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64()
            .to_string();

        // * Show event: SEND (->)
        println!(
        "{timestamp:<20}s | {local_host:>16} -> {server_host:<16} | {elapsed:>12} | {length:>5} | {hash_sent:22}",
        timestamp = current_datetime,
        local_host = "0.0.0.0",
        server_host = &server_host,
        elapsed = 0,
        length = bytes_to_be_sent.len(),
        hash_sent = hash_sent
    );

        // * Send Data
        let (number_of_bytes, local_host, peer_host, received_bytes) =
            Client::send_message(&bytes_to_be_sent, server_host)?;

        // * Get system datetime of Receive Confirmation
        let current_datetime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64()
            .to_string();

        let hash_received = format!("{:x}", md5_compute(&received_bytes));

        // * Stop time measurement
        let elapsed = time_measure_startpoint.elapsed().as_millis();

        // * Show event: RECEIVE Confirmation (<-)
        println!(
                "{timestamp:<20}s | {local_host:>16} <- {peer_host:<16} | {elapsed:>12} | {length:>5} | {hash:<32} ",
                timestamp = current_datetime,
                local_host = local_host,
                peer_host = peer_host,
                elapsed = elapsed,
                length = number_of_bytes,
                hash = hash_received
            );
    }
    Ok(())
    // }

    // ? Control through unix socket
    // * Server, Monitor and cli
    // * See OpenBSD software achitecture like bgpd, smtpd, ldapd, etc.
}
