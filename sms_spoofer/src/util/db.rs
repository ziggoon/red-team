use rusqlite::{Connection, Result};


/*#[derive(Debug)]
struct Message {
    id: i32,
    number_to: String,
    number_from: String,
    body: String
}

#[derive(Debug)]
struct PhoneNumber {
    id: i32,
    number: String,
}*/

pub async fn check_db(conn: &Connection) -> Result<()> {
    //let conn = Connection::open("db.db")?;
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

pub async fn insert_message(conn: &Connection, args: Vec<String>) {
    //let conn = Connection::open("db.db").expect("connection failed");
    conn.execute(
        "insert into messages (number_to, number_from, msg_body) values (?1, ?2, ?3)",
        &[args[1].as_str(), args[2].as_str(), args[3].as_str()],
    ).expect("insert failed");
}

pub async fn insert_number(conn: &Connection, args: Vec<String>) {
    //let conn = Connection::open("db.db").expect("connection failed");
    conn.execute(
        "insert into numbers (number) values (?1)",
        &[args[1].as_str()],
    ).expect("insert failed");
}
 
/*pub async fn get_numbers(conn: &Connection) {
    let row: Result<String, rusqlite::Error> = conn.query_row("select number from numbers;", [], |row|)
}*/