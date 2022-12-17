# Hello Cargo

Cargo 是 Rust 的构建系统和包管理器。

## 安装cargo

官方自带的[安装](https://kaisery.github.io/trpl-zh-cn/ch01-01-installation.html#installation)部分已经安装了Cargo

## 查看版本

```shell
$ cargo --version
cargo 1.66.0 (d65d197ad 2022-11-15)
```

## 使用cargo创建项目

### cargo new

Cargo 将代码放在 src 目录，同时项目根目录包含一个 Cargo.toml 配置文件。

```shell
$ cargo new hello-cargo
$ cd hello-cargo
$ ls
Cargo.toml src
```

#### Cargo.toml

配置文件， 如下:

```toml
# 一个片段（section）标题，表明下面的语句用来配置一个包
[package]
name = "hello-cargo"  # 项目的名称
version = "0.1.0"     # 项目的版本
edition = "2021"      # Rust 版本

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

# 罗列项目依赖的代码包, 代码包被称为 crates。这个项目并不需要其他的 crate
[dependencies]
```

[TOML](https://toml.io/)格式是 Cargo 配置文件的格式。

`edition` 的值参考[附录E](https://kaisery.github.io/trpl-zh-cn/appendix-05-editions.html)

#### src

项目源文件存放在 src 目录中。

项目根目录只存放 `README`、`license` 信息、**配置文件**和**其他跟代码无关的文件**。

##### src/main.rs

Cargo 为生成的一个 “Hello, world!” 程序

```rust
fn main() {
    println!("Hello, world!");
}
```

## 构建并运行 Cargo 项目

通过 Cargo 构建和运行 “Hello, world!” 程序

### cargo build

```shell
# 在hello-cargo目录执行
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
$ ls
Cargo.lock Cargo.toml src        target  # 会创建一个可执行文件 target/debug/hello-cargo, 而不是放在目前目录下。
# 由于默认的构建方法是调试构建（debug build），Cargo 会将可执行文件放在名为 debug 的目录中。 运行可执行文件:
$ ./target/debug/hello-cargo 
Hello, world!
```

首次运行 `cargo build` 时，也会使 Cargo 在项目根目录创建一个新文件：`Cargo.lock`。这个文件记录项目依赖的实际版本。这个项目并没有依赖，所以其内容比较少。自己永远也不需要碰这个文件，让 Cargo 处理它就行了。

### cargo run

可以使用 cargo run 在一个命令中同时编译并运行生成的可执行文件：

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/hello-cargo`
Hello, world!
```

Cargo 发现文件并没有被改变，所以它并没有重新编译，而是直接运行了可执行文件。

如果修改了源文件的话，Cargo 会在运行之前重新构建项目，并会出现像这样的输出：

```shell
$ cargo run
   Compiling hello-cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.33 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

### cargo check

该命令快速检查代码确保其可以编译，但并不产生可执行文件:

```shell
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

> **为什么你会不需要可执行文件呢？**
>
> 通常 `cargo check` 要比 `cargo build` 快得多，因为它省略了生成可执行文件的步骤。
> 如果你在编写代码时持续的进行检查，`cargo check` 可以让你快速了解现在的代码能不能正常通过编译！
> 为此很多 Rustaceans 编写代码时定期运行 `cargo check` 确保它们可以编译。
> 当准备好使用可执行文件时才运行 `cargo build`。

## 其他

### git仓库

Cargo创建项目时，会初始化一个 git 仓库，以及一个 `.gitignore` 文件。如果在一个已经存在的 git 仓库中运行 `cargo new`，则这些 git 相关文件则不会生成；可以通过运行 `cargo new --vcs=git` 来覆盖这些行为。

### 发布（release）构建

当项目最终准备好发布时，可以使用 `cargo build --release` 来优化编译项目。
这会在 target/release 而不是 target/debug 下生成可执行文件。
这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。
这也就是为什么会有两种不同的配置：
一种是为了开发，你需要经常快速重新构建；
另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。
如果你在测试代码的运行时间，请确保运行 `cargo build --release` 并使用 `target/release` 下的可执行文件进行测试。

### 把 Cargo 当作习惯

对于简单项目， Cargo 并不比 rustc 提供了更多的优势，不过随着开发的深入，终将证明其价值。
一旦程序壮大到由多个文件组成，亦或者是需要其他的依赖，让 Cargo 协调构建过程就会简单得多。

即便 hello_cargo 项目十分简单，它现在也使用了很多在你之后的 Rust 生涯将会用到的实用工具。
其实，要在任何已存在的项目上工作时，可以使用如下命令通过 Git 检出代码，移动到该项目目录并构建：

```shell
git clone example.org/someproject
cd someproject
cargo build
```

关于更多 Cargo 的信息，请查阅[其文档](https://doc.rust-lang.org/cargo/)。
