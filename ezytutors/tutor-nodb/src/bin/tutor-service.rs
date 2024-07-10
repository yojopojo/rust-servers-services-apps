use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../models.rs"]
mod models;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use routes::*;
use state::AppState;


#[actix_rt::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState { // 애플리케이션 상태 초기화
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![]), // Mutex로 보호된 빈 vector로 초기화
    });
    let app = move || { // 웹 애플리케이션 정의
        App::new()
            .app_data(shared_data.clone()) // 웹 애플리케이션 상태 등록
            .configure(general_routes)
            .configure(course_routes) // 새로운 course_routes 그룹을 애플리케이션에 등록
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await // 웹 애플리케이션과 함게 Actix 웹 서버 초기화
}