use actix_web::{App, HttpServer};
use std::sync::{Arc, Mutex};

pub mod inputs;
pub mod orderbook;
pub mod outputs;
pub mod routes;

use crate::{
    orderbook::Orderbook,
    routes::{create_order, delete_order, get_depth},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orderbook = Arc::new(Mutex::new(Orderbook::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(orderbook.clone())
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
