use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::configuration;
use crate::{AppState};

pub async fn get_config(data: web::Data<AppState>) -> impl Responder {
    let shards = data.shard_collection.get_shards_configuration();
    let body = configuration::get_cluster_configuration(shards);
    HttpResponse::Ok().body(body)
}