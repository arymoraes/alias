pub mod alias;
pub mod store;

use std::env;

use crate::store::Store;
use alias::Alias;

extern crate dirs;

fn main() {
    println!("Hello, world!");

    // let mut cachorrimoso = Store::new();

    // let danlialias = Alias::new(String::from("test_key23"), String::from("test_command23"));

    // danlialias.create(&mut cachorrimoso);

    // println!("{:?}", cachorrimoso);

    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("You need to use an action")
    }

    let action = &args[1];

    // println!("{}", action);

    match action.as_str() {
        "add" => {
            print!("You are trying to add an alias");
        }
        "run" => {
            println!("You are trying to use an alias");
        }
        &_ => {}
    }
}
