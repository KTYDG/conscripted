use self::table::*;
use conscripted_bot::*;
use diesel::prelude::*;

fn main() {
    use self::schema::people::dsl::*;

    let connection = &mut establish_connection();
    let results: Vec<People> = people
        .load::<People>(connection)
        .expect("Error loading posts");

    println!("Displaying {} conscripted people", results.len());
    for line in results {
        println!("{} {}", line.fio, from_date(line.birthday));
    }
}
