use std::collections::HashMap;

use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::errors::ShopError;

use super::product::Product;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cart {
    pub items: HashMap<String, usize>, // K: product_id, V: quantity
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: HashMap::new(),
        }
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
}

#[derive(Deserialize)]
pub struct CartItem {
    pub quantity: usize,
}
