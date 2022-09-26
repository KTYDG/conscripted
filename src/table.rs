use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct People {
    pub fio: String,
    pub birthday: NaiveDate,
}

use crate::schema::people;

#[derive(Insertable)]
#[diesel(table_name=people)]
pub struct NewMan<'a> {
    pub fio: &'a str,
    pub birthday: NaiveDate,
}
