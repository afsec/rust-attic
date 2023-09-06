// * Quicknet Client

use crate::errors::TrickError;
use crate::udp::UDP_DATAGRAM_MAX_LENGTH;
use crate::udp::{PortChecker, RandomPort, UdpClient};

use std::fmt::Debug;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::mpsc::channel;
use std::thread;
use std::time::{Duration, Instant};

struct Socket;
impl UdpClient for Socket {}
impl RandomPort for Socket {}
impl PortChecker for Socket {}

fn check_max_lenth(bytes_to_be_sent: &[u8]) -> Result<(), TrickError> {
    if bytes_to_be_sent.len() > UDP_DATAGRAM_MAX_LENGTH {
        dbg!(bytes_to_be_sent);
        Err(TrickError::BufferOverflow)
    } else {
        Ok(())
    }
}

pub trait QuicknetClient {
    fn send_message<A: Debug + ToSocketAddrs>(
        buffer: &[u8],
        server_host: A,
    ) -> Result<(usize, SocketAddr, SocketAddr, Vec<u8>), TrickError> {
        let mut buffer_received: [u8; UDP_DATAGRAM_MAX_LENGTH] = [0; UDP_DATAGRAM_MAX_LENGTH];
        // * 1. Check message max length
        check_max_lenth(&buffer)?;

        // * 2. Open UDPSocket on selected Interface IP
        let socket = Socket.bind("0.0.0.0")?;
        // let socket = Socket.bind("172.17.42.12")?;
        let local_host = socket.local_addr()?;

        // * 3. Send Datagram
        let sent_bytes_length = socket.send_to(&buffer, &server_host)?;

        // * 4. Wait for Confirmation
        let (channel_sender, channel_receiver) = channel();
        thread::spawn(move || {
            let (received_bytes_length, peer_host) = socket
                .recv_from(&mut buffer_received)
                .expect("no data received");
            let buffer_to_return: Vec<u8> = buffer_received.to_vec();
            channel_sender.send((received_bytes_length, peer_host, buffer_to_return))
        });

        // ? 5. Return bytes_sent_length
        // let (received_bytes_length, peer_host) = channel_receiver.recv_timeout(Duration::from_millis(5000))?;
        match channel_receiver.recv_timeout(Duration::from_millis(200)) {
            Ok((received_bytes_length, peer_host, buffer_received)) => {
                if received_bytes_length == sent_bytes_length {
                    Ok((
                        received_bytes_length,
                        local_host,
                        peer_host,
                        buffer_received,
                    ))
                } else {
                    dbg!(received_bytes_length, sent_bytes_length);
                    Err(TrickError::DatagramSizeMismatch)
                }
            }
            Err(_) => Ok((0, local_host, local_host, Vec::new())),
        }
        // TODO:
        //       - Request
        //       - Response
        //       - CRC/Checksum/RAID UDPCast style
        //       - Retransmission
    }
}

// TODO: Check bind through server route
