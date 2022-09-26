use conscripted_bot::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let connection = &mut establish_connection();

    let file = File::open("test.txt").expect("Can't open file");
    let buf = BufReader::new(file);

    let mut i: usize = 0;
    for man in buf.lines() {
        let m = man.unwrap();
        let mut man: Vec<&str> = m.split('\t').collect();

        man.reverse();
        let fio = man.pop().unwrap();
        let birthday = man.pop().unwrap();

        create_man(connection, fio, birthday);
        println!("{i}");
        i += 1;
    }
}
