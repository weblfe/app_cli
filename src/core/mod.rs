use clap::ArgMatches;

pub mod core;
pub mod list;


#[allow(unused)]
pub fn version_runner(version : String, key : &str, args : &ArgMatches) {
    println!("version {}",version)
}


pub fn list_runner (key : &str, args : &ArgMatches) {
    list::list(key,args)
}


pub fn main_runner(key : &str, args : &ArgMatches) {
    core::download(key, args)
}

#[cfg(all(not(no_global_oom_handling), not(test)))]
#[macro_export]
macro_rules! hashmap {
    () => (
        {
            std::collections::HashMap::new()
        }
    );
    ($(($key:expr,$value:expr)),*) => (
        {
            std::collections::HashMap::from([
                $(($key,$value)),*
            ])
        }
    );
}