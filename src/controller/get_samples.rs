use axum::{
    extract::Extension,
    Json,
};
use rusqlite::Connection;

use crate::model::database::{Sample, get_all_samples};

async fn get_samples_handler(Extension(conn): Extension<Connection>) -> Json<Vec<Sample>> {
    let samples = get_all_samples(&conn).unwrap();
    Json(samples)
}