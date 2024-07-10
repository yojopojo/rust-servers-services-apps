use super::state::AppState;
use actix_web::{web, HttpResponse};

use super::models::Course;
use chrono::Utc;

// Actix 웹 애플리케이션에 등록된 애플리케이션 상태는 자동으로 모든 핸들러 함수들이
// web::Data<T> 라는 추출자 객체(extractor object)를 사용해 접근할 수 있음
pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {

    let health_check_response = &app_state.health_check_response;

    // 공유된 뮤터블 상태를 나타내는 필드(visit_count)는 접근하기 전에 먼저 잠겨야 함
    // -> 여러 스레드가 동시에 업데이트할 수 없도록 함
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);

    // 공유된 뮤터블 상태를 나타내는 필드값을 업데이트
    // lock은 핸들러 함수 실행이 종료되는 즉시 반환됨
    *visit_count += 1;

    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {

    println!("Received new course");
    let course_count_for_user = app_state
        .courses
        .lock() // courses 컬렉션이 Mutex로 보호되므로, 데이터에 접근하기 위해 먼저 잠가야 함
        .unwrap()
        .clone()
        .into_iter() // 컬렉션을 이터레이터로 변환함
        .filter(|course| course.tutor_id == new_course.tutor_id)
        // .collect::<Vec<Course>>()
        // .len();
        .count();

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some((course_count_for_user + 1).try_into().unwrap()),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };

    // 새로운 강의 인스턴스를 강의 컬렉션에 추가
    app_state.courses.lock().unwrap().push(new_course);

    HttpResponse::Ok().json("Added course")
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> HttpResponse {
    // let tutor_id: i32 = params.0;
    let tutor_id: i32 = params.into_inner();

    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 { // 강사의 강의를 찾은 경우
        HttpResponse::Ok().json(filtered_courses)
    } else { // 강사의 강의를 찾지 못한 경우
        HttpResponse::Ok().json("No courses found for tutor".to_string())
    }
}

pub async fn get_course_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> HttpResponse {
    // let (tutor_id, course_id) = params.0;
    let (tutor_id, course_id) = params.into_inner();
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.tutor_id == tutor_id && x.course_id == Some(course_id))
        .ok_or("Course not found"); // Option<T>를 Result<T, E>로 변환
    // Some(val)이면 Ok(val)를, 아니면 Err(err)를 리턴함

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course not found".to_string())
    }
}

#[cfg(test)] // 테스트 환경에서만 실행됨(cargo build 혹은 run에서는 실행되지 않음)
mod tests { // 테스트는 tests 모듈에 작성
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test] // Actix Web의 비동기 런타임이 비동기 테스트 함수를 실행하도록 지정
    async fn post_course_test() {
        
        // 요청 데이터 페이로드 생성
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is a test course".into(),
            course_id: None,
            posted_time: None,
        });

        // 애플리케이션 상태 객체 생성
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        // 애플리케이션 상태 및 시뮬레이션된 요청 데이터 페이로드와 함께 핸들러 함수 호출
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success() {

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let tutor_id: web::Path<i32> = web::Path::from(1);

        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_course_detail(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}