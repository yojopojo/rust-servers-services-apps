use super::models::Course;
use std::sync::Mutex;

pub struct AppState {
    pub health_check_response: String, // 공유된 이뮤터블 상태
    pub visit_count: Mutex<u32>, // 공유된 뮤터블 상태
    pub courses: Mutex<Vec<Course>>, // 강의들을 Vec 컬렉션으로 애플리케이션 상태에 저장(Mutex로 보호됨)
}