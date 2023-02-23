use std::{
    fs,
    net::TcpStream,
    time::Duration,
    io::{prelude::*, BufReader},
    thread,
};

pub fn handle_connections(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let response = get_response(buf_reader);

    stream.write_all(response.as_bytes()).unwrap();
}

pub fn get_response(buf_reader: BufReader<&mut TcpStream>) -> String {
    let request_line = buf_reader.lines().next()
        .expect("got empty request")
        .unwrap();

    let (filename, status_line) = match &request_line[..] {
        "GET / HTTP/1.1" => ("index.html", "HTTP/1.1 200 OK\r\n\r\n"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("index.html", "HTTP/1.1 200 OK\r\n")
        }
        _ => ("404.html", "HTTP/1.1 404 Not Found\r\n\r\n"),
    };

    let contents = fs::read_to_string(filename).unwrap();

    format!("{status_line}\r\n\r\n{contents}")
}