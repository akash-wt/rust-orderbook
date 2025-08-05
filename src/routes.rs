use crate::{
    inputs::{CreateOrderInput, DeleteOrderInput},
    orderbook::{self, Orderbook},
    outputs::{CreateOrderResponse, DeleteOrderResponse, DepthResponse},
};
use actix_web::{
    HttpResponse, Responder, body, delete, get, post,
    web::{Data, Json},
};
use std::{
    ops::Bound,
    sync::{Arc, Mutex},
};

#[post("/order")]
pub async fn create_order(
    body: Json<CreateOrderInput>,
    mutex_orderbook: Data<Arc<Mutex<Orderbook>>>,
) -> impl Responder {
    let price = body.0.price;
    let quantity = body.0.quantity;
    let user_id = body.0.user_id;
    let side = body.0.side;

    let mut orderbook = mutex_orderbook.lock().unwrap();
    orderbook.create_order(price, quantity, user_id, side);

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: String::from("hi there"),
        filled_quantity: 2.0,
        average_price: 100.0,
        reamining_quantity: 1.1,
    });
}

#[delete("/order")]
pub async fn delete_order(Json(body): Json<DeleteOrderInput>) -> impl Responder {
    let user_id = body.user_id;
    let order_id = body.order_id;
    println!("{:?}", user_id);
    println!("{:?}", order_id);

    return HttpResponse::Ok().json(DeleteOrderResponse {
        success: true,
        remaining_quantity: 12.0,
        filled_quantity: 22.0,
        average_price: 45.0,
    });
}
#[get("depth")]
pub async fn get_depth() -> impl Responder {
    "get_depth"
}
