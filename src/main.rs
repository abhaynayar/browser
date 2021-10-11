use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use std::collections::HashMap;

fn main() {
    let (x,y) = request("http://example.org/index.html");
    println!("{:?}, {}", x, y);
}

fn request(mut url: &str) -> (HashMap<String, String>, String) {
    // Parse URL:
    assert!(url.starts_with("http://"));
    url = &url["http://".len()..];

    let host = url.splitn(2,"/").nth(0).unwrap();
    let _path = format!("/{}", url.splitn(2,"/").nth(1).unwrap());

    // Send HTTP request:
    let mut stream = TcpStream::connect(host.to_owned() + ":80").unwrap();
    let msg = b"GET /index.html HTTP/1.0\r\nHost: example.org\r\n\r\n";
    stream.write(msg).unwrap();

    // Read HTTP response:
    let mut response = "";
    let mut data = [0 as u8; 1024];
    match stream.read_exact(&mut data) {
        Ok(_) => { response = from_utf8(&data).unwrap(); },
        Err(e) => { println!("Failed to receive data: {}", e); }
    }

    let mut response_lines = response.lines();
    
    // Parse HTTP response:
    let status_line = response_lines.next().unwrap();
    let status = status_line.split(" ").nth(1).unwrap();
    let _version = status_line.split(" ").nth(0).unwrap();
    let _explanation = status_line.split(" ").nth(2).unwrap();
    assert!(status=="200");

    // Parse HTTP headers:
    let mut headers = HashMap::new();

    loop {
        let line = response_lines.next().unwrap();
        if line=="" { break; }
        let header = line.split(":").nth(0).unwrap();
        let value = line.split(":").nth(1).unwrap();
        headers.insert(header.to_lowercase(), value.trim().to_string());
    }

    // Extract HTTP body:
    let body = response_lines.fold(String::new(), |a,b| a+b+"\n");
    return (headers, body);
}
