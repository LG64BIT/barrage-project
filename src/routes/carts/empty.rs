use crate::errors::ShopError;
use actix_web::{cookie::Cookie, HttpResponse};

pub async fn handle() -> Result<HttpResponse, ShopError> {
    let mut cookie = Cookie::new("cart", "");
    cookie.make_removal();
    let mut resp = HttpResponse::Ok().finish();
    resp.add_removal_cookie(&cookie)?;
    Ok(resp)
}
