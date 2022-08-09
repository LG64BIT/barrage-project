table! {
    order_items (id) {
        id -> Varchar,
        order_id -> Varchar,
        product_id -> Varchar,
        quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    orders (id) {
        id -> Varchar,
        user_id -> Varchar,
        total -> Int4,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        price -> Int4,
        stock_quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Varchar,
        email -> Varchar,
        password -> Varchar,
        is_admin -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(order_items, orders, products, users,);
