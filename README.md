# MultiThreadServer-Rust

A simple web server written using only rust std library.
Uses multi-threading to handle multiple requests concurrently.

It's a project from chap 20 of the rust by example book 
https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html

but with proper error handling and testing.

The thread_pool.rs and thread_pool_errors.rs files are indipendent from the rest of the project, so they can also be used (together) for other purposes.
