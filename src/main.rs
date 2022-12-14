use actix_web::{get, post, web::Data, App, HttpResponse, HttpServer, Responder, Result};
use futures::TryStreamExt;
use sqlx::{postgres::PgPoolOptions, Postgres, Row};

mod call_api;
mod math_func;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("server is working...")
}

#[get("/math-test")]
async fn math_test() -> impl Responder {
    let math_res = math_func::run();
    HttpResponse::Ok().body(math_res)
}

#[get("/simple-math")]
async fn simple_math() -> impl Responder {
    let a = 999;
    let b = 888;
    let c = 777;
    let res = a * b / c;
    HttpResponse::Ok().body(res.to_string())
}

#[get("/get-from-api")]
async fn get_from_api() -> impl Responder {
    let fact = call_api::get_cat_fact().await.unwrap();
    HttpResponse::Ok().body(fact)
}

#[get("/get-from-db")]
async fn get_from_db(pool: Data<sqlx::Pool<Postgres>>) -> Result<impl Responder> {
    let mut conn = pool.acquire().await.unwrap();
    let mut rows = sqlx::query("SELECT * FROM data").fetch(&mut conn);

    let mut res = String::new();

    while let Some(row) = rows.try_next().await.unwrap() {
        let id: i32 = row.get(0);
        res = id.to_string();
    }

    return Ok(HttpResponse::Ok().body(res));
}

#[post("/write-to-db")]
async fn write_to_db(pool: Data<sqlx::Pool<Postgres>>) -> Result<impl Responder> {
    let mut conn = pool.acquire().await.unwrap();
    let mut rows = sqlx::query("INSERT INTO data VALUES (777)").fetch(&mut conn);

    while let Some(row) = rows.try_next().await.unwrap() {
        let id: i32 = row.get(0);
        println!("id: {}", id);
    }

    return Ok(HttpResponse::Ok().body("OK"));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/jordan")
        .await
        .unwrap();

    println!("server running on port 8080...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(get_from_db)
            .service(write_to_db)
            .service(get_from_api)
            .service(math_test)
            .service(simple_math)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
