table! {
    bicycles (id) {
        id -> Uuid,
        owner_id -> Uuid,
        color -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}
