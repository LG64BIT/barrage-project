use crate::diesel::ExpressionMethods;
use crate::errors::ShopError;
use crate::schema::products;
use crate::utils;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub stock_quantity: i32,
}

///product with real values for frontend
#[derive(Queryable, Debug, Serialize)]
pub struct RealProduct {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f32,
    pub stock_quantity: i32,
}

impl RealProduct {
    pub fn to_product(&self) -> Product {
        Product {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            price: utils::to_cents(self.price),
            stock_quantity: self.stock_quantity,
        }
    }
}

impl Product {
    pub fn to_real_product(&self) -> RealProduct {
        RealProduct {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            price: utils::from_cents(self.price),
            stock_quantity: self.stock_quantity,
        }
    }

    pub fn to_real_products(products: &Vec<Product>) -> Vec<RealProduct> {
        let mut real_products: Vec<RealProduct> = Vec::new();
        let mut product: RealProduct;
        for i in 0..products.len() {
            product = RealProduct {
                id: products[i].id.clone(),
                name: products[i].name.clone(),
                description: products[i].description.clone(),
                price: utils::from_cents(products[i].price),
                stock_quantity: products[i].stock_quantity,
            };
            real_products.push(product);
        }
        real_products
    }

    pub fn get_by_id(connection: &PgConnection, id: &str) -> Result<Product, ShopError> {
        let result = products::table
            .select(products::all_columns)
            .filter(products::id.eq(id))
            .first::<Self>(connection)?;
        Ok(result)
    }

    pub fn get_all(connection: &PgConnection) -> Result<Vec<Product>, ShopError> {
        let results = products::table
            .select(products::all_columns)
            .load::<Self>(connection)?;
        Ok(results)
    }

    pub fn insert(
        connection: &PgConnection,
        insertable_product: InsertableProduct,
    ) -> Result<usize, ShopError> {
        Ok(diesel::insert_into(products::table)
            .values(insertable_product)
            .execute(connection)?)
    }

    pub fn get_stock_quantity(
        connection: &PgConnection,
        product_id: &str,
    ) -> Result<usize, ShopError> {
        let stock_quantity = products::table
            .select(products::stock_quantity)
            .filter(products::id.eq(product_id))
            .first::<i32>(connection)?;
        Ok((stock_quantity as usize).try_into().unwrap())
    }
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct InsertableProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
    pub stock_quantity: i32,
}

impl InsertableProduct {
    pub fn new(product: NewProduct) -> InsertableProduct {
        InsertableProduct {
            name: product.name,
            description: product.description,
            price: utils::to_cents(product.price),
            stock_quantity: product.stock_quantity,
        }
    }
}
///Structure that client fills in form when creating new product
#[derive(Deserialize, validator::Validate)]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    #[validate(range(min = 0))]
    pub price: f32,
    pub stock_quantity: i32,
}
