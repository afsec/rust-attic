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

pub const UDP_DATAGRAM_MAX_LENGTH: usize = 65507;

use nanorand::{WyRand, RNG};
use std::fmt::Debug;
use std::io;
use std::net::{ToSocketAddrs, UdpSocket};

pub trait RandomPort {
    fn random_port(&self) -> u16 {
        let mut rng = WyRand::new();
        rng.generate_range::<u16>(10000, u16::MAX)
    }
}

pub trait PortChecker {
    fn check_available<A: ToSocketAddrs + Debug>(&self, addr: A) -> io::Result<UdpSocket> {
        UdpSocket::bind(&addr)
    }
}

pub trait UdpServer {
    fn bind(&self, address: &str, port: u16) -> io::Result<UdpSocket>
    where
        Self: PortChecker,
    {
        let first_try = format!(
            "{local_address}:{local_port}",
            local_address = address,
            local_port = port
        );
        match self.check_available(&first_try) {
            Ok(socket) => Ok(socket),
            Err(_) => {
                let second_try = format!(
                    "{local_address}:{local_port}",
                    local_address = address,
                    local_port = port + 1
                );
                dbg!(&second_try);
                Ok(UdpSocket::bind(&second_try)?)
            }
        }
    }
}

pub trait UdpClient {
    fn bind(&self, address: &str) -> io::Result<UdpSocket>
    where
        Self: RandomPort + PortChecker,
    {
        let mut selected_port = self.random_port();
        loop {
            let bind_address = format!(
                "{local_address}:{local_port}",
                local_address = address,
                local_port = selected_port
            );
            match self.check_available(&bind_address) {
                Ok(socket) => return Ok(socket),
                Err(_) => {
                    selected_port += 1;
                    continue;
                }
            }
        }
    }
    // fn send_to(&bytes_to_be_sent, &server_host);
    // fn recv_from(&mut main_buffer);
}
