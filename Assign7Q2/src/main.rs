use std::io;
mod lib;
use lib::university::department;
fn main() {
    println!("Enter your university department");
    let mut depart_name = String::new();
    io::stdin().read_line(&mut depart_name).expect("failed to get input");
    department::show_depart_name(&depart_name);
}
