use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;

        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");
                
                tokio::spawn(async move {
                    let (mut read_stream, mut write_stream) = stream.split();

                    let buf_reader = BufReader::new(&mut read_stream);
                    let mut lines = buf_reader.lines();
                    
                    while let Some(line) = lines.next_line().await.unwrap() {
                        if line == "PING" {
                            write_stream.write_all(b"+PONG\r\n").await.unwrap();
                        }
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
