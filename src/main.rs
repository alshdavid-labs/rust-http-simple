use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

async fn main_async() {
    let listener = TcpListener::bind("0.0.0.0:9000").unwrap();
    println!("Listening for connections on port {}", 8080);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut request = httparse::Request::new(&mut headers);

        let status = request.parse(buffer.as_slice()).unwrap().unwrap();
        let body = &buffer[status..];
        let contents = std::str::from_utf8(body).unwrap();

        let status_line = "HTTP/1.1 200 OK";
        let length = contents.len();

        let response =
            format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn main() {
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    // .worker_threads(threads as usize)
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async());
}