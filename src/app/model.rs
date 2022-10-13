use std::collections::HashMap;
use clap::{Command,Arg};


pub struct Cmd {
     app : Command,
     exec: fn(Command),
     runner: HashMap<String,super::app::Runner>,
}


#[allow(unused)]
impl Cmd {

    /// 构建命令行
    /// ```rust
    /// let app = Cmd::new(Command::new());
    /// ```
    pub fn new(c : Command) -> Cmd {
        return Cmd{
            app: c,
            exec: super::app::run,
            runner: HashMap::new(),
        }
    }

    /// 构建命令
    /// ```rust
    /// fn new() ->Command<'static> {
    ///     return Command::new();
    /// }
    /// let app = Cmd::from(new);
    ///     app.run();
    /// ```
    pub fn from(c: fn() ->Command) ->Cmd {
        return Cmd::new(c());
    }

    /// 绑定执行逻辑
    /// ```rust
    ///  fn runner(app:Command) {
    ///     !todo();
    ///  }
    ///  let app = Cmd::new(Command::new());
    ///      app.bind_exec(runner);
    ///      app.run();
    /// ```
    pub fn bind_exec(mut self, exec :fn(Command)) ->Self {
        self.exec = exec;
        return self;
    }

    /// 添加执行相关逻辑
    /// ```rust
    ///  fn runner(name:&str,args:&ArgMatches) {
    ///     !todo();
    ///  }
    ///  Cmd::new(Command::new()).
    ///      add_runner("app",runner).
    ///      run();
    /// ```
    pub fn add_runner(mut self, name:&str, exec : super::app::Runner) ->Self {
        self.runner.insert(name.to_string(),exec);
        return self;
    }

    pub fn set_default(mut self, exec : super::app::Runner) ->Self {
        self.runner.insert(super::app::DEFAULT_RUNNER_KEY.to_string(), exec);
        return self;
    }

    /// 添加命令应用options参数
    /// ```rust
    ///  Cmd::new(Command::new()).
    ///      add_arg(arg!(-y --yes [YES] "force install protobuf bin")).
    ///      run();
    /// ```
    pub fn add_arg(mut self, a: impl Into<Arg>) -> Self {
        self.app = self.app.arg(a);
        return self;
    }

    /// 添加命令应用子命令
    ///
    /// ```rust
    ///  Cmd::new(Command::new()).
    ///      add_command(Command::new(LIST_CMD)
    ///      .about("list protobuf versions")
    ///      .arg(arg!(-v --version [VERSION] "The protobuf version"))
    ///      .arg(arg!(-t --table [TABLE] "Show the protobuf view format table"))
    ///      .arg_required_else_help(true)).
    ///      run();
    /// ```
    pub fn add_command (mut self, subcmd:impl Into<Command>, handler: super::app::Runner) -> Self {
        let binging = subcmd.into();
        self.runner.insert(binging.get_name().to_string(),handler);
        self.app = self.app.subcommand(binging);
        return self;
    }

    /// 批量添加
    /// ```rust
    ///   let m = hashmap![|n:&str,args:&ArgMatches|{} as super::core::Runner];
    ///  Cmd::new(Command::new()).
    ///      batch_add_runners(m).
    ///      run();
    /// ```
    pub fn batch_add_runners(mut self, runners:HashMap<&str,super::app::Runner>) ->Self {
        for (key,runner) in runners {
            self.runner.insert(key.to_string(),runner);
        }
        return self;
    }

    /// 运行命令行
    /// ```rust
    ///  let app = Cmd::new(Command::new());
    ///      app.run();
    /// ```
    pub fn run(self) {
       if self.runner.len() <= 0 {
           let exec = self.exec;
           exec(self.app);
           return;
       }
        super::app::exec(self.app, self.runner);
    }

}