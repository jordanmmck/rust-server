use actix_web::{get, App, HttpResponse, HttpServer, Responder, Result};
// use postgres::{Client, NoTls};
use tokio_postgres::{Error, NoTls};

mod call_api;
// mod get_from_db;
mod math_func;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("server is working...")
}

#[get("/math-test")]
async fn math() -> impl Responder {
    let math_res = math_func::run();
    println!("{:?}", math_res);
    HttpResponse::Ok().body("Hello world!")
}

#[get("/get-from-api")]
async fn fetch() -> impl Responder {
    let fact = call_api::get_cat_fact().await.unwrap();
    HttpResponse::Ok().body(fact)
}

#[tokio::main]
#[get("/get-from-db")]
async fn get_int() -> Result<impl Responder> {
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect("host=localhost user=jordan", NoTls)
        .await
        .unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query("SELECT * FROM data", &[&"hello world"])
        .await
        .unwrap();

    // And then check that we got back the same string we sent over.
    // let value: &str = rows[0].get(0);
    // assert_eq!(value, "hello world");

    Ok(HttpResponse::Ok().body(""))
}

// #[get("/get-from-db")]
// async fn get_int() -> Result<impl Responder> {
//     let mut client =
//         Client::connect("postgres://postgres:postgres@localhost:5432/jordan", NoTls).unwrap();

//     let result = client.query("SELECT * FROM data", &[]);
//     match result {
//         Ok(rows) => {
//             for row in rows {
//                 let id: i32 = row.get(0);
//                 println!("data: {}", id);
//             }
//         }
//         Err(_) => {
//             return Err(actix_web::error::ErrorInternalServerError("rip"));
//         }
//     }

//     return Ok(HttpResponse::Ok().body(""));
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(math)
            .service(fetch)
            .service(get_int)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
