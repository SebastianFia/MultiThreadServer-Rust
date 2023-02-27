use std::net::TcpListener;

use multi_thread_server::{connection::handle_connections, thread_pool::ThreadPool};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut pool = ThreadPool::build(4).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let execute_result = pool.execute(move || handle_connections(stream));
        if let Err(e) = execute_result {
            eprintln!("Failed to execute stream: {e}");
        }
    }
}
