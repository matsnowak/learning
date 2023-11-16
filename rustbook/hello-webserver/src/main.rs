use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();
        handle_connection(_stream);
        println!("Connection established!");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buff_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let lenght = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {lenght}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request);
}
