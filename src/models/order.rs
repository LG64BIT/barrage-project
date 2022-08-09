use super::{cart::Cart, product::Product};
use crate::diesel::RunQueryDsl;
use crate::models::order_item::InsertableOrderItem;
use crate::schema::order_items;
use crate::{errors::ShopError, schema::orders};
use actix_web::HttpResponse;
use actix_web::cookie::Cookie;
use chrono::NaiveDateTime;
use diesel::PgConnection;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub total: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Order {
    pub fn make_order(
        connection: &PgConnection,
        user_id: &str,
        cart: Cart,
    ) -> Result<HttpResponse, ShopError> {
        //check if quantity in stock and if is, calculate total sum
        let mut total = 0;
        for (product_id, cart_quantity) in &cart.items {
            if *cart_quantity > Product::get_stock_quantity(connection, &product_id)? {
                return Err(ShopError::NotEnoughInStockError);
            }
            //if its fine reduce product quantity
            total += Product::get_by_id(connection, &product_id)?.price;
            Product::reduce_stock_quantity(connection, &product_id, *cart_quantity)?;
        }
        //create record in orders
        let insertable = InsertableOrder {
            user_id: user_id.to_string(),
            total,
            status: "Pending".to_string(),
        };
        //get created order id
        let created_order: Order = diesel::insert_into(orders::table)
            .values(insertable)
            .returning(orders::all_columns)
            .get_result::<Order>(connection)?;
        // make records in order_items
        let mut order_items: Vec<InsertableOrderItem> = Vec::new();
        for (product_id, cart_quantity) in &cart.items {
            order_items.push(InsertableOrderItem {
                order_id: created_order.id.clone(),
                product_id: product_id.clone(),
                quantity: *cart_quantity as i32,
            });
        }
        diesel::insert_into(order_items::table)
            .values(&order_items)
            .execute(connection)?;
        //empty cart
        let mut cookie = Cookie::new("cart", "");
        cookie.make_removal();
        let mut resp = HttpResponse::Ok().json(created_order);
        resp.add_removal_cookie(&cookie)?;
        Ok(resp)
    }
}

#[derive(Insertable, Debug)]
#[table_name = "orders"]
pub struct InsertableOrder {
    pub user_id: String,
    pub total: i32,
    pub status: String, //default/initial status is pending
}
