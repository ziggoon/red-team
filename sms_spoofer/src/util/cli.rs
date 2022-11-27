use rustyline::error::ReadlineError;
use rustyline::Editor;
use rusqlite::{Connection, Result};

use crate::util;

fn banner() {
    let banner = r#"
        .-')       _ (`-.                                                     
        ( OO ).    ( (OO  )                                                    
       (_)---\_)  _.`     \  .-'),-----.  .-'),-----.    ,------.   ,--.   ,--.
       /    _ |  (__...--'' ( OO'  .-.  '( OO'  .-.  '('-| _.---'    \  `.'  / 
       \  :` `.   |  /  | | /   |  | |  |/   |  | |  |(OO|(_\      .-')     /  
        '..`''.)  |  |_.' | \_) |  |\|  |\_) |  |\|  |/  |  '--.  (OO  \   /   
       .-._)   \  |  .___.'   \ |  | |  |  \ |  | |  |\_)|  .--'   |   /  /\_  
       \       /  |  |         `'  '-'  '   `'  '-'  '  \|  |_)    `-./  /.__) 
        `-----'   `--'           `-----'      `-----'    `--'        `--'     "#;
    println!("{}", banner);
}

fn desc() {
    let desc = r#"
    [+] welcome to Spoofy, a command line tool to send SMS messages using Twilio
    [+] requires a valid Twilio account / API key with active DID phone numbers

                        *usage* : send <to> <from> <body>
                    for additional cmd information type help"#;
    println!("{}\n", desc);
}

fn main_help() {
    let help = r#"
      send          sends new message
                    usage: send <to> <from> <body>"
      help          this page lol
      quit          exits the program"#;
    println!("{}", help);
}

async fn check_db() -> Result<()> {
    let conn = Connection::open("db.db")?;
    conn.execute(
        "create table if not exists numbers (
            id integer primary key autoincrement,
            number text not null unique
        )",
        [],
    )?;
    conn.execute(
        "create table if not exists messages (
            id integer primary key autoincrement,
            number_to text not null,
            number_from text not null,
            msg_body text not null  
        )",
        [],
    )?;
    Ok(())
}

async fn inset_number(args: Vec<String>) {
    let conn = Connection::open("db.db").expect("connection failed");
    conn.execute(
        "insert into messages (number_to, number_from, msg_body) values (?1, ?2, ?3)",
        &[args[1].as_str(), args[2].as_str(), args[3].as_str()],
    ).expect("insert failed");
}

// python readline() equivalent
fn cli_line(prompt: &str) -> Vec<String> {
    use std::io::{stdin, stdout, Write};
    print!("{}", prompt);
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    if s.is_empty() {
        return vec![String::from("")];
    }
    get_string_vec(s)
}

fn get_string_vec(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![String::from("")];
    }
    s.split_whitespace().map(str::to_string).collect()
}

pub async fn main_loop() {
    banner();
    desc();
    check_db().await;
    let mut user_input: Vec<String>;
    let mut rl = Editor::<()>::new();
    if rl.load_history(".history").is_err() {
           println!("no previous history...");
    }    
    loop {
        let readline = rl.readline("spoofy# ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "send" => util::client::send(user_input).await,
                    "help" => main_help(),
                    "test" => inset_number(user_input).await,
                    "quit" => std::process::exit(0),
                    _ => continue,
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c pressed. quitting now..");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("ctrl+d pressed. quitting now..");
                break
            },
            Err(err) => {
                println!("error: {:?}", err);
                break
            }
        } 
    }
    rl.save_history(".history").unwrap();
}