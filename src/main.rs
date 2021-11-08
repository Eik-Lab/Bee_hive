#[macro_use]
extern crate diesel;
use crate::models::Measurement;
use actix_web::{web, web::Data, App, HttpServer};
use chrono::Utc;
use diesel::dsl::Filter;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
pub mod models;
pub mod schema;
use crate::models::Pool;
use diesel::prelude::*;
use dotenv;
use schema::measurements;
use std::env;

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
            .service(get_data)
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
#[derive(Serialize, Deserialize)]
pub struct QueryFilter {
    pub start_time: Option<chrono::DateTime<Utc>>,
    pub stop_time: Option<chrono::DateTime<Utc>>,
    pub sensor_id: Option<String>,
}

#[actix_web::get("/data")]
async fn get_data(
    filter: web::Query<QueryFilter>,
    pool: web::Data<Pool>,
) -> actix_web::HttpResponse {
    let start;
    let stop;
    if filter.start_time.is_none() {
        start = chrono::Utc::now() - chrono::Duration::days(1);
    } else {
        start = filter.start_time.unwrap();
    }
    if filter.stop_time.is_none() {
        stop = start + chrono::Duration::days(1);
    } else {
        stop = filter.stop_time.unwrap();
    }
    let conn = pool.get().unwrap();
    let mut query = measurements::dsl::measurements
        .order(measurements::measurement_time.desc())
        .filter(
            measurements::measurement_time
                .ge(start)
                .and(measurements::measurement_time.lt(stop)),
        )
        .into_boxed();
    if let Some(id) = &filter.sensor_id {
        query = query.filter(measurements::pi_id.eq(id)); // Add another filter to the request
    }

    let data: Vec<Measurement> = query.get_results(&conn).unwrap();
    actix_web::HttpResponse::Ok().json(data)
}
