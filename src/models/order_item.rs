use crate::schema::order_items;
use chrono::NaiveDateTime;

pub struct OrderItem {
    pub id: String,
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[table_name = "order_items"]
pub struct InsertableOrderItem {
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
}
