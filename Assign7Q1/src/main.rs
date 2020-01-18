mod university{
    pub mod department{
        pub fn show_depart_name(depart_name:&String){
            println!("Your department is {}",depart_name);
        }
    }
}
use std::io;
use crate::university::department;
fn main() {
    println!("Enter your university department");
    let mut depart_name = String::new();
    io::stdin().read_line(&mut depart_name).expect("failed to get input");
    department::show_depart_name(&depart_name);
}
