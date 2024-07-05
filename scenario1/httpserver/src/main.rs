mod handler;
mod router;
mod server;
use server::Server;

fn main() {
    // 서버 시작
    let server = Server::new("localhost:3000");

    // 서버 실행
    server.run();
}