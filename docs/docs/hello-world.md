# hello world

## 创建rs文件

创建`helloworld.rs`文件

```rust
fn main(){
    println!("Hello, world!");
}
```

## 编译

```shell
# 编译源文件
rustc helloworld.rs
# 生成 helloworld 可执行文件.
```

## 执行

```shell
# 执行可执行文件
./helloworld
# 输出:
Hello, world!
```

## 释义

1. `main`函数: 在可执行的 Rust 程序中，它总是最先运行的代码。 参考: [分析这个 Rust 程序](https://kaisery.github.io/trpl-zh-cn/ch01-02-hello-world.html#%E5%88%86%E6%9E%90%E8%BF%99%E4%B8%AA-rust-%E7%A8%8B%E5%BA%8F)
2. `println!`函数: 一个 Rust 宏（macro）。
3. "Hello, world!": 是一个字符串。我们把这个字符串作为一个参数传递给 println!，字符串将被打印到屏幕上。
4. 该行以分号结尾（;）: 这代表一个表达式的结束和下一个表达式的开始。大部分 Rust 代码行以分号结尾。

## 编译和运行是彼此独立的步骤

```shell
# 运行 Rust 程序之前，必须先使用 Rust 编译器编译,
$ rustc main.rs  
# 输出一个二进制的可执行文件, 在 shell 中输入 ls 命令可以看见这个可执行文件
$ ls
helloword    helloword.rs
# 从这里开始运行 helloword 文件
$ ./helloword 
Hello, world!
```

> `Rust` 是一种 预编译静态类型（ahead-of-time compiled）语言，这意味着你可以编译程序，并将可执行文件送给其他人，他们甚至不需要安装 Rust 就可以运行。
