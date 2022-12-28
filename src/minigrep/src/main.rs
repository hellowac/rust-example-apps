use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析参数错误: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);
    println!("搜索的字符串为: {}", config.query);
    println!("搜索的文件: {} ", config.filename);

    if let Err(e) = run(config) {
        println!("程序错误: {}", e);
        process::exit(1);
    }
}
