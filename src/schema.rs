// @generated automatically by Diesel CLI.

diesel::table! {
    people (fio, birthday) {
        fio -> Varchar,
        birthday -> Date,
    }
}
