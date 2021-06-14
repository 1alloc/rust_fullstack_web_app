use actix_web::{web};
use crate::routes::api::setup_routes;

pub fn initialize(cfg: &mut web::ServiceConfig) {
    setup_routes(cfg);
}