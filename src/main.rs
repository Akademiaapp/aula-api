use actix_web::{App, HttpResponse, HttpServer, post, Responder};
use actix_web::web::Json;
use actix_cors::Cors;
use serde_derive::Deserialize;

use crate::aula_handler::{AulaSession, LoginInfo};

mod aula_handler;

mod unilogin;

mod request_structs;
mod response_structs;
mod tests;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let port = if args.len() > 1 {
        args[1].parse().unwrap_or(8080)
    } else {
        8080
    };
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::permissive() // This enables all CORS requests
            )
            .service(login)
            .service(get_events)
            .service(get_notifs)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(info: Json<LoginRequest>) -> impl Responder {
    println!("hi");
    let aula_session = AulaSession::from_credentials(&info.username, &info.password).await;

    HttpResponse::Ok().json(aula_session.get_login_info())
}

#[derive(Deserialize)]
struct EventRequest {
    login_info: LoginInfo,
    start: String,
    end: String,
}

#[post("/getCalenderEvents")]
async fn get_events(info: Json<EventRequest>) -> impl Responder {
    let aula_session = AulaSession::from_login_info(&info.login_info).await;
    let events = aula_session
        .request_events(info.start.to_string(), info.end.to_string())
        .await
        .unwrap();
    HttpResponse::Ok().json(events)
}

#[derive(Deserialize)]
struct Request {
    login_info: LoginInfo,
}

#[post("/getNotifications")]
async fn get_notifs(info: Json<Request>) -> impl Responder {

    let aula_session = AulaSession::from_login_info(&info.login_info).await;

    let url = format!("https://www.aula.dk/api/v18/?method=notifications.getNotificationsForActiveProfile&activeInstitutionCodes[]={}", aula_session.institution_code);

    let res = aula_session.request_get(url).await.unwrap();

    HttpResponse::Ok().json(res)
}

// #[get("/getMessages")]
// async fn get_messages(info: web::Query<HashMap<String, String>>) -> impl Responder {
//     let page = info.get("page").unwrap_or(&"0".to_string());
//
//     let aula_session = AulaSession::from_login_info(&info.into_inner()).await;
//
//     let url = format!("https://www.aula.dk/api/v18/?method=messaging.getMessages&page={}", page);
//
//     let res = aula_session.request_get(url).await.unwrap();
//
//     HttpResponse::Ok().json(res)
// }
