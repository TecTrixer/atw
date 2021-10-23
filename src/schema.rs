table! {
    questions (id) {
        id -> Uuid,
        question -> Varchar,
        created_at -> Timestamp,
        votes_yes -> Int4,
        votes_no -> Int4,
        created_by -> Varchar,
        active -> Bool,
    }
}
