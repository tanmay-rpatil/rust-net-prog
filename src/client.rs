use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let get_req = b"GET / HTTP/1.1\r\n";

            stream.write(get_req).unwrap();
            println!("Sent get req, awaiting reply...");

            let mut data = [0 as u8; 1024]; // using 6 byte buffer
            match stream.read(&mut data) {
                Ok(ret) => {
					let text = from_utf8(&data).unwrap();
					println!("rcvd reply: of size {} \n: {}",ret, text);
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}