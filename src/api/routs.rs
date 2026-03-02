use actix_web::{web, HttpResponse, Responder};
use crate::{api::config, device::shelly::ShellyClient};
use std::net::IpAddr;
use crate::utils::config::ShellyConfig;

async fn shelly_status() -> impl Responder {
    let shelly_config = ShellyConfig::load().unwrap();
    let shelly_ip = shelly_config.shelly_ip.parse::<IpAddr>().unwrap();
    let client = ShellyClient::new(&shelly_ip);
    match client.get_status().await {
        Ok(status) => HttpResponse::Ok().json(status),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

pub fn init_routs(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/shelly", web::get().to(shelly_status))
            .route("/getshelly", web::get().to(config::get_config_shelly))
            .route("/setshelly", web::post().to(config::set_config_shelly))
    );
}