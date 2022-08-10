use std::collections::HashMap;

use actix_web::{cookie::Cookie, HttpRequest};
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::errors::ShopError;

use super::product::Product;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cart {
    items: HashMap<String, usize>, // K: product_id, V: quantity
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: HashMap::new(),
        }
    }

    pub fn get_content(&self) -> &HashMap<String, usize> {
        &self.items
    }

    /// Checks whether cart is empty
    pub fn is_empty(&self) -> bool {
        self.get_content().is_empty()
    }

    pub fn add(
        &mut self,
        connection: &PgConnection,
        product_id: String,
        reqested_quantity: usize,
    ) -> Result<(), ShopError> {
        if reqested_quantity <= 0 {
            return Err(ShopError::InvalidInput);
        }
        let stock_quantity = Product::get_stock_quantity(&connection, &product_id)?;
        if reqested_quantity > stock_quantity {
            return Err(ShopError::NotEnoughInStockError);
        }
        if !self.items.contains_key(&product_id) {
            self.items.insert(product_id, reqested_quantity);
            return Ok(());
        }
        *self.items.get_mut(&product_id).unwrap() += reqested_quantity;
        Ok(())
    }

    pub fn remove(&mut self, product_id: String, quantity: usize) {
        if !self.items.contains_key(&product_id) {
            return;
        }
        let qty = self.items.get_mut(&product_id).unwrap();
        if *qty <= 1 || *qty <= quantity {
            self.items.remove(&product_id);
            return;
        }
        *qty -= quantity;
    }
    /// get Cart from request cookies
    pub fn get(req: &HttpRequest) -> Result<Cart, ShopError> {
        let cookie = match req.cookie("cart") {
            Some(c) => c,
            None => {
                let cart = Cart::new();
                Cookie::new("cart", serde_json::to_string(&cart)?)
            }
        };
        Ok(serde_json::from_str::<Cart>(&cookie.value().to_string())?)
    }
}

#[derive(Deserialize)]
pub struct CartItem {
    pub quantity: usize,
}
