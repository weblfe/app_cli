use clap::ArgMatches;

#[allow(unused)]
pub fn list(name : &str, args : &ArgMatches ) {

}


#[cfg(all(not(no_global_oom_handling), not(test)))]
#[macro_export]
macro_rules! hashmap {
    () => (
        $crate::std::collections::HashMap::new()
    );
    (($key:expr,$value:expr)) => (
        $crate::std::collections::HashMap::from([
            ($key ,$value),
        ])
    );
    (($key:expr,$value:expr),$($args:tt)*) => {
        hashmap(($key,$value),$($b)*)
    };
}

