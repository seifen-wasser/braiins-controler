use actix_web::{web, HttpResponse, Responder};
use crate::utils::config::ShellyConfig;
use crate::device::shelly::ShellyClient;

pub async fn set_config_shelly(config: web::Json<ShellyConfig>) -> impl Responder{

    if ShellyClient::test_shelly(&config.shelly_ip).await {
        println!("Successfully connected to Shelly at {}", config.shelly_ip);
        config.save().unwrap();

        HttpResponse::Ok().json("status: success")
    } else {
        println!("Failed to connect to Shelly at {}", config.shelly_ip);
        return HttpResponse::BadRequest().body("Failed to connect to Shelly with provided IP");
    }

}

pub async fn get_config_shelly() -> impl Responder {
    let config = ShellyConfig::load().unwrap();
    HttpResponse::Ok().json(config)
}