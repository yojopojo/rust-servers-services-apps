use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// 라우트 구성
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // '/health' 경로로 유입되는 GET 요청을 health_check_handler라는 핸들러로 전달함
    cfg.route("/health", web::get().to(health_check_handler));
}

// 핸들러 구성
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

// HTTP 서버를 인스턴스화하고 실행
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // App 생성, 설정된 경로에 등록
    let app = move || App::new().configure(general_routes);

    // 웹 서버 초기화, 애플리케이션 로드, 소켓에 바인딩 후 서버 실행
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
} 