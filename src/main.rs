use actix::{Actor, StreamHandler};
use actix_web::{web, App, HttpServer, HttpResponse};
use actix_web_actors::ws;
use actix_web::HttpRequest;

async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("index.html"))
}

async fn ws_index(ws: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(MyWebSocket {}, &ws, stream);
    println!("{:?}", resp);
    resp.map_err(|e| {
        eprintln!("Error starting WebSocket: {}", e);
        actix_web::error::ErrorInternalServerError("WebSocket start error")
    })
}

struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                println!("Received message: {}", text);
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/ws/", web::get().to(ws_index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

