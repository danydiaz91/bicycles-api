table! {
    bicycles (id) {
        id -> Nullable<Uuid>,
        owner_id -> Uuid,
        color -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}
