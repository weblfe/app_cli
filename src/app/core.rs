use clap::{ArgMatches, Command};
use std::ffi::OsString;
use std::path::PathBuf;
use std::collections::HashMap;

#[allow(unused)]
pub fn run (app : Command) {
    let matches = app.get_matches();


    match matches.subcommand() {
        Some(("clone", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("push", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Adding {:?}", paths);
        }
        Some(("stash", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
            match stash_command {
                ("apply", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Applying {:?}", stash);
                }
                ("pop", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Popping {:?}", stash);
                }
                ("push", sub_matches) => {
                    let message = sub_matches.get_one::<String>("message");
                    println!("Pushing {:?}", message);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }
}


#[allow(unused)]
pub fn exec (app : Command,runner : HashMap<String,fn(name :&str,matches:&ArgMatches)> ) {
    let matches = app.get_matches();

    match matches.subcommand() {
        Some((name, sub_matches)) => {
             match runner.get(name) {
                 Some(exec) => {
                     exec(name,sub_matches);
                 }
                 _ => unreachable!(),
             }
        },
        _ => unreachable!(),
    }
}