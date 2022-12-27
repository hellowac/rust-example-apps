use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

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
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
