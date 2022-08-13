use std::collections::HashMap;
use clap::{ArgMatches, Command};


pub struct Cmd<'help> {
     app : Command<'help>,
     exec: fn(Command),
     runner: HashMap<String,fn(&str,&ArgMatches)>,
}


#[allow(unused)]
impl Cmd<'static> {

    /// 构建命令行
    /// ```rust
    /// let app = Cmd::new(Command::new());
    /// ```
    pub fn new(c : Command<'static>) -> Cmd {
        return Cmd{
            app: c,
            exec: super::core::run,
            runner: HashMap::new(),
        }
    }

    /// 构建命令
    /// ```rust
    /// fn new() ->Command<'static> {
    ///     return Command::new();
    /// }
    /// let app = Cmd::form(new);
    ///     app.run();
    /// ```
    pub fn form( c: fn()->Command<'static>) ->Cmd<'static> {
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
    pub fn add_runner(mut self,name:&str,exec : fn(&str,&ArgMatches)) ->Self {
        self.runner.insert(name.to_string(),exec);
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
        super::core::exec(self.app,self.runner);
    }

}