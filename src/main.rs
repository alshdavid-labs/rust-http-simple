use tokio::net::TcpListener;

use tokio::io::AsyncWriteExt;
use tokio::io::AsyncReadExt;


async fn main_async() {
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    println!("Listening for connections on port {}", 9000);

    loop {
      let (mut socket, _) = listener.accept().await.unwrap();

      tokio::spawn(async move {
          let mut buffer = [0; 1024];
          socket.read(&mut buffer).await.unwrap();
    
          let mut headers = [httparse::EMPTY_HEADER; 64];
          let mut request = httparse::Request::new(&mut headers);
    
          let status = request.parse(buffer.as_slice()).unwrap().unwrap();
          let body = &buffer[status..];
          let contents = std::str::from_utf8(body).unwrap();
    
          let status_line = "HTTP/1.1 200 OK";
          let length = contents.len();
    
          let response =
              format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
          socket.write_all(response.as_bytes()).await.unwrap();
      });
    }
}

fn main() {
  tokio::runtime::Builder::new_multi_thread()
    .worker_threads(num_cpus::get())
    // .worker_threads(4)
    .enable_all()
    .build()
    .unwrap()
    .block_on(main_async());
}