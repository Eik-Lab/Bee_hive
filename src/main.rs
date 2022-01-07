#[macro_use]
extern crate diesel;
use crate::models::Measurement;
use actix_web::{web, web::Data, App, HttpServer};
use chrono::Utc;
use diesel::RunQueryDsl;
use serde::{Deserialize, Serialize};
pub mod models;
pub mod schema;
use crate::models::Pool;
use diesel::prelude::*;
use schema::measurements;
use std::env;
use actix_core::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if dotenv::dotenv().is_ok() {
        println!("Loaded dotenv");
    }
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
        let cors = Cors::default()
                .allow_any_origin()
                .allow_any_header()
                .allow_any_method();
        App::new()
            .app_data(Data::new(pool.clone()))
            .wrap(cors)
            .service(post_data)
            .service(index)
            .service(get_data)
            .service(serve_docs)
            .service(get_unique_ids)
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
    To send some data, go to the /data endpoint\r\n
    You can find the documentation at the /docs endpoint \r\n"
}

use actix_web::Result;
#[actix_web::get("/docs")]
async fn serve_docs() -> Result<actix_files::NamedFile> {
    Ok(actix_files::NamedFile::open("./static/swagger.html")?)
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

#[actix_web::get("/get_unique_ids")]
async fn get_unique_ids(pool: web::Data<Pool>) -> actix_web::HttpResponse {
    let conn = pool.get().unwrap();
    let mut data: Vec<String> = measurements::dsl::measurements
        .select(measurements::pi_id)
        .distinct()
        .get_results(&conn)
        .unwrap();
    let cleaned_data = remove_whitespace(&mut data);
    actix_web::HttpResponse::Ok().json(cleaned_data)
}

// Function that removes whitespace from a vector of strings
fn remove_whitespace(vec: &mut Vec<String>) -> &mut Vec<String> {
    for i in 0..vec.len() {
        vec[i] = vec[i].replace(" ", "");
    }
    vec
}
