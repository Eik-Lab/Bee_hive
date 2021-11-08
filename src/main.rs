#[macro_use]
extern crate diesel;
use actix_web::{web, web::Data, App, HttpServer};
use diesel::RunQueryDsl;
use crate::models::Measurement;
pub mod models;
pub mod schema;
use crate::models::Pool;
use std::env;
use schema::{measurements};
use diesel::prelude::*;
use dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv();
    println!("Start");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let api_route = format!("0.0.0.0:{}", port);
    let db_url = match std::env::var("DATABASE_URL") {
        Ok(db_url) => db_url,
        Err(_) => {
            println!("DATABASE_URL not set");
            panic!()
        }
    };
    println!("Getting DB pool");
    let pool = crate::models::init_pool(&db_url);
    println!("Starting API services");
    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(post_data)
            .service(index)
    })
    .bind(api_route)
    .unwrap()
    .run()
    .await;
    println!("API server shut down here");
    Ok(())
}
#[actix_web::get("/")]
async fn index() -> &'static str {
    "Hello and Welcome to Bee-Hive!\r\n
    To send some data, go to the /data endpoint\r\n"
}

#[actix_web::post("/data")]
async fn post_data(
    web::Json(data): web::Json<Measurement>,
    pool: web::Data<Pool>,
) -> actix_web::HttpResponse {
    let res = data.insert(&pool.get().unwrap());
    match res {
        Ok(res) => println!("OK: {}{}", res.pi_id, res.measurement_time),
        Err(e) => println!("ERR: {}", e.to_string()),
    }
    actix_web::HttpResponse::Ok().finish()
}


#[actix_web::get("/data")]
async fn get_data(
pool: web::Data<Pool>
) -> actix_web::HttpResponse {
    let conn = pool.get().unwrap();
    let data: Vec<Measurement> = measurements::dsl::measurements
    .order( measurements::measurement_time.desc() )
    .get_results(&conn).unwrap();
    actix_web::HttpResponse::Ok().json(data)
}

