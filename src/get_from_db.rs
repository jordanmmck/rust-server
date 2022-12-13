// use postgres::{Client, Error, NoTls};

// pub async fn run() -> Result<(), Error> {
//     let mut client = Client::connect("postgres://postgres:postgres@localhost:5432/jordan", NoTls)?;

//     for row in client.query("SELECT * FROM data", &[])? {
//         let id: i32 = row.get(0);
//         println!("data: {}", id);
//     }

//     Ok(())
// }

// use tokio_postgres::{Error, NoTls};
use postgres::{Client, NoTls};

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
pub async fn run() -> Result<(), Error> {
    let mut client;
    std::thread::spawn(|| {
        client =
            Client::connect("postgres://postgres:postgres@localhost:5432/jordan", NoTls).unwrap();
    });

    // Connect to the database.

    let result = client.query("SELECT * FROM data", &[]);
    match result {
        Ok(rows) => {
            for row in rows {
                let id: i32 = row.get(0);
                println!("data: {}", id);
            }
        }
        Err(_) => {
            return Err(actix_web::error::ErrorInternalServerError("rip"));
        }
    }
}
