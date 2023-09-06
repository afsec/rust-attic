#![warn(clippy::all)]

// use env_logger;
use std::io::{stdout, Read, Write};
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::thread::sleep;
use std::time::Duration;
fn start_connection(domain_name: &str) {
    // let server_name: SocketAddr = domain_name.parse().expect("invalid DNS name");
    // let sock = SocketAddrV4::;
    let vec_sock_addr = domain_name
        .to_socket_addrs()
        .expect("Can't resolve to IP Address")
        .filter(|sock_addr| sock_addr.is_ipv4())
        .collect::<Vec<SocketAddr>>();
    if !vec_sock_addr.is_empty() {
        if let Some(sock_addr) = vec_sock_addr.first() {
            dbg!(&sock_addr);
            // addrs_iter.next()
            let mut sock = TcpStream::connect(sock_addr).unwrap();
            sock.set_nodelay(true).unwrap();

            let request = format!("\
            GET / HTTP/1.1\r\n\
            Host: {}\r\n\
            User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:94.0) Gecko/20100101 Firefox/94.0\r\n\
            Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8\r\n\
            Accept-Language: en-US,en;q=0.5\r\n\
            Accept-Encoding: gzip, deflate\r\n\
            DNT: 1\r\n\
            Connection: keep-alive\r\n\
            Upgrade-Insecure-Requests: 1\r\n\
            Sec-Fetch-Dest: document\r\n\
            Sec-Fetch-Mode: navigate\r\n\
            Sec-Fetch-Site: none\r\n\
            Sec-Fetch-User: ?1",
            domain_name);

            // If early data is available with this server, then early_data()
            // will yield Some(WriteEarlyData) and WriteEarlyData implements
            // io::Write.  Use this to send the request.

            // If we didn't send early data, or the server didn't accept it,
            // then send the request as normal.
            sock.write_all(request.as_bytes()).unwrap();

            let millis = Duration::from_millis(10);
            // TODO: Generates a "random sequence"
            // for i in 0..5 {
            for i in 0..u64::MAX {
                let header_name = format!("\r\nSec-Fetch-Labolicha-{}: ", i);

                if let Err(_) = sock.write(header_name.as_bytes()) {
                    panic!("Error: Unable do send new Header '{}'", header_name);
                }

                let mut header_value = vec![];

                for _ in 0..1 {
                    for c in 0..94 {
                        // let buf = [33u8 + c, 1];
                        // sock.write(&buf)
                        //     .expect(format!("Error: Unable do send byte '{:?}'", &buf).as_str());
                        // use std::thread::sleep;
                        // use std::time::Duration;
                        // let millis = Duration::from_millis(1);
                        // sleep(millis);

                        header_value.push(33 + c); // Char 32 + (0 ~ 93)
                    }
                }
                // println!("Sending {}, Bytes", header_value.len());
                if let Err(_) = sock.write(&header_value) {
                    panic!(
                        "Error: Unable do send Header value'{}'",
                        String::from_utf8_lossy(&header_value)
                    );
                }
                if i % 10_000 == 0 {
                    println!("10 ms..");
                    sleep(millis);
                }
            }
            // for _ in 0..100 {
            //     sock.write(&buf).expect("unable do send byte 'A'");
            //     sleep(millis);
            //     println!("A");
            // }
            sock.write_all("\r\n\r\n".as_bytes()).unwrap();

            let mut plaintext = Vec::new();
            sock.read_to_end(&mut plaintext).unwrap();
            stdout().write_all(&plaintext).unwrap();
        }
    }
}

fn main() {
    // env_logger::init();

    let host_name = "localhost:8000";
    // let host_name = "localhost:65432";
    start_connection(host_name);
}
