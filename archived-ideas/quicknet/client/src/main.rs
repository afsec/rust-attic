#![warn(clippy::all)]

use std::net::UdpSocket;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use md5::compute as md5_compute;
use nanorand::{WyRand, RNG};

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

#[derive(Debug)]
pub struct TrickError;

fn main() {
    let result = start();
    if let Err(error) = result {
        eprintln!("ERROR: {:?}", error);
    }
}

fn random_source_port() -> u16 {
    let mut rng = WyRand::new();
    // let mut rng = rand::thread_rng();
    rng.generate_range::<u16>(10000, u16::MAX)
}

pub fn start() -> Result<(), TrickError> {
    let mut main_buffer = [0; UDP_DATAGRAM_MAX_LENGTH];
    let start = SystemTime::now();
    // * Get host from env::args or use default host
    let peer_host = "127.0.0.1:8000".to_owned();
    let local_host = format!("127.0.0.1:{local_port}", local_port = random_source_port());

    // * Open UDP socket
    println!("Starting CLIENT:\n");
    let socket = UdpSocket::bind(&local_host).expect("failed to bind host socket");

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
    let mut i = 0;
    for item in main_buffer.iter_mut() {
        *item = b'A';
        if i > data_length {
            break;
        } else {
            i += 1;
        }
    }

    // dbg!(&main_buffer.len());
    // dbg!(&first_eight_bytes);

    let bytes_to_be_sent = main_buffer;

    // * Generate MD5 hash
    let hash_sent = format!("{:x}", md5_compute(&bytes_to_be_sent));

    // * Start time measurement
    let now = Instant::now();

    // * Send Data
    let sent_bytes_length = socket
        .send_to(&bytes_to_be_sent, &peer_host)
        .expect("failed to send message");

    // * Stop time measurement
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs_f64().to_string();

    // * Show event: SEND (->)
    println!(
        "{timestamp:<20}s | {local_host:>16} -> {peer_host:<16} | {elapsed:>12} | {length:>5} | {hash_sent:22}",
        timestamp = since_the_epoch,
        local_host = local_host,
        peer_host = peer_host,
        elapsed = now.elapsed().as_millis(),
        length = sent_bytes_length,
        hash_sent = hash_sent
    );

    // * Receive Data
    // TODO: Implement Timeout
    let (number_of_bytes, peer_host) = socket
        .recv_from(&mut main_buffer)
        .expect("no data received");
    let now = Instant::now();
    let received_bytes: Vec<u8> = main_buffer
        .iter()
        .take(number_of_bytes)
        .copied()
        .collect::<Vec<u8>>();

    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs_f64().to_string();

    let hash = format!("{:x}", md5_compute(&received_bytes));

    // * Show event: RECEIVE (<-)
    println!(
        "{timestamp:<20}s  | {local_host:>16} <- {peer_host:<16} | {elapsed:>12} | {length:>5} | {hash:<32} ",
        timestamp = since_the_epoch,
        local_host = local_host,
        peer_host = peer_host,
        elapsed = now.elapsed().as_millis(),
        length = number_of_bytes,
        hash = hash
    );

    Ok(())
    // }

    // ? Control through unix socket
    // * Server, Monitor and cli
    // * See OpenBSD software achitecture like bgpd, smtpd, ldapd, etc.
}
