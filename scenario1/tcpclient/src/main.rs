use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    // TCP 클라이언트는 localhost:3000에서 실행중인 원격 서버에 커넥션을 초기화
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    // Hello 라는 메시지를 TCP 서버 커넥션에 작성
    stream.write("Hello".as_bytes()).unwrap();

    let mut buffer = [0; 5];

    // 서버로부터 수신된 바이트 읽음
    stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );

}
