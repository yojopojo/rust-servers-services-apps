use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self { // Self라고 쓰면 자기 자신의 타입 리턴, 특정 타입을 지정하면 해당 타입 리턴
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = "";

        // 유입되는 HTTP 요청에서 각 행을 읽음
        for line in req.lines() {

            // 읽은 행이 request 행이면 process_req_line() 호출
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_version = version;
                parsed_resource = resource;

            // 읽은 행이 header 행이면 process_header_line() 호출
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            
            // 빈 행이면 아무것도 안 함
            } else if line.len() == 0 {
            
            // 전부 아니면 메시지 바디로 취급
            } else {
                parsed_msg_body = line;
            }
        }

        // 유입되는 HTTP 요청을 HttpRequest 구조체(struct)로 파싱
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            msg_body: parsed_msg_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {

    // 요청 행을 공백으로 구분된 개별 덩어리로 파싱
    let mut words = s.split_whitespace();

    // 요청 행의 첫 번째 부분에서 HTTP 메서드 추출
    let method = words.next().unwrap();

    // 요청 행의 두 번째 부분에서 리소스(URI/URL)을 추출
    let resource = words.next().unwrap();

    // 요청 행의 세 번째 부분에서 HTTP 버전을 추출
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {

    // 구분자(':')로 나누어진 단어들로 행을 파싱
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    // 헤더의 key 부분을 추출
    if let Some(k) = header_items.next() {
        key = k.to_string();
    }

    // 헤더의 value 부분을 추출
    if let Some(v) = header_items.next() {
        value = v.to_string()
    }

    (key, value)
}

#[derive(Debug, PartialEq)] // Debug는 toString(), PartialEq는 equals()와 유사
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

// From 트레이트를 구현하면 타입을 쉽게 변환할 수 있음
impl From<&str> for Method {
    fn from(s: &str) -> Method { // 타입 변환하는 메서드(다른 타입 -> 현재 타입)
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized, // 기타(와일드카드 패턴)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Version {
        match s {
            "HTTP/1.1" => Version::V1_1,
            _ => Version::Uninitialized,
        }
    }
}

#[cfg(test)] // 변환이 잘 되는지 테스트
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into(); // 타입 변환하는 메서드(현재 타입 -> 다른 타입)
        assert_eq!(m, Method::Get);
    }
    #[test]
    fn test_version_into() {
        let m: Version = "HTTP/1.1".into();
        assert_eq!(m, Version::V1_1);
    }
    #[test]
    fn test_read_http() {
        // 유입되는 HTTP 요청
        let s: String 
            = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");

        // 헤더 구성
        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        // 구조체 파싱(into 메서드로 타입 변경)
        let req: HttpRequest = s.into();
        
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}