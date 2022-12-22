use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // simple1();
    // simple2();
    // simple3();
    read_username_from_file();
}

fn simple1() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => panic!("打开文件失败: {:?}", err),
    };
}

fn simple2() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            other_error => {
                panic!("打开文件失败: {:?}", other_error)
            }
        },
    };
}

fn simple3() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// 传播错误
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello1.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误, 运用 ? 运算符
fn read_username_from_file2() -> Result<String, io::Error> {
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    // 如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。

    // ? 将会把 Ok 中的值返回给变量 f, 如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    // ？运用同上
    f.read_to_string(&mut s)?;
    Ok(s) // 返回Result枚举的Ok值。
}

// 传播错误, 运用 ? 运算符 并进行链式调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// 尝试在返回 () 的函数中使用 ? 运算符的代码不能编译
fn erro_open() {
    // let f = File::open("hello.txt")?;
}

// 在 Option<T> 值上使用 ? 运算符
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
