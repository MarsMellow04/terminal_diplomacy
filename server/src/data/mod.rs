use postgres::{Client, NoTls, Error};

pub fn start_database() -> Result<(), Error> {
    // Start database locally with 
    // docker run --name my-postgres -e POSTGRES_PASSWORD=mysecretpassword -e POSTGRES_USER=postgres -e POSTGRES_DB=library -p 5432:5432 -d postgres
    let mut client = Client::connect("postgresql://postgres:mysecretpassword@localhost/library", NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS author (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR NOT NULL,
            country         VARCHAR NOT NULL
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS book  (
            id              SERIAL PRIMARY KEY,
            title           VARCHAR NOT NULL,
            author_id       INTEGER NOT NULL REFERENCES author
            )
    ")?;

    Ok(())

}