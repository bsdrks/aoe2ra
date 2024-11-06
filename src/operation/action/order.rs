use crate::order_type::OrderType;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Order {

    pub building_id: i32,
    pub order_type: OrderType,
    pub cancel_order: u8,
    pub x: f32,
    pub y: f32,
    pub flags: Option<Vec<u8>>,
    pub unit_ids: Vec<u32>,
}
