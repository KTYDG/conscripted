use conscripted_bot::*;

fn main() {
    let connection = &mut establish_connection();
    println!("I've got {} conscripted people", amount(connection));
}
