use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use learn::ThreadPool;

mod basic;
use basic::learn_mod;


fn main() {
   // learn_mod::learn_main();

    // let listener = TcpListener::bind("0.0.0.0:8089").unwrap();
    // for stream in listener.incoming(){
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    // }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let contents = fs::read_to_string("hello.html").unwrap();
    let content_type = "text/plain";

    println!("connection established!");
    let response =format!(
     "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
    contents.len(),
    content_type,
    contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}