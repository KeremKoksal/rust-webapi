table! {
    departments (id) {
        id -> Int2,
        name -> Varchar,
        short_name -> Varchar,
        external_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        staff_title -> Nullable<Varchar>,
        education_title -> Nullable<Varchar>,
        email -> Varchar,
        password -> Nullable<Varchar>,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        department_id -> Nullable<Int2>,
        email_verified -> Bool,
        active -> Bool,
        roles -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(users -> departments (department_id));

allow_tables_to_appear_in_same_query!(
    departments,
    users,
);
