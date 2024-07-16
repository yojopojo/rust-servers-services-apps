use super::models::Course;
use sqlx::postgres::PgPool;

// 한 강사의 모든 강의를 가져옴
pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {

    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id = $1",
        tutor_id
    )
    .fetch_all(pool) // Query 실행, CP를 매개변수로 받음
    .await // PostgreSQL DB를 비동기로 호출
    .unwrap();
    
    // 결과 추출
    course_rows
        .iter() // iterator에 의해 각 행을 반환
        .map(|course_row| Course { // 각 행을 Course 구조체로 변환
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())), // posted_time을 NaiveDateTime으로 변환
        })
        .collect() // 결과를 Vector로 변환
}

// 강의 하나의 상세 정보를 가져옴
pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 where tutor_id = $1 and course_id = $2",
        tutor_id, course_id
    )
    .fetch_one(pool) // fetch_all 이 아닌 fetch_one을 사용
    .await
    .unwrap();
    
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}

// 새로운 강의 등록
pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let course_row = sqlx::query!(
        "insert into ezy_course_c4 (course_id,tutor_id, course_name)  
                                  values ($1,$2,$3) returning 
                                  tutor_id, 
                                  course_id, 
                                  course_name,  
                                  posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool) // 강의 삽입 후 삽입된 강의를 꺼냄
    .await
    .unwrap();
    
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}