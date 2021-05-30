use std::{
    fs,
    io::{Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
    thread, time,
};
use ThreadPool::ThreadPool;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let (content, start_line) = if buffer.starts_with(get) {
        (
            fs::read_to_string("main.html").unwrap(),
            "HTTP/1.1 200 OK\r\n\r\n",
        )
    } else {
        (
            fs::read_to_string("404.html").unwrap(),
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
        )
    };
    let response = format!("{}{}", start_line, content);
    stream.write(response.as_bytes()).unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    stream.flush().unwrap();
    // multi-threads test
    let ten_mills = time::Duration::from_millis(10000);
    thread::sleep(ten_mills);
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let poll = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream?;
        poll.execute(|| handle_client(stream));
    }

    Ok(())
}
