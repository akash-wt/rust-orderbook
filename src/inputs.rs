use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateOrderInput {
    pub price: u32,
    pub quantity: u32,
    pub user_id: u32,
    pub side: Side,
}
#[derive(Debug, Deserialize, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}
#[derive(Debug, Deserialize)]
pub struct DeleteOrderInput {
    pub order_id: String,
    pub user_id: String,
}
