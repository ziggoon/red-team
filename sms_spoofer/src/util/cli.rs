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
    [+] requires a valid Twilio account / API key with active DID phone numbers"#;
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

fn get_string_vec(s: String) -> Vec<String> {
    if s.is_empty() {
        return vec![String::from("")];
    }
    s.split_whitespace().map(str::to_string).collect()
}

pub async fn main_loop() {
    banner();
    desc();
    
    let conn = Connection::open("db.db").expect("connection failed");
    util::db::check_db(&conn).await;

    let mut user_input: Vec<String>;
    let mut rl = Editor::<()>::new();
    if rl.load_history(".history").is_err() {
           println!("no previous history...");
    }
    println!("\t\t type 'new <number>' to add a number to the db");
    println!("\t\t     type 'exit' to leave configuration mode\n");
    loop {
        let readline = rl.readline("CONFIG# ");
        match readline {
            Ok(line) => {
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "new" => util::db::insert_number(&conn, user_input).await,
                    "exit" => break,
                    _ => continue,
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("ctrl+c pressed. quitting now..");
                std::process::exit(0);
            },
            Err(ReadlineError::Eof) => {
                println!("ctrl+d pressed. quitting now..");
                std::process::exit(0);
            },
            Err(err) => {
                println!("error: {:?}", err);
                std::process::exit(0);
            }
        }
    }
    println!("\t\t\t *usage* : send <to> <from> <body>");
    println!("\t\t     for additional cmd information type 'help'");
    loop {
        let readline = rl.readline("spoofy# ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                user_input = get_string_vec(line);
                match user_input[0].as_str() {
                    "send" => util::client::send(&conn, user_input).await,
                    //"get" => util::db::get_numbers(&conn).await,
                    "help" => main_help(),
                    "exit" => std::process::exit(0),
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