pub mod actions;
pub mod alias;
pub mod store;

use std::{env, process::Command};

use crate::store::Store;
use actions::Action;
use alias::Alias;

extern crate dirs;

fn main() {
    let mut store = Store::new();

    let args: Vec<String> = env::args().collect();

    if args.len() < 1 {
        panic!("You need to use an action")
    }

    let action_argument = &args[1];
    let action = Action::from_str(action_argument);

    match action {
        Action::Add => {
            let arguments = &args[2..];

            let mut is_command = false;
            let mut is_alias = false;

            let mut command = String::new();
            let mut alias = String::new();

            for element in arguments.iter() {
                if element == "--alias" {
                    is_alias = true;
                    is_command = false;
                    continue;
                }
                if element == "--command" {
                    is_command = true;
                    is_alias = false;
                    continue;
                }

                if is_command {
                    command = format!("{} {}", command, element);
                    continue;
                }

                if is_alias {
                    alias = format!("{} {}", alias, element);
                    continue;
                }
            }

            Alias::new(alias.trim().to_string(), command.trim().to_string()).create(&mut store);

            println!("Stored new alias {}", alias);
        }
        Action::Run => {
            let arguments = &args[2..];

            let alias = arguments.join(" ");

            let command = store.get_command(alias);

            println!("{}", command);

            Command::new(command)
                .output()
                .expect("failed to execute process");
        }
    }
}
