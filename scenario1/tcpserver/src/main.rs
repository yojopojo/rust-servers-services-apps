use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 서버를 초기화해서 IP주소(localhost)와 포트(3000)에 바인딩
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    // unwrap()은 Optional.get() + 예외처리

    println!("Running on port 3000");

    for stream in connection_listener.incoming() { // 유입되는 커넥션 기다림(listen)

        // // 새로운 커넥션이 유입됨
        // let _stream = stream.unwrap(); // unwrap 성공 시 TcpStream 리턴, 오류 발생 시 패닉과 함께 프로그램 종료

        // 스트림을 뮤터블로 만들어서 읽고 쓸 수 있도록 함
        let mut stream = stream.unwrap();
        println!("Connection established");

        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap(); // 유입되는 스트림에서 읽기
        stream.write(&mut buffer).unwrap(); // 받은 데이터를 같은 커넥션을 통해 클라이언트에게 전송
    }
}
