// @generated automatically by Diesel CLI.

diesel::table! {
    todo_tasks (id) {
        id -> Int4,
        content -> Varchar,
        state -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
