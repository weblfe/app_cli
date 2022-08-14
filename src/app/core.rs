use std::ffi::OsString;
use std::collections::HashMap;
use clap::{ArgMatches, Command};

pub const MAIN_RUNNER_KEY: &str = "main";

#[allow(unused)]
pub fn run (app : Command) {
    let matches = app.get_matches();

    match matches.subcommand() {
        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable!()
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
                 // 默认主逻辑
                 _ => match  runner.get(MAIN_RUNNER_KEY)  {
                     Some(exec) => {
                         exec(name,sub_matches);
                     },
                     _ => unreachable!()
                 }
             }
        },
        _ => unreachable!(),
    }
}