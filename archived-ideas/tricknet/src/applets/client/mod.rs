use crate::errors::TrickError;

use std::net::UdpSocket;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use md5::compute as md5_compute;
use rand::Rng;

const PAYLOAD_MAX_PRINT: usize = 40;

fn random_source_port() -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(10000..u16::MAX)
}

pub fn encrypt_block(block: Vec<u8>) -> [u8; 8] {
    // TODO: The first block must be a SALT.
    // ?      Always?!

    use des::{
        cipher::{
            generic_array::{typenum::U8, GenericArray},
            BlockCipher, NewBlockCipher,
        },
        Des,
    };
    // * Encrypt with DES
    let shared_key = b"BOZOLAND";
    let des_engine = Des::new_varkey(shared_key).unwrap();
    let mut encrypted_block: GenericArray<u8, U8> = *GenericArray::from_slice(&block);
    dbg!(&encrypted_block.len());
    des_engine.encrypt_block(&mut encrypted_block);
    *encrypted_block.as_ref()
}

pub fn start() -> Result<(), TrickError> {
    let start = SystemTime::now();
    // * Get host from env::args or use default host
    let peer_host = "127.0.0.1:8000".to_owned();
    let local_host = format!("127.0.0.1:{local_port}", local_port = random_source_port());

    // * IP Header: "Total Length" has 16 bits
    // Total Length is the length of the datagram, measured in octets,
    //  including internet header and data.  This field allows the length of
    //  a datagram to be up to 65,535 octets.

    // * Set buffer to UDP field Length where max lentgh is 65507. Because:
    //  - UDP Headers is 8 octects
    //  - IP Headers is 20 bytes
    // * Open UDP socket
    println!("Starting CLIENT:\n");
    let socket = UdpSocket::bind(&local_host).expect("failed to bind host socket");

    // println!(
    //     "      TIMESTAMP       |                 FLOW                 | ELAPSED (ms) | BYTES |    HASH Content(MD5) "
    // );
    println!(
            "      TIMESTAMP       |                 FLOW                 | ELAPSED (ms) | BYTES |              HASH Content(MD5)            |  PAYLOAD (First {length} Bytes)",length=PAYLOAD_MAX_PRINT
        );
    // * Threadpool?
    // loop {
    let mut main_buffer = [0; 65507];
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

    // * Send Data

    let block_before_encryption: Vec<u8> = main_buffer
        .iter()
        .take(8)
        .map(|x| x.clone())
        .collect::<Vec<u8>>();
    dbg!(&main_buffer.len());
    dbg!(&block_before_encryption);

    let encrypted_block = encrypt_block(block_before_encryption.clone());

    dbg!(&encrypted_block);

    let bytes_to_be_sent = encrypted_block;

    // * Generate MD5 hash
    let hash_sent = format!("{:x}", md5_compute(&bytes_to_be_sent));

    let now = Instant::now();

    let sent_bytes_length = socket
        .send_to(&bytes_to_be_sent, &peer_host)
        .expect("failed to send message");

    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    println!(
        "{timestamp:?} | {local_host:>16} -> {peer_host:<16} | {elapsed:>12} | {length:>5} | {hash_sent} | ",
        timestamp = since_the_epoch,
        local_host = local_host,
        peer_host = peer_host,
        elapsed = now.elapsed().as_millis(),
        length = sent_bytes_length,
        hash_sent = hash_sent
    );

    println!(
        "{message:<length$}",
        message = String::from_utf8(block_before_encryption).unwrap(),
        length = PAYLOAD_MAX_PRINT
    );
    // * Receive Data
    let (number_of_bytes, peer_host) = socket
        .recv_from(&mut main_buffer)
        .expect("no data received");
    let now = Instant::now();
    let received_bytes: Vec<u8> = main_buffer
        .iter()
        .take(number_of_bytes)
        // .filter(|&x| *x > 0)
        .map(|x| x.clone())
        .collect::<Vec<u8>>();

    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let hash = format!("{:x}", md5_compute(&received_bytes));

    println!(
        "{timestamp:?} | {local_host:>16} <- {peer_host:<16} | {elapsed:>12} | {length:>5} | {hash} ",
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
