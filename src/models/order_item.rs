use super::order::ShowOrder;
use super::product::Product;
use super::product::RealProduct;
use crate::diesel::ExpressionMethods;
use crate::errors::ShopError;
use crate::schema::{order_items, products};
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::Serialize;

/// Struct used for returning in response after order is made
#[derive(Serialize, Debug)]
pub struct OrderItems {
    pub order: ShowOrder,
    pub items: Vec<OrderItem>,
}

/// Struct for showing order item
#[derive(Serialize, Debug)]
pub struct OrderItem {
    pub product: RealProduct,
    pub quantity: i32,
}

impl OrderItem {
    pub fn from_insertable(
        connection: &PgConnection,
        insertable_order_item: Vec<InsertableOrderItem>,
    ) -> Result<Vec<Self>, ShopError> {
        let mut order_items: Vec<Self> = Vec::new();
        for i in 0..insertable_order_item.len() {
            let product = products::table
                .select(products::all_columns)
                .filter(products::id.eq(&insertable_order_item[i].product_id))
                .get_result::<Product>(connection)?;
            order_items.push(Self {
                product: product.to_real_product(),
                quantity: insertable_order_item[i].quantity,
            });
        }
        Ok(order_items)
    }
}

#[derive(Insertable, Serialize, Debug)]
#[table_name = "order_items"]
pub struct InsertableOrderItem {
    pub order_id: String,
    pub product_id: String,
    pub quantity: i32,
}
