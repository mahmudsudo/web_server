use std::{io::prelude::*,
net::{TcpListener,TcpStream},fs::File};
fn main() {
     let listener = TcpListener::bind("127.0.0.1:80").unwrap();
    let me = listener.accept().unwrap();

    }