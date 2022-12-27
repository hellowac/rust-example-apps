use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    println!("{:?}", args);
    println!("搜索的字符串为 {}", query);
    println!("搜索的文件 {} ", filename);

    let contents = fs::read_to_string(filename).expect("读取文件失败");

    println!("文件内容: \n{}", contents)
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
