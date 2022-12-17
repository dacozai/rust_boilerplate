// @generated automatically by Diesel CLI.

diesel::table! {
  ppl (id) {
    id -> Integer,
    surname -> Nullable<Varchar>,
    givenname -> Nullable<Varchar>,
  }
}

diesel::allow_tables_to_appear_in_same_query!(
  ppl,
);
