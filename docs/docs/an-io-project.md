# 构建一个命令行程序

本章既是一个目前所学的很多技能的概括，也是一个更多标准库功能的探索。我们将构建一个与文件和命令行输入/输出交互的命令行工具来练习现在一些你已经掌握的 Rust 技能。

Rust 的**运行速度**、**安全性**、**单二进制文件输出**和**跨平台支持**使其成为创建命令行程序的绝佳选择，所以我们的项目将创建一个我们自己版本的经典命令行工具：**grep**。
**grep** 是 “Globally search a Regular Expression and Print.” 的首字母缩写。**grep** 最简单的使用场景是在特定文件中搜索指定字符串。
为此，**grep** 获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。

在这个过程中，我们会展示如何让我们的命令行工具利用很多命令行工具中用到的终端功能。读取环境变量来使得用户可以配置工具的行为。
打印到**标准错误控制流**（stderr） 而不是**标准输出**（stdout），例如这样用户可以选择将成功输出重定向到文件中的同时仍然在屏幕上显示错误信息。

一位 Rust 社区的成员，Andrew Gallant，已经创建了一个功能完整且非常快速的 **grep** 版本，叫做 **ripgrep**。相比之下，我们的 **grep** 版本将非常简单，本章将教会你一些帮助理解像 **ripgrep** 这样真实项目的背景知识。

我们的 grep 项目将会结合之前所学的一些内容：

- **代码组织**（使用 第七章 学习的模块）
- **vector** 和 **字符串**（第八章，集合）
- **错误处理**（第九章）
- **合理的使用** **trait** 和**生命周期**（第十章）
- **测试**（第十一章）

另外还会简要的讲到**闭包**、**迭代器**和 **trait 对象**，他们分别会在 第十三章 和 第十七章 中详细介绍。

## 接受命令行参数

**版本1**:

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("{:?}", args);
    println!("搜索的字符串为 {}", query);
    println!("搜索的文件 {} ", filename);

    let contents = fs::read_to_string(filename).expect("读取文件失败");

    println!("文件内容: \n{}", contents)
}
```

**版本2**:

```rust
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
```

**版本3**:

```rust
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

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

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
```

## 读取文件

```rust
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
```

**版本5**:

```rust
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
```

## 重构改进模块性和错误处理

**版本6**:

```rust
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
    println!("搜索的字符串为: {}", config.query);
    println!("搜索的文件: {} ", config.filename);

    run(config);
}

fn run(config: Config) {
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
```

**版本7**:

```rust
use std::env;
use std::error::Error;
use std::fs;
use std::process;

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

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("文件内容: \n{}", contents);

    Ok(())
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
```

## 采用测试驱动开发完善库的功能

**版本8**:

```rust
// src/lib.rs 文件

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // println!("文件内容: \n{}", contents);

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // 这告诉 Rust 不要在字符串字面值内容的开头加入换行符
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

```rust
// src/main.rs

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

```

## 处理环境变量

**版本9:**

```rust
// src/lib.rs

use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("参数不够");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    // println!("文件内容: \n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        // 这告诉 Rust 不要在字符串字面值内容的开头加入换行符
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

```

```rust
// src/main.rs

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

```

## 输出重定向到标准错误

**版本10:**

```rust
// src/main.rs

use std::env;
use std::process;

use minigrep::run;
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("解析参数错误: {}", err);
        process::exit(1);
    });

    // println!("{:?}", args);
    // println!("搜索的字符串为: {}", config.query);
    // println!("搜索的文件: {} ", config.filename);

    if let Err(e) = run(config) {
        eprintln!("程序错误: {}", e);
        process::exit(1);
    }
}
```
