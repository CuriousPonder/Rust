table! {
    groups (id) {
        id -> Int4,
        group_ids -> Int4,
        group_name -> Varchar,
        subject -> Varchar,
        description -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        post_title -> Varchar,
        description -> Varchar,
        file_id -> Int4,
        files -> Varchar,
        file_size -> Varchar,
        file_ext -> Varchar,
        file_name -> Varchar,
        is_published -> Bool,
    }
}

table! {
    superusers (id) {
        id -> Int4,
        super_id -> Nullable<Int4>,
        username -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        email_address -> Varchar,
        password -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        email_address -> Varchar,
        password -> Varchar,
    }
}

joinable!(posts -> users (user_id));

allow_tables_to_appear_in_same_query!(
    groups,
    posts,
    superusers,
    users,
);
