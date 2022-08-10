use super::order_item::{OrderItem, OrderItems};
use super::{cart::Cart, product::Product};
use crate::diesel::RunQueryDsl;
use crate::models::order_item::InsertableOrderItem;
use crate::schema::order_items;
use crate::{errors::ShopError, schema::orders};
use actix_web::cookie::Cookie;
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use diesel::PgConnection;
use serde::Serialize;

#[derive(Queryable, Debug)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub total: i32,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Debug)]
pub struct ShowOrder {
    pub id: String,
    pub user_id: String,
    pub total: f32,
    pub status: String,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing)]
    pub updated_at: NaiveDateTime,
}

impl Order {
    pub fn make_order(
        connection: &PgConnection,
        user_id: &str,
        cart: Cart,
    ) -> Result<HttpResponse, ShopError> {
        //check for empty cart
        if cart.is_empty() {
            return Err(ShopError::ParseError("Cart is empty!".to_string()));
        }
        //check if quantity is in stock and if is, calculate total sum
        let mut total = 0;
        for (product_id, cart_quantity) in cart.get_content() {
            if *cart_quantity > Product::get_stock_quantity(connection, &product_id)? {
                return Err(ShopError::NotEnoughInStockError);
            }
            //if its fine reduce product quantity
            total += Product::get_by_id(connection, &product_id)?.price * (*cart_quantity as i32);
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
        for (product_id, cart_quantity) in cart.get_content() {
            order_items.push(InsertableOrderItem {
                order_id: created_order.id.clone(),
                product_id: product_id.clone(),
                quantity: *cart_quantity as i32,
            });
        }
        diesel::insert_into(order_items::table)
            .values(&order_items)
            .execute(connection)?;
        let order_items = OrderItems {
            order: created_order.to_show_order(),
            items: OrderItem::from_insertable(connection, order_items)?,
        };
        //empty cart
        let mut cookie = Cookie::new("cart", "");
        cookie.make_removal();
        let mut resp = HttpResponse::Ok().json(order_items);
        resp.add_removal_cookie(&cookie)?;
        Ok(resp)
    }

    fn to_show_order(self) -> ShowOrder {
        ShowOrder {
            id: self.id,
            user_id: self.user_id,
            total: self.total as f32 / 100.0,
            status: self.status,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Insertable, Debug)]
#[table_name = "orders"]
pub struct InsertableOrder {
    pub user_id: String,
    pub total: i32,
    pub status: String, //default/initial status is pending
}
