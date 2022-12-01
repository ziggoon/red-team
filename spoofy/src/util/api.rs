use hyper::service::{make_service_fn, service_fn};
use futures::TryStreamExt; // 0.3.7
use hyper::{server::Server, service, Body, Method, Request, Response}; // 0.13.9
use hyper::body;
use std::convert::Infallible;
use twilio::{Client, Message, OutboundMessage};
use rusqlite::Connection;

use dotenv;
use crate::util;
use std::thread;

pub async fn send(conn: &Connection, args: Vec<String>) {
    //println!("welcome to client::send()");
    let to = &args[1];
    let from = &args[2];
    let body = &args[3];
    let sid = dotenv::var("TWILIO_SID").expect("$TWILIO_SID is not set");
    let token = dotenv::var("TWILIO_TOKEN").expect("$TWILIO_TOKEN is not set");
    let client = Client::new(sid.as_str(), token.as_str());
    let msg = OutboundMessage::new(from, to, body);
    
    println!("TO:{} FROM:{} BODY:{}", to, from, body);
    match client.send_message(msg).await {
        Ok(m) => {
            println!("{:?}", m);
            util::db::insert_message(&conn, args).await.unwrap();
        },
        Err(e) => eprintln!("{:?}", e)
    }
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let sid = dotenv::var("TWILIO_SID").expect("$TWILIO_SID is not set");
    let token = dotenv::var("TWILIO_TOKEN").expect("$TWILIO_TOKEN is not set");
    let client = twilio::Client::new(sid.as_str(), token.as_str());

    let cloned_uri = req.uri().clone();
    println!("Got a request for: {}", cloned_uri);
    
    let bytes = body::to_bytes(req.into_body()).await?;
    let bod = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");
    
    let split: Vec<&str> = bod.split(|c| c == '&' || c == '=').collect();
    let args: Vec<String> = vec!["add".to_string(),split[25].to_string(), split[37].to_string(), split[21].to_string()];
    let conn = Connection::open("db.db").expect("connection failed");
    conn.execute(
        "insert into messages (number_to, number_from, msg_body) values (?1, ?2, ?3)",
        &[args[1].as_str(), args[2].as_str(), args[3].as_str()],
    ).expect("insert failed");
    Ok(Response::new(Body::from(bod)))
}

#[tokio::main]
pub async fn main() {
    let addr = "0.0.0.0:3000".parse().expect("Unable to parse address");

    let server = Server::bind(&addr).serve(service::make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service::service_fn(handle_request))
    }));

    println!("Listening on http://{}.", server.local_addr());

    if let Err(e) = server.await {
        eprintln!("Error: {}", e);
    }
}
