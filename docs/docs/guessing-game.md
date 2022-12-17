# 猜数游戏

## 创建新项目

```shell
cargo new guessing_game
cd guessing_game
```

## 处理一次猜测输入

文件名: src/main.rs

```rust
// 预导入
// io输入/输出库引入当前作用域; 
// io 库来自于标准库，也被称为 std
use std::io;  

// main 函数是程序的入口点
// fn 语法声明了一个新函数
// () 表明没有参数
// { 作为函数体的开始
fn main() {
    println!("Guess the number!");          // println! 是一个在屏幕上打印字符串的宏

    println!("Please input your guess.");   // println! 是一个在屏幕上打印字符串的宏

    // 创建一个 变量（variable）来储存用户输入
    // 在 Rust 中，变量默认是不可变的，这意味着一旦我们给变量赋值，这个值就不再可以修改了。 
    let mut guess = String::new();         // 在变量名前使用 mut 来使一个变量可变
    // String 是一个标准库提供的字符串类型，它是 UTF-8 编码的可增长文本块。
    // ::new 那一行的 :: 语法表明 new 是 String 类型的一个 关联函数（associated function）。
    // 关联函数是针对类型实现的，在这个例子中是 String，而不是 String 的某个特定实例。一些语言中把它称为 静态方法（static method）。
    // new 函数创建了一个新的空字符串，你会发现很多类型上有 new 函数，因为它是创建类型实例的惯用函数名。
    // 总的来说，let mut guess = String::new(); 这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上。


    // 如果程序的开头没有使用 use std::io 引入 io 库，我们仍可以通过把函数调用写成 std::io::stdin 来使用函数。
    // stdin 函数返回一个 std::io::Stdin 的实例，这代表终端标准输入句柄的类型。
    io::stdin()
        .read_line(&mut guess)              // 从标准输入句柄获取用户输入。
            // 将 &mut guess 作为参数传递给 read_line() 函数，让其将用户输入储存到这个字符串中。
            // read_line 的工作是，无论用户在标准输入中键入什么内容，都将其追加（不会覆盖其原有内容）到一个字符串中，因此它需要字符串作为参数。
            // 这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去。
            // & 表示这个参数是一个 引用（reference），它允许多处代码访问同一处数据，而无需在内存中多次拷贝。
        .expect("Failed to read line");
            // 会返回一个类型为 Result 的值。 Result 是一种枚举类型，通常也写作 enum。
            // 枚举类型变量的值可以是多种可能状态中的一个。我们把每种可能的状态称为一种 枚举成员（variant）。
            // 这里的 Result 类型将用来编码错误处理的信息。

            // Result 的成员是 Ok 和 Err，
            // Ok 成员表示操作成功，内部包含成功时产生的值。
            // Err 成员则意味着操作失败，并且包含失败的前因后果。

            // Result 类型的值，像其他类型一样，拥有定义于其上的方法。Result 的实例拥有 expect 方法。
            // 如果 io::Result 实例的值是 Err，expect 会导致程序崩溃，并显示当做参数传递给 expect 的信息。
            // 如果 read_line 方法返回 Err，则可能是来源于底层操作系统错误的结果。
            // 如果 Result 实例的值是 Ok，expect 会获取 Ok 中的值并原样返回。
            // 在本例中，这个值是用户输入到标准输入中的字节数。

            // 如果不调用 expect，程序也能编译，不过会出现一个警告： 
            // note: this `Result` may be an `Err` variant, which should be handled

            // Rust 警告我们没有使用 read_line 的返回值 Result，说明有一个可能的错误没有处理。
            // 消除警告的正确做法是实际去编写错误处理代码，不过由于我们就是希望程序在出现问题时立即崩溃，所以直接使用 expect。
            // 继续深入学习的话，将会学习到如何从错误中恢复。

    println!("You guessed: {guess}");
        // 打印了存储用户输入的字符串。
        // 第一个参数是格式化字符串，里面的 {} 是预留在特定位置的占位符
        // 使用 {} 也可以打印多个值：
        // 第一对 {} 使用格式化字符串之后的第一个值，第二对则使用第二个值，依此类推。调
        // 用一次 println! 打印多个值看起来像这样：
        // println!("x = {} and y = {}", x, y);
}
```

## 生成一个秘密数字

