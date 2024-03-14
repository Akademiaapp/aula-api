use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::Json;
use serde_derive::{Deserialize, Serialize};

mod aulaHandler;

mod unilogin;

mod response_structs;
mod request_structs;
mod util;
mod tests;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(login)
            .service(get_events)
            .service(get_notifs)

    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[get("/login")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    token: String,
    php_session: String,
}

#[post("/login")]
async fn login(info: Json<LoginRequest>) -> impl Responder {
    println!("hi");
    let aula_session = aulaHandler::AulaSession::from_credentials(&info.username, &info.password).await;

    HttpResponse::Ok().json(LoginInfo { token: aula_session.token, php_session: aula_session.php_session })
}

#[derive(Deserialize)]
struct EventRequest {
    login_info: LoginInfo,
    start: String,
    end: String
}

#[post("/getCalenderEvents")]
async fn get_events(info: Json<EventRequest>) -> impl Responder {
    let aula_session = aulaHandler::AulaSession::from_cookies(info.login_info.token.clone(), info.login_info.php_session.clone()).await;
    let events = aula_session.request_events(info.start.to_string(), info.end.to_string()).await.unwrap();
    HttpResponse::Ok().json(events)
}

#[get("/getNotifications")]
pub async fn get_notifs(info: Json<LoginInfo>) -> impl Responder {
    let aula_session = aulaHandler::AulaSession::from_cookies(info.token, info.php_session).await;

    let url = format!("https://www.aula.dk/api/v18/?method=notifications.getNotificationsForActiveProfile&activeInstitutionCodes[]={}", aula_session.institution_code);

    let res = aula_session.request_get(url).await.unwrap();

    HttpResponse::Ok().json(res)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}