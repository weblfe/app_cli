mod app;
mod core;
mod config;
mod download;
mod output;


extern crate libc;

use libc::c_char;
use app::model::{Cmd as App};
use std::ffi::{ CStr,c_int};
use clap::{arg, ArgMatches, Command};

// 引入hash
include!(concat!(env!("OUT_DIR"), "/git.rs"));

extern {
    fn display_name();
    fn max(n1:c_int,n2:c_int) ->c_int;
    fn get_module_name() ->*const c_char;
}

const LIST_CMD: &str = "list";
const VERSION_CMD: &str = "version";

const AUTHOR: &str = "weblinuxgame@126.com";
const APPNAME: &str = "protobuf_install";
const USAGE: &str = "protobuf_install [OPTIONS]";
const VERSION: &str = "0.1.0";


/// 构建应用
fn new_app() -> App<'static> {
    let version_runner = move |key: &str, args: &ArgMatches| {
        let hash : &str = version_id();
        core::version_runner(VERSION.to_owned() + "-" + hash, key, args)
    };

    let  cmd = Command::new(APPNAME).
        author(AUTHOR).
        version(VERSION).
        override_usage(USAGE).
        arg_required_else_help(true);

    let list_cmd = Command::new(LIST_CMD)
        .about("list protobuf versions")
        .arg(arg!(-v --version [VERSION] "The protobuf version"))
        .arg(arg!(-t --table [TABLE] "Show the protobuf view format table"))
        .arg_required_else_help(true);

    let version_cmd = Command::new(VERSION_CMD).
        about("show tool version").
        arg_required_else_help(false);

    return App::new(cmd).add_arg(arg!(-y --yes [YES] "force install protobuf bin")).
        add_arg(arg!(-c --config [CONFIG] "protobuf installer config file")).
        add_arg(arg!(-p --prefix [PREFIX] "protobuf install prefix dir")).
        add_arg(arg!(-m --mirror [MIRROR] "protobuf repo mirror address")).
        add_arg(arg!(-l --local  [LOCAL] "use local cache for protobuf")).
        add_arg(arg!(-v --version [VERSION] "print the tool version")).
        add_command(list_cmd, core::list_runner).
        add_command(version_cmd, version_runner).
        set_default(core::main_runner);
}

fn init() {
    unsafe {
        let n= 10;
        let n2  = 20;
        let name = get_module_name();
        let max = max(n as c_int,n2 as c_int);
        display_name();
        println!("max {}",max as i32);
        println!("hello {}",CStr::from_ptr(name).to_str().unwrap());
    }
}

/// 构建cli应用
///
fn main() {
    init();
    // 构建应用
    new_app().run();

}
