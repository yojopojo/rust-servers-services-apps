use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(
    Deserialize, Serialize, // Serde 크레이트의 일부, 데이터 구조체를 전송용 포맷으로(혹은 반대로) 변환하는 것을 도와줌
    Debug, // 디버깅 목적으로 Course 구조체 출력 가능
    Clone // 처리하는 동안 러스트의 소유권 규칙을 해결할 수 있음
)]
pub struct Course {
    pub tutor_id: i32,
    pub course_id: Option<i32>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>, // 타임스탬프 정보를 저장하는 Chrono 데이터 타입
}
impl From<web::Json<Course>> for Course { // 유입되는 HTTP 요청의 데이터를 러스트 구조체로 변환
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}