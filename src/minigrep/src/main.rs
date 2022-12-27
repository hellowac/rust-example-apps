use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("解析参数错误: {}", err);
        process::exit(1);
    });

    println!("{:?}", args);
    println!("搜索的字符串为 {}", config.query);
    println!("搜索的文件 {} ", config.filename);

    let contents = fs::read_to_string(config.filename).expect("读取文件失败");

    println!("文件内容: \n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
