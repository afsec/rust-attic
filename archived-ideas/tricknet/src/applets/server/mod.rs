use crate::errors::TrickError;
use std::net::UdpSocket;
use std::time::Instant;
use std::time::{SystemTime, UNIX_EPOCH};
const PAYLOAD_MAX_PRINT: usize = 40;

pub fn decrypt_block(block: Vec<u8>) -> [u8; 8] {
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
    let mut decrypted_block: GenericArray<u8, U8> = *GenericArray::from_slice(&block);
    dbg!(&decrypted_block.len());
    des_engine.decrypt_block(&mut decrypted_block);
    *decrypted_block.as_ref()
}

pub fn start() -> Result<(), TrickError> {
    let start = SystemTime::now();
    // * Get host from env::args or use default host
    let host = "127.0.0.1:8000".to_owned();

    // * IP Header: "Total Length" has 16 bits
    // Total Length is the length of the datagram, measured in octets,
    //  including internet header and data.  This field allows the length of
    //  a datagram to be up to 65,535 octets.

    // * Set buffer to UDP field Length where max lentgh is 65507. Because:
    //  - UDP Headers is 8 octects
    //  - IP Headers is 20 bytes

    // * Open UDP socket
    println!("Starting server at [{}]:\n", &host);
    let socket = UdpSocket::bind(&host).expect("failed to bind host socket");

    println!(
        "      TIMESTAMP       |                 FLOW                 | ELAPSED (ms) | BYTES |           PAYLOAD (First {length} Bytes)",length=PAYLOAD_MAX_PRINT
    );
    // println!(        "----------------------+--------------------------------------+--------------+-------+--------------------------------------------"    );

    // * Threadpool?
    loop {
        let mut main_buffer = [0; 65507];
        // * Receive Data
        let (number_of_bytes, client_host) = socket.recv_from(&mut main_buffer).expect("no data received");
        let now = Instant::now();
        // println!("Client connected: {:?}", client_host);
        // println!("Received: {} Bytes.", number_of_bytes);

        let received_bytes: Vec<u8> = main_buffer
            .iter()
            .filter(|&x| *x > 0)
            .map(|x| x.clone())
            .collect::<Vec<u8>>();
        // println!("Received message: {}", message);
        // !
        let block_before_decryption: Vec<u8> = received_bytes
            .iter()
            .take(8)
            .map(|x| x.clone())
            .collect::<Vec<u8>>();
        dbg!(&main_buffer.len());
        dbg!(&block_before_decryption);

        let decrypted_block = decrypt_block(block_before_decryption);

        dbg!(&decrypted_block);

        let message_received = decrypted_block;
        // !
        let message_to_be_printed: Vec<u8> = message_received
            .iter()
            .take(PAYLOAD_MAX_PRINT)
            .map(|x| x.clone())
            .collect::<Vec<u8>>();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        print!(
            "{timestamp:?} | {host:>16} <- {client:<16} | {elapsed:>12} | {length:>5} | ",
            timestamp = since_the_epoch,
            host = host,
            client = client_host,
            elapsed = now.elapsed().as_millis(),
            length = number_of_bytes
        );
        let message = String::from_utf8(message_to_be_printed).unwrap_or("".to_owned());
        if received_bytes.len() > PAYLOAD_MAX_PRINT {
            // if message.ends_with('\n') {
            //     message.pop();
            // }
            println!(
                "{message:<length$}...",
                message = message,
                length = PAYLOAD_MAX_PRINT
            );
        } else {
            println!(
                "{message:<length$}",
                message = &message,
                length = PAYLOAD_MAX_PRINT
            );
        }

        // * Send Data
        let now = Instant::now();
        let sent_bytes_length = socket
            .send_to(&received_bytes, client_host)
            .expect("failed to send message");
        print!(
            "{timestamp:?} | {host:>16} -> {client:<16} | {elapsed:>12} | {length:>5} | ",
            timestamp = since_the_epoch,
            host = host,
            client = client_host,
            elapsed = now.elapsed().as_millis(),
            length = sent_bytes_length
        );
        if received_bytes.len() > PAYLOAD_MAX_PRINT {
            // if message.ends_with('\n') {
            //     message.pop();
            // }
            println!(
                "{message:<length$}...",
                message = message,
                length = PAYLOAD_MAX_PRINT
            );
        } else {
            println!(
                "{message:<length$}",
                message = &message,
                length = PAYLOAD_MAX_PRINT
            );
        }
    }

    // ? Control through unix socket
    // * Server, Monitor and cli
    // * See OpenBSD software achitecture like bgpd, smtpd, ldapd, etc.
}
