use rustyline::error::ReadlineError;
use rustyline::Editor;

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
                    usage: send <FROM> <TO> <body>"
      help          this page lol
      quit          exits the program"#;
    println!("{}", help);
}

// python readline() equivalent
pub fn cli_line(prompt: &str) -> Vec<String> {
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