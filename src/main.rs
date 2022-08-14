mod app;
mod core;
mod config;
mod download;

use app::model::{Cmd};
use clap::{arg, ArgMatches, Command};

const VERSION :&str = "0.1.0";
const AUTHOR : &str = "weblinuxgame@126.com";
const APPNAME :&str = "protobuf_install";
const USAGE: &str =  "protobuf_install [OPTIONS]";

/// 创建子命令行
fn create_cmds() -> Vec<Command<'static>> {
    return vec![
        Command::new("list")
            .about("list protobuf versions")
            .arg(arg!(-v --version [VERSION] "The protobuf version"))
            .arg(arg!(-t --table [TABLE] "Show the protobuf view format table"))
            .arg_required_else_help(true),
        Command::new("version").about("show tool version").arg_required_else_help(true),
    ]
}

/// 构建应用
fn new_app() -> Cmd<'static> {
    return Cmd::form(||->Command<'static>{
        Command::new(APPNAME).
            author(AUTHOR).
            version(VERSION).
            override_usage(USAGE).
            arg_required_else_help(true).
            arg(arg!(-y --yes [YES] "force install protobuf bin")).
            arg(arg!(-c --config [CONFIG] "protobuf installer config file")).
            arg(arg!(-p --prefix [PREFIX] "protobuf install prefix dir")).
            arg(arg!(-m --mirror [MIRROR] "protobuf repo mirror address")).
            arg(arg!(-l --local  [LOCAL] "use local cache for protobuf")).
            arg(arg!(-v --version [VERSION] "print the tool version")).
            subcommands(create_cmds())
    }).add_runner("list",core::list_runner).
    add_runner("version",|key:&str,args:&ArgMatches| {
        core::version_runner(VERSION,key,args)
    }). // 版本号
    add_runner(app::core::MAIN_RUNNER_KEY,core::download_runner) // 核心主逻辑
}

/// 构建cli应用
///
fn main() {
   // 构建应用
    new_app().run()
}
