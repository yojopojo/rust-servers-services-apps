use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok(); // 환경변수 로드

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap(); // CP 생성

    // App 상태 구현
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    // App 구성, 라우트 구성
    let app = move || {
        App::new()
            .app_data(shared_data.clone()) // CP를 크로스-애플리케이션 디펜던시로 주입
            .configure(general_routes)
            .configure(course_routes)
    };

    let hostname_port = env::var("SERVER_HOSTNAME_PORT").expect("SERVER_HOSTNAME_PORT is not set in .env file");

    // HTTP 서버를 시작한다
    HttpServer::new(app).bind(hostname_port).unwrap().run().await

}