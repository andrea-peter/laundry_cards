use rusqlite::{Connection, Error, OpenFlags, Result};

pub fn hello() {
    println!("Hello");
}

pub fn create_db(db_path: &str) -> Result<(), Error> {
    let conn = Connection::open_with_flags(
        db_path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;

    conn.execute(
        "CREATE TABLE card (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            uid BLOB,
            secret_key BLOB
        )",
        (),
    )?;
    Ok(())
}
