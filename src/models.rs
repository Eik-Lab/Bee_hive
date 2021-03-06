// Generated by diesel_ext
use crate::schema::*;
use chrono::offset::Utc;
use chrono::DateTime;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2;
use serde::{Deserialize, Serialize};

type Result<V, K = diesel::result::Error> = std::result::Result<V, K>;
pub type Pool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
/// Initialize database pool
pub fn init_pool(db_url: &str) -> Pool {
    let manager = r2d2::ConnectionManager::<PgConnection>::new(db_url);

    r2d2::Pool::new(manager).expect("db pool failure")
}

#[derive(
    Identifiable, Insertable, Queryable, Deserialize, Serialize, Debug, Clone, AsChangeset,
)]
#[primary_key(pi_id, measurement_time)]
pub struct Measurement {
    pub pi_id: String,
    pub measurement_time: DateTime<Utc>,
    pub temp1: f32,
    pub temp2: f32,
    pub temp3: f32,
    pub temp4: f32,
    pub bme_temp1: f32,
    pub bme_temp2: f32,
    pub pressure1: f32,
    pub pressure2: f32,
    pub rh1: f32,
    pub rh2: f32,
    pub image_data: Vec<f32>,
}

impl Measurement {
    pub fn insert(&self, conn: &PgConnection) -> Result<Self> {
        diesel::insert_into(crate::schema::measurements::table)
            .values(self)
            .get_result(conn)
    }
}