Rust 标准库中尚未包含随机数功能。然而，Rust 团队还是提供了一个包含上述功能的 [rand crate](https://crates.io/crates/rand)。

[rand crate](https://crates.io/crates/rand) 是一个 **库 crate**，**库 crate** 可以包含任意能被其他程序使用的代码，但是不能自执行。

在使用 rand 编写代码之前，需要修改 `Cargo.toml` 文件，引入一个 `rand` 依赖。现在打开这个文件并将下面这一行添加到 \[dependencies\] 片段标题之下。

文件名: Cargo.toml

```toml
rand = "0.8.3"
```

\[dependencies\] 片段告诉 Cargo 本项目依赖了哪些外部 crate 及其版本。本例中，我们使用语义化版本 `0.8.3` 来指定 [rand crate](https://crates.io/crates/rand)的版本。Cargo 理解 [语义化版本](http://semver.org/)（Semantic Versioning）（有时也称为 SemVer），这是一种定义版本号的标准。`0.8.3` 事实上是 `^0.8.3` 的简写，它表示任何至少是 `0.8.3` 但小于 `0.9.0` 的版本。

Cargo 认为这些版本与 0.8.3 版本的公有 API 相兼容，这样的版本指定确保了我们可以获取能使本章代码编译的最新的补丁（patch）版本。任何大于等于 0.9.0 的版本不能保证和接下来的示例采用了相同的 API。

现在，不修改任何代码，构建项目，如下所示：

```shell
$ cargo build
    Updating crates.io index
  Downloaded ppv-lite86 v0.2.17
  Downloaded rand v0.8.5
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.4
  Downloaded cfg-if v1.0.0
  Downloaded getrandom v0.2.8
  Downloaded libc v0.2.138
  Downloaded 7 crates (794.8 KB) in 2.72s
   Compiling libc v0.2.138
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.17
   Compiling getrandom v0.2.8
   Compiling rand_core v0.6.4
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing-game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 3m 45s
# 将 rand crate 添加为依赖之后运行 cargo build 的输出
```

现在系统有了一个外部依赖，Cargo 从 registry 上获取所有包的最新版本信息，这是一份来自 [Crates.io](https://crates.io/) 的数据拷贝。Crates.io 是 Rust 生态环境中的开发者们向他人贡献 Rust 开源项目的地方。

在更新完 registry 后，Cargo 检查 \[dependencies\] 部分并下载列表中包含但还未下载的 crates 。本例中，虽然只声明了 rand 一个依赖，然而 Cargo 还是额外获取了 rand 所需要的其他 crates，因为 rand 依赖它们来正常工作。下载完成后，Rust 编译依赖，然后使用这些依赖编译项目。

文件名: src/main.rs

```rust
// 新增了一行 use rand::Rng。
// Rng 是一个 trait，它定义了随机数生成器应实现的方法，想使用这些方法的话，此 trait 必须在作用域中。
use rand::Rng;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);
        // 调用了 rand::thread_rng 函数提供实际使用的随机数生成器：它位于当前执行线程的本地环境中，并从操作系统获取 seed。
        // 接着调用随机数生成器的 gen_range 方法。这个方法由 use rand::Rng 语句引入到作用域的 Rng trait 定义。
        // gen_range 方法获取一个范围表达式（range expression）作为参数，并生成一个在此范围之间的随机数。
        // 这里使用的这类范围表达式使用了 start..=end 这样的形式，也就是说包含了上下端点，所以需要指定 1..=100 来请求一个 1 和 100 之间的数。

    println!("生成的密码数字是: {secret_number}");

    println!("请输入一个值:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读数失败!");

    println!("你猜测的数为: {guess}")
}
```

> **注意**：
>
> 你不可能凭空就知道应该 use 哪个 trait 以及该从 crate 中调用哪个方法，因此每个crate 有使用说明文档。
>
> Cargo 有一个很棒的功能是：运行 `cargo doc --open` 命令来构建所有本地依赖提供的文档，并在浏览器中打开。
>
> 例如，假设你对 `rand crate` 中的其他功能感兴趣，你可以运行 `cargo doc --open` 并点击左侧导航栏中的 rand。

## 比较猜测的数字和秘密数字

文件名: src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
    // 从标准库引入了一个叫做 std::cmp::Ordering 的类型到作用域中。 
    // Ordering 也是一个枚举，不过它的成员是 Less、Greater 和 Equal。
    // 这是比较两个值时可能出现的三种结果。
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("生成的密码数字是: {secret_number}");

    println!("请输入一个值:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读数失败!");

    println!("你猜测的数为: {guess}");

    match guess.cmp(&secret_number){
        // cmp 方法用来比较两个值并可以在任何可比较的值上调用。
        // 它获取一个被比较值的引用：这里是把 guess 与 secret_number 做比较。
        // 然后它会返回一个刚才通过 use 引入作用域的 Ordering 枚举的成员。
        // 使用一个 match 表达式，根据对 guess 和 secret_number 调用 cmp 返回的 Ordering 成员来决定接下来做什么。
        Ordering::Less => println!("太小了！"),
        Ordering::Greater => println!("太大了！"),
        Ordering::Equal => println!("猜对了！"),
    }
}
```

一个 match 表达式由 分支（arms） 构成。

一个分支包含一个 模式（pattern）和表达式开头的值与分支模式相匹配时应该执行的代码。

Rust 获取提供给 match 的值并挨个检查每个分支的模式。

此时进行 `cargo build` 会出现如下错误:

```shell
cargo build
   Compiling guessing-game v0.1.0 (/files/rust-example-apps/src/guessing-game)
error[E0308]: mismatched types
  --> src/main.rs:20:21
   |
20 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: associated function defined here

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing-game` due to previous error
```

错误的核心表明这里有 **不匹配的类型（mismatched types）**。

Rust 有一个静态强类型系统，同时也有类型推断。

当我们写出 let guess = String::new() 时，Rust 推断出 guess 应该是 String 类型，并不需要我们写出类型。

另一方面，secret_number，是数字类型。几个数字类型拥有 1 到 100 之间的值：32 位数字 i32；32 位无符号数字 u32；64 位数字 i64 等等。

Rust 默认使用 i32，所以它是 secret_number 的类型，除非增加类型信息，或任何能让 Rust 推断出不同数值类型的信息。

这里错误的原因在于 Rust 不会比较字符串类型和数字类型。

所以我们必须把从输入中读取到的 String 转换为一个真正的数字类型，才好与秘密数字进行比较。这可以通过在 main 函数体中增加如下代码来实现：

文件名: src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("生成的密码数字是: {secret_number}");

    println!("请输入一个值:");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("读数失败!");

    let guess: u32 = guess.trim().parse().expect("请输入数字!");
        // 这里创建了一个叫做 guess 的变量。
        // 不过等等，不是已经有了一个叫做 guess 的变量了吗？
        // 确实如此，不过 Rust 允许用一个新值来 隐藏 （shadow） guess 之前的值。
        // 这个功能常用在需要转换值类型之类的场景。
        // 它允许我们复用 guess 变量的名字，而不是被迫创建两个不同变量，诸如 guess_str 和 guess 之类。

        // 将这个新变量绑定到 guess.trim().parse() 表达式上。
        // 表达式中的 guess 指的是包含输入的字符串类型 guess 变量。
        // String 实例的 trim 方法会去除字符串开头和结尾的空白字符，我们必须执行此方法才能将字符串与 u32 比较，因为 u32 只能包含数值型数据。
        // 用户必须输入 enter 键才能让 read_line 返回并输入他们的猜想，这将会在字符串中增加一个换行（newline）符。
        // 例如，用户输入 5 并按下 enter, guess 看起来像这样：5\n 或者 5\r\n。
        // \n 代表 “换行”，回车键；\r 代表 “回车”，回车键。
        // trim 方法会消除 \n 或者 \r\n，只留下 5。

        // 字符串的 parse 方法 将字符串转换成其他类型。
        // 这里用它来把字符串转换为数值。我们需要告诉 Rust 具体的数字类型，这里通过 let guess: u32 指定。
        // guess 后面的冒号（:）告诉 Rust 我们指定了变量的类型。
        // Rust 有一些内建的数字类型；u32 是一个无符号的 32 位整型。
        // 对于不大的正整数来说，它是不错的默认类型，第三章还会讲到其他数字类型。
        // 另外，程序中的 u32 注解以及与 secret_number 的比较，意味着 Rust 会推断出 secret_number 也是 u32 类型。
        // 现在可以使用相同类型比较两个值了！

        // parse 方法只有在字符逻辑上可以转换为数字的时候才能工作所以非常容易出错。
        // 例如，字符串中包含 A👍%，就无法将其转换为一个数字。
        // 因此，parse 方法返回一个 Result 类型。
        // 像之前 “使用 Result 类型来处理潜在的错误” 讨论的 read_line 方法那样，再次按部就班的用 expect 方法处理即可。
        // 如果 parse 不能从字符串生成一个数字，返回一个 Result 的 Err 成员时，expect 会使游戏崩溃并打印附带的信息。
        // 如果 parse 成功地将字符串转换为一个数字，它会返回 Result 的 Ok 成员，然后 expect 会返回 Ok 值中的数字。



    println!("你猜测的数为: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("太小了！"),
        Ordering::Greater => println!("太大了！"),
        Ordering::Equal => println!("猜对了！"),
    }
}

```

## 使用循环来允许多次猜测

loop 关键字创建了一个无限循环。

文件名: src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("生成的密码数字是: {secret_number}");

    loop {
        println!("请输入一个值:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读数失败!");

        let guess: u32 = guess.trim().parse().expect("请输入数字!");

        println!("你猜测的数为: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => println!("猜对了！"),
        }
    }
}
```

如上所示，我们将提示用户猜测之后的所有内容移动到了循环中。确保 loop 循环中的代码多缩进四个空格，再次运行程序。注意这里有一个新问题，因为程序忠实地执行了我们的要求：永远地请求另一个猜测，用户好像无法退出啊！

用户总能使用 ctrl-c 终止程序。不过还有另一个方法跳出无限循环，就是 “比较猜测与秘密数字” 部分提到的 parse：如果用户输入的答案不是一个数字，程序会崩溃。我们可以利用这一点来退出，如下所示：

```shell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
猜数游戏！！
生成的密码数字是: 57
请输入一个值:
12
你猜测的数为: 12
太小了！
请输入一个值:
57
你猜测的数为: 57
猜对了！
请输入一个值:
asdf
thread 'main' panicked at '请输入数字!: ParseIntError { kind: InvalidDigit }', src/main.rs:19:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### 猜测正确后退出

让我们增加一个 break 语句，在用户猜对时退出游戏：

文件名: src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("生成的密码数字是: {secret_number}");

    loop {
        println!("请输入一个值:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读数失败!");

        let guess: u32 = guess.trim().parse().expect("请输入数字!");

        println!("你猜测的数为: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}a

```

通过在 You win! 之后增加一行 break，用户猜对了神秘数字后会退出循环。退出循环也意味着退出程序，因为循环是 main 的最后一部分。

### 处理无效输入

为了进一步改善游戏性，不要在用户输入非数字时崩溃，需要忽略非数字，让用户可以继续猜测。可以通过修改 guess 将 String 转化为 u32 那部分代码来实现

文件名: src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("生成的密码数字是: {secret_number}");

    loop {
        println!("请输入一个值:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读数失败!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            // 将 expect 调用换成 match 语句，以从遇到错误就崩溃转换为处理错误。
            // 须知 parse 返回一个 Result 类型，而 Result 是一个拥有 Ok 或 Err 成员的枚举。
            // 这里使用的 match 表达式，和处理 cmp 方法返回 Ordering 时用的一样。

            // 如果 parse 能够成功的将字符串转换为一个数字，它会返回一个包含结果数字的 Ok。
            // 这个 Ok 值与 match 第一个分支的模式相匹配，该分支对应的动作返回 Ok 值中的数字 num，
            // 最后如愿变成新创建的 guess 变量。

            // 如果 parse 不能将字符串转换为一个数字，它会返回一个包含更多错误信息的 Err。
            // Err 值不能匹配第一个 match 分支的 Ok(num) 模式，但是会匹配第二个分支的 Err(_) 模式：
            // _ 是一个通配符值，本例中用来匹配所有 Err 值，不管其中有何种信息。
            // 所以程序会执行第二个分支的动作，continue 意味着进入 loop 的下一次循环，请求另一个猜测。
            // 这样程序就有效的忽略了 parse 可能遇到的所有错误！

        println!("你猜测的数为: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
```

现在万事俱备，只需运行 cargo run：

```shell
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
猜数游戏！！
生成的密码数字是: 66
请输入一个值:
12d
请输入一个值:
12
你猜测的数为: 12
太小了！
请输入一个值:
66
你猜测的数为: 66
猜对了！
```

### 最终版本

太棒了！再有最后一个小的修改，就能完成猜猜看游戏了：还记得程序依然会打印出秘密数字。在测试时还好，但正式发布时会毁了游戏。删掉打印秘密数字的 println!。

最终代码：

文件名: src/main.rs

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入一个值:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读数失败!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数为: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
```
