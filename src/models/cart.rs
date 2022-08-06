use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Cart {
    pub items: HashMap<String, usize>, // K: product_id, V: quantity
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, product_id: String, quantity: usize) {
        if !self.items.contains_key(&product_id) {
            self.items.insert(product_id, quantity);
            return;
        }
        *self.items.get_mut(&product_id).unwrap() += quantity; //check if stock has enough quantity to add to cart
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
