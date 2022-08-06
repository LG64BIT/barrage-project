use crate::diesel::ExpressionMethods;
use crate::schema::products;
use crate::utils;
use diesel::result::Error;
use diesel::{PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
}

///product with real values for frontend
#[derive(Queryable, Debug, Serialize)]
pub struct RealProduct {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: f32,
}

impl RealProduct {
    pub fn to_product(&self) -> Product {
        Product {
            id: self.id.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            price: utils::to_cents(self.price),
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
            };
            real_products.push(product);
        }
        real_products
    }

    pub fn get_by_id(connection: &PgConnection, id: &str) -> Result<Product, Error> {
        let result = products::table
            .select(products::all_columns)
            .filter(products::id.eq(id))
            .load::<Self>(connection);
        if result.is_err() {
            return Err(result.err().unwrap());
        }
        let result = result.unwrap();
        if result.len() != 1 {
            return Err(Error::NotFound);
        }
        Ok(Product {
            id: result[0].id.clone(),
            name: result[0].name.clone(),
            description: result[0].description.clone(),
            price: result[0].price.clone(),
        })
    }
}

#[derive(Insertable)]
#[table_name = "products"]
pub struct InsertableProduct {
    pub name: String,
    pub description: String,
    pub price: i32,
}

impl InsertableProduct {
    pub fn new(product: NewProduct) -> InsertableProduct {
        InsertableProduct {
            name: product.name,
            description: product.description,
            price: utils::to_cents(product.price),
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
}
