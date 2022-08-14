use clap::ArgMatches;

mod list;
mod download;


#[allow(unused)]
pub fn version_runner(version :&str,key : &str, args : &ArgMatches) {
    println!("version {}",version)
}


pub fn list_runner (key : &str, args : &ArgMatches) {
    list::list(key,args)
}


pub fn download_runner (key : &str, args : &ArgMatches) {
    download::download(key,args)
}
