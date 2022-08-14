use clap::ArgMatches;

#[allow(unused)]
pub fn list(name : &str, args : &ArgMatches ) {

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

