use serde::{Deserialize, Serialize};

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
#[derive(Debug, Deserialize, Serialize)]
pub struct DepthResponse {
    pub bids: Vec<[u32; 2]>,
    pub asks: Vec<[u32; 2]>,
    pub last_updated_id: String,
}
