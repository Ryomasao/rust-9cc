extern crate rust9cc;
use std::env;
use std::process;

use rust9cc::run;
use rust9cc::Config;

fn main() {
    // Configパースが必要になったら復活する
    //let config = Config::new(env::args()).unwrap_or_else(|err| {
    //    eprintln!("引数のパースに失敗! :{}, 引数:{:?}", err, env::args());
    //    process::exit(1);
    //});

    let config = Config::new(env::args());

    if let Err(e) = run(&config) {
        eprintln!("コンパイルに失敗! \n config:{:#?}\n error:{} \n", config, e);
        process::exit(1);
    }
}
