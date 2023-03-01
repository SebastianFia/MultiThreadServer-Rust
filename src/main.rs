use multi_thread_server::{connection::handle_connections, thread_pool::ThreadPool};
use std::net::TcpListener;

fn main() {
    let port = "127.0.0.1:7878";
    let listener = TcpListener::bind(port).unwrap();
    let mut pool = ThreadPool::build(4).unwrap();
    println!("Server running on port: {port}\n");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let execute_result = pool.execute(move || handle_connections(stream));
        if let Err(e) = execute_result {
            eprintln!("Failed to execute stream: {e}");
        }
    }
}
