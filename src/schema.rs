table! {
    products (id) {
        id -> Varchar,
        name -> Varchar,
        description -> Varchar,
        price -> Int4,
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

allow_tables_to_appear_in_same_query!(products, users,);
