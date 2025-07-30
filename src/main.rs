use actix_web::{App, HttpServer, Responder, delete, get, post};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrder {
    pub price: f64,
    pub quantity: f64,
    pub user_id: String,
    pub side: Side,
}
#[derive(Debug, Deserialize, Serialize)]
pub enum Side {
    Buy,
    Sell,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteOrder {
    pub order_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateOrderResponse {
    pub order_id: String,
    pub filled_quantity: f64,
    pub reamining_quantity: f64,
    pub average_price: f64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteOrderResponse {
    pub success: bool,
    pub remaining_quantity: f64,
    pub filled_quantity: f64,
    pub average_price: f64,
}

#[post("/order")]
pub async fn create_order() -> impl Responder {
    "create_order"
}

#[delete("/order")]
pub async fn delete_order() -> impl Responder {
    "delete order"
}
#[get("depth")]
pub async fn get_depth() -> impl Responder {
    "get_depth"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_order)
            .service(delete_order)
            .service(get_depth)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
