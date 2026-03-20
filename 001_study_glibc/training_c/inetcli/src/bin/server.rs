use std::io::Read;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5555").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buf = [0u8; 1024];

        let n = stream.read(&mut buf).unwrap();

        println!("{}", String::from_utf8_lossy(&buf[..n]));
    }
}
