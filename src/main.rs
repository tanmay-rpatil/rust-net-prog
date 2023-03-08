use std::fs;
// use std::str;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
	let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		println!("Connection est!");
		handle_conn(stream);
	}
}

fn handle_conn(mut stream: TcpStream) {
	let mut buffer = [0;1024];

	stream.read(&mut buffer).unwrap();
	// println!(
	// 	"Request: {}",
	// 	String::from_utf8_lossy(&buffer[..])
	// );
	
	let get = b"GET / HTTP/1.1\r\n";
	// println!("{:?}",str::from_utf8(&buffer));

	let mut resp= String::from("");
	let mut fname= String::from("");
	if buffer.starts_with(get){
		print!("valid get req\n");
		resp.push_str("HTTP/1.1 200 OK");
		fname.push_str("static/index.html");
	} else {
		print!("invalid get req\n");
		resp.push_str("HTTP/1.1 405 NOT ALLOWED\r\nAllow: \"GET\"");
		fname.push_str("static/405.html");
	}

	let contents = fs::read_to_string(fname).unwrap();

	let response = format!(
		"{}\r\nContent-Length: {}\r\n\r\n{}", 
		resp,
		contents.len(),
		contents
	) ;
	stream.write(response.as_bytes()).unwrap();
	stream.flush().unwrap();
}