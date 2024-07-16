use dotenv::dotenv;
use std::env;
use std::io;
use sqlx::postgres::PgPool;
use chrono::NaiveDateTime;

// 구조체 정의
#[derive(Debug)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// 비동기 웹 서버 실행, sqlx를 사용해 DB와 연결
#[actix_rt::main]
async fn main() -> io::Result<()> {

    // 환경변수를 메모리에 로드
    dotenv().ok();

    // 환경변수 사용
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    // 커넥션 풀 생성
    let db_pool = PgPool::connect(&database_url).await.unwrap();

    // Query 정의
    let course_rows = sqlx::query!(
        r#"select course_id, tutor_id, course_name, posted_time from ezy_course_c4 where course_id = $1"#, 1
    )
    .fetch_all(&db_pool) // 커넥션 풀의 참조를 전달해서 테이블의 모든 행을 가져옴
    .await
    .unwrap();
    let mut courses_list = vec![];
    for course_row in course_rows {
        courses_list.push(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name,
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
    } 
    println!("Courses = {:?}", courses_list);
    Ok(())
}