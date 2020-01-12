use std::net::TcpListener;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

  println!("Listening for connection on localhost port 8000");

	for stream in listener.incoming() {
		let _stream = stream.unwrap();

		println!("Someone openned a website on port 8000!");
	}
}