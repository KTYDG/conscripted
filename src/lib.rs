pub mod schema;
pub mod table;

use {
    diesel::{pg::PgConnection, prelude::*},
    dotenv::dotenv,
};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use crate::table::People;
use diesel::{
    dsl::{exists, not},
    select
};

use self::table::NewMan;
pub fn create_man(conn: &mut PgConnection, fio_a: &str, birthday_a: &str) {
    use crate::schema::{people, people::dsl::*};
    let birthday_a = to_date(birthday_a);
    let new_man = NewMan {
        fio: fio_a,
        birthday: birthday_a,
    };

    if select(not(exists(
        people.filter(fio.eq(fio_a).and(birthday.eq(birthday_a))),
    )))
    .get_result(conn)
    .unwrap()
    {
        diesel::insert_into(people::table)
            .values(&new_man)
            .get_result::<People>(conn)
            .expect("Error adding man");
    }
}
// pub fn create_men(conn: &mut PgConnection, men: Vec<NewMan>) {
//     use crate::schema::people;

//     diesel::insert_into(people::table)
//         .values(men)
//         .get_results::<People>(conn)
//         .expect("Error adding man");
// }

use chrono::NaiveDate;
pub fn to_date(birthday: &str) -> NaiveDate {
    NaiveDate::parse_from_str(birthday, "%d.%m.%Y").expect("Wrong Birthday ")
}
pub fn from_date(birthday: NaiveDate) -> String {
    birthday.format("%d.%m.%Y").to_string()
}

pub fn amount(conn: &mut PgConnection) -> i64 {
    use crate::schema::people::dsl::*;
    people.count().get_result(conn).unwrap_or(0)
}
pub fn amount_by_fio(conn: &mut PgConnection, f: String) -> i64 {
    use crate::schema::people::dsl::*;
    people.filter(fio.like(f)).count().get_result(conn).unwrap_or(0)
}

pub fn find_man(conn: &mut PgConnection, f: String, bd: NaiveDate) -> bool {
    use crate::schema::people::dsl::*;
    select(exists(people.filter(fio.eq(f).and(birthday.eq(bd)))))
        .get_result(conn)
        .unwrap()
}
pub fn find_fio(conn: &mut PgConnection, f: String) -> bool {
    use crate::schema::people::dsl::*;
    select(exists(people.filter(fio.like(f))))
        .get_result(conn)
        .unwrap()
}

pub fn get_men(conn: &mut PgConnection, f: String) -> String {
    use crate::schema::people::dsl::*;
    let mut message = format!("Найден(о) {} человек(a):\n", amount_by_fio(conn, f.clone()));
    let results: Vec<People> = people
        .filter(fio.like(f))
        .load::<People>(conn)
        .expect("Error loading posts");
    for row in results {
        message = format!("{} {} {}\n", message, row.fio, from_date(row.birthday));
    }
    message
}