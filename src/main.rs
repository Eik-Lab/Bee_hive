#[macro_use]
extern crate diesel;

use actix_web::{web, web::Data, App, HttpServer};

use crate::models::Measurement;
pub mod models;
pub mod schema;
use crate::models::Pool;
use std::env;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let pool = crate::models::init_pool(&db_url);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(example)
            .service(no_params)
    })
    .bind(api_route)
    .unwrap()
    .run()
    .await;
    Ok(())
}
#[actix_web::get("/")]
async fn no_params() -> &'static str {
    "Hello and Welcome to Bee-Hive!\r\n
    To send some data, go to the /data endpoint\r\n"
}

#[actix_web::post("/data")]
async fn example(
    web::Json(test_var): web::Json<Measurement>,
    pool: web::Data<Pool>,
) -> actix_web::HttpResponse {
    println!("{:#?}", test_var);
    test_var.insert(&pool.get().unwrap());
    actix_web::HttpResponse::Ok().finish()
}
