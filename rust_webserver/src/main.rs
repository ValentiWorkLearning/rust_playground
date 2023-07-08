use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use threadpool::ThreadPool;


fn handle_connection(mut tcp_stream:TcpStream){
    let buf_reader = BufReader::new(& mut tcp_stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Http request is: {:#?}", request_line);

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "/Users/valentynkorniienko/Documents/Development/rust_playground/rust_webserver/src/main.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "/Users/valentynkorniienko/Documents/Development/rust_playground/rust_webserver/src/main.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "/Users/valentynkorniienko/Documents/Development/rust_playground/rust_webserver/src/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    tcp_stream.write_all(response.as_bytes()).unwrap();
}
fn main() {
    let pool = ThreadPool::new(4);

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // listener.incoming().for_each(|x|{
    //     let stream = x.unwrap();
    //     println!("Connection established!");
    //     pool.execute(|| {
    //         handle_connection(stream);
    //     });
    // });
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}
