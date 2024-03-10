use actix_web::{App, get, HttpResponse, HttpServer, post, Responder};
use actix_web::web::Json;
use serde_derive::Deserialize;

mod aulaHandler;

mod unilogin;

mod response_structs;
mod request_structs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(login)

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

#[post("/login")]
async fn login(info: Json<LoginRequest>) -> impl Responder {
    println!("hi");
    let aula_session = aulaHandler::AulaSession::new(&info.username, &info.password).await;


    HttpResponse::Ok().json(aula_session.request_events("2024-03-10T15:06:09+00:00".to_string(), "2024-03-12T15:06:09+00:00".to_string()).await.unwrap())
}
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}