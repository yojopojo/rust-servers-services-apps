use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // scope는 공통 루트 경로를 갖는 리소스 셋
        web::scope("/courses") // courses라 불리는 새로운 리소스 스코프를 만듦
            .route("/", web::post().to(new_course))
            .route("/{tutor_id}", web::get().to(get_courses_for_tutor)) // 책에는 tutor_id가 user_id로 표기되어있음
            .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail)),
    );
}