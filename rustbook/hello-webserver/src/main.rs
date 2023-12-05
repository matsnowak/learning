use hello_webserver::ThreadPool;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let mut _stream = stream.unwrap();

        pool.execute(|| handle_connection(_stream));
    }
    println!("Shutting down")
}

fn handle_connection(mut stream: TcpStream) {
    let buff_reader = BufReader::new(&mut stream);
    let http_request = buff_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &http_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let lenght = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {lenght}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request);
}
