use dotenv;
use twilio::{Client, OutboundMessage};

pub async fn send(args: Vec<String>) {
    //println!("welcome to client::send()");
    let to = &args[1];
    let from = &args[2];
    let body = &args[3];
    let sid = dotenv::var("TWILIO_SID").expect("$TWILIO_SID is not set");
    let token = dotenv::var("TWILIO_TOKEN").expect("$TWILIO_TOKEN is not set");
    let client = Client::new(sid.as_str(), token.as_str());
    let msg = OutboundMessage::new(from, to, body);
    
    println!("{} {}", to, from);
    match client.send_message(msg).await {
        Ok(m) => println!("{:?}", m),
        Err(e) => eprintln!("{:?}", e),
    }
}