use tokio;
use log;
use std::error::Error;
use env_logger;
use axum::{
    Router, body::Body, http::{StatusCode, header::HeaderMap}, response::IntoResponse, routing::{get, post}

};


pub mod pingVerifier;
use pingVerifier::PingVerifier;

struct Campaign {
    active: bool,
    players: Vec<Player>,
    channel_id: String,
    
}

struct Player {
    id: String,
    health: u8,
}
struct AppState {
    campaigns: Vec<String>
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    log::info!("starting discord dm bot");

    let app = Router::new()
        .route("/", post(pong));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn pong(header: HeaderMap, body: String) -> impl IntoResponse {

    let pv: PingVerifier = PingVerifier::new();

    log::info!("this is the headers {:?}", header);
    
    let sig = match header.get("X-Signature-Ed25519") {
        Some(s) => match s.to_str() {
            Ok(ss) => ss,
            Err(e) => {
                log::error!("could not convert X-Signature-Ed25519 to String\n{}", e);
                return (StatusCode::UNAUTHORIZED, "\"type\": 1")
            }
        },
        None => {
            log::debug!("sig sign not found");
            return (StatusCode::UNAUTHORIZED, "\"type\": 1")
        }
    };

    let time_stamp = match header.get("X-Signature-Timestamp") {
        Some(t) => match t.to_str() {
            Ok(ts) => ts,
            Err(e) => {
                log::error!("could not convert X-Signature-Timestamp to String\n{}", e);
                return (StatusCode::UNAUTHORIZED, "\"type\": 1")
            }
        },
        None => {
            log::debug!("time time not found");
            return (StatusCode::UNAUTHORIZED, "\"type\": 1")
        }
    };

    let payload = format!("{}{}",body, time_stamp );

    match pv.verify(&payload, &sig) {
        true => (StatusCode::OK, "\"type\": 1"),
        false => (StatusCode::UNAUTHORIZED, "\"type\": 1") 
    }
}


async fn init() {

}

async fn start() {

}

async fn join() {
    
}

async fn action() {
    
}
