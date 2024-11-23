mod http_parser;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use threadpool::ThreadPool;

use futures::task::SpawnExt;

fn main() {
    println!("Starting basic http server....");
    let listener = TcpListener::bind(("0.0.0.0", 8080)).unwrap();
    println!("Server is running on localhost:8080");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_request(stream);
        });
    }
}


fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer);

    http_parser::parse(request.into_owned());

    let response = format!("HTTP/1.1 200 OK\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}