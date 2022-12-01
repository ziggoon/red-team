use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Message {
    id: i32,
    number_to: String,
    number_from: String,
    body: String
}

#[derive(Debug)]
struct PhoneNumber {
    id: i32,
    number: String
}

pub async fn check_db(conn: &Connection) -> Result<()> {
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

pub async fn insert_message(conn: &Connection, args: Vec<String>) -> Result<()> {
    println!("inside msg insert");
    conn.execute(
        "insert into messages (number_to, number_from, msg_body) values (?1, ?2, ?3)",
        &[args[1].as_str(), args[2].as_str(), args[3].as_str()],
    ).expect("insert failed");
    Ok(())
}

pub async fn insert_number(conn: &Connection, args: Vec<String>) -> Result<()> {
    conn.execute(
        "insert into numbers (number) values (?1)",
        &[args[1].as_str()],
    ).expect("insert failed");
    Ok(())
}
 
pub async fn get_numbers(conn: &Connection) -> Result<()> {
   let mut stmt = conn.prepare("select id, number from numbers")?;
   let num_iter = stmt.query_map([], |row| {
    Ok(PhoneNumber {
        id: row.get(0)?,
        number: row.get(1)?,
    })
   })?;
   
   for num in num_iter {
    println!("found number {:?}", num.unwrap())
   }
   Ok(())
}

pub async fn get_messages(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("select id, number_to, number_from, msg_body from messages")?;
    let msg_iter = stmt.query_map([], |row| {
     Ok(Message {
         id: row.get(0)?,
         number_to: row.get(1)?,
         number_from: row.get(2)?,
         body: row.get(3)?,
     })
    })?;
    
    for msg in msg_iter {
     println!("found message {:?}", msg.unwrap())
    }
    Ok(())
 }
