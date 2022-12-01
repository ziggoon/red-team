use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response};
use std::convert::Infallible;
use std::net::SocketAddr;
use twilio::twiml::Twiml;
use twilio::{Client, OutboundMessage};
use rusqlite::Connection;

use dotenv;
use crate::util;

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

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let sid = dotenv::var("TWILIO_SID").expect("$TWILIO_SID is not set");
    let token = dotenv::var("TWILIO_TOKEN").expect("$TWILIO_TOKEN is not set");
    let client = twilio::Client::new(sid.as_str(), token.as_str());

    let cloned_uri = req.uri().clone();
    println!("Got a request for: {}", cloned_uri);

    let response = match cloned_uri.path() {
        "/sms" => {
            client
                .respond_to_webhook(req, |msg: twilio::Message| {
                    let mut t = Twiml::new();
                    t.add(&twilio::twiml::Message {
                        txt: format!("You told me: '{}'", msg.body.unwrap()),
                    });
                    t
                })
                .await
        }
        _ => panic!("Hit an unknown path."),
    };

    Ok(response)
}

#[tokio::main]
pub async fn main() {
    let addr: SocketAddr = "0.0.0.0:3000".parse().unwrap();
    let make_service = make_service_fn(|_| async { Ok::<_, Infallible>(service_fn(handle)) });
    let server = hyper::Server::bind(&addr).serve(make_service);
    println!("Listening on http://{}", addr);
    server.await.unwrap();
}