use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:5000").unwrap();
    stream.write_all("Hello".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).unwrap();
    println!("{:?}", str::from_utf8(&buffer).unwrap());
}
