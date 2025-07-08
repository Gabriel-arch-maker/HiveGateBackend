use actix::{Actor, StreamHandler};
use actix_web::{get, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use sea_orm::sea_query::ExprTrait;
use serde::{Serialize, Deserialize};
use serde_json;
pub struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}

#[derive(Debug, Deserialize)]
struct IncomingMessage {
    action: String,
    payLoad: Option<String>
}

#[derive(Debug, Serialize)]
struct ResponseMessage {
    status: String,
    message: String,
}

impl StreamHandler<actix_web::Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: actix_web::Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<IncomingMessage>(&text) {
                    Ok(parsed) => {
                        let response = ResponseMessage {
                            status: "success".into(),
                            message: format!("Received message: {}", parsed.action)
                        };
                        let json_response = serde_json::to_string(&response).unwrap();
                        ctx.text(json_response);
                    }
                    Err(err) => {
                        ctx.text(format!("Invalid message: {}", err));
                    }
                }
                
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => {
                println!("Connection closed: {:?}", reason)
            },
            _ => ()
        }
    }
}

#[get("/ws/")]
pub async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error>
{
    ws::start(MyWebSocket {}, &req, stream)
}

pub fn init_ws_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(ws_index);
}