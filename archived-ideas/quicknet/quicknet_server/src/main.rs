#![warn(clippy::all)]

use md5::compute as md5_compute;
use std::net::UdpSocket;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
// use quicknet_core::QuicknetServer;
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

pub fn start() -> Result<(), TrickError> {
    let mut main_buffer = [0; UDP_DATAGRAM_MAX_LENGTH];
    // * Get host from env::args or use default host
    // let host = "127.0.0.1:8000".to_owned();
    let host = "0.0.0.0:8000".to_owned();

    // * Open UDP socket
    println!("Starting server at [{}]:\n", &host);
    let socket = UdpSocket::bind(&host).expect("failed to bind host socket");

    print!("      TIMESTAMP       |");
    print!("                 FLOW                 |");
    print!(" ELAPSED (ms) |");
    print!(" BYTES |");
    print!("         HASH Content(MD5)        ");
    println!();

    // * Threadpool?
    loop {
        // * Receive Data
        let (number_of_bytes, client_host) = socket
            .recv_from(&mut main_buffer)
            .expect("no data received");

        // * Get system datetime of received datagram
        let system_datetime = SystemTime::now();

        // * Start time measurement
        let now = Instant::now();

        // * Copy receive data to some local buffer
        let mut received_bytes: Vec<u8> = main_buffer
            .iter()
            .take(number_of_bytes)
            .copied()
            .collect::<Vec<u8>>();

        let received_datagram_hash = format!("{:x}", md5_compute(&received_bytes));

        let current_datetime = system_datetime
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs_f64()
            .to_string();

        println!(
            "{timestamp:<20}s | {host:>16} <- {client:<16} | {elapsed:>12} | {length:>5} | {hash:32}",
            timestamp = current_datetime,
            host = host,
            client = client_host,
            elapsed = now.elapsed().as_millis(),
            length = number_of_bytes,
            hash = received_datagram_hash
        );

        // * Replace all 'AAA...' to 'BBB...'
        // for item in received_bytes.iter_mut() {
        //     if *item > 0 {
        //         *item += 1;
        //     } else {
        //         break;
        //     }
        // }

        // * Send Data
        let now = Instant::now();
        let sent_bytes_length = socket
            .send_to(&received_bytes, client_host)
            .expect("failed to send message");

        println!(
            "{timestamp:<20}s | {host:>16} -> {client:<16} | {elapsed:>12} | {length:>5} | {hash:32}",
            timestamp = current_datetime,
            host = host,
            client = client_host,
            elapsed = now.elapsed().as_millis(),
            length = sent_bytes_length,
            hash = received_datagram_hash
        );
    }

    // ? Control through unix socket
    // * Server, Monitor and cli
    // * See OpenBSD software achitecture like bgpd, smtpd, ldapd, etc.
}
