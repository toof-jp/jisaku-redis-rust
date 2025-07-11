#![allow(unused_imports)]
use std::{io::{BufRead, BufReader, Write}, net::TcpListener};

use bytes::buf;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let reader_stream = stream.try_clone().unwrap();
                let buf_reader = BufReader::new(reader_stream);
                
                for line in buf_reader.lines() {
                    if let Ok(s) = line && s == "PING" {
                        stream.write_all(b"+PONG\r\n").unwrap();
                    }
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
