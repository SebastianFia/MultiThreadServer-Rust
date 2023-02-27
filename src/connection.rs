use std::{
    fs,
    io::{prelude::*, BufReader},
    net::TcpStream,
    thread,
    time::Duration,
};

use crate::connection_errors::{BoxResult, EmptyRequestError, ResponseResult};

pub fn handle_connections(mut stream: TcpStream) -> BoxResult<()> {
    let buf_reader = BufReader::new(&mut stream);
    let response = get_response(buf_reader)?;
    stream.write_all(response.as_bytes())?;

    Ok(())
}

pub fn get_response(buf_reader: BufReader<&mut TcpStream>) -> ResponseResult<String> {
    let request_first_line = buf_reader.lines().next();

    let request_first_line = match request_first_line {
        None => Err(EmptyRequestError::new())?,
        Some(line) => line?,
    };

    let (filename, status_line) = match &request_first_line[..] {
        "GET / HTTP/1.1" => ("index.html", "HTTP/1.1 200 OK\r\n\r\n"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("index.html", "HTTP/1.1 200 OK\r\n")
        }
        _ => ("404.html", "HTTP/1.1 404 Not Found\r\n\r\n"),
    };
    let contents = fs::read_to_string(filename)?;

    Ok(format!("{status_line}\r\n\r\n{contents}"))
}
