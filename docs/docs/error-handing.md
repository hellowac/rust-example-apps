# 错误处理

错误是软件中不可否认的事实，所以 Rust 有一些处理出错情况的特性。{++在许多情况下，Rust 要求你承认错误的可能性，并在代码编译前采取一些行动。++}这一要求使你的程序更加健壮，因为它可以确保你在将代码部署到生产环境之前就能发现错误并进行适当的处理。

Rust 将错误分为两大类：**可恢复的**（recoverable）和 **不可恢复的**（unrecoverable）错误。对于一个可恢复的错误，比如文件未找到的错误，很可能只想向用户报告问题并重试操作。不可恢复的错误总是 bug 出现的征兆，比如试图访问一个超过数组末端的位置，因此我们要立即停止程序。

大多数语言并不区分这两种错误，并采用类似异常这样方式统一处理他们。Rust 没有异常。相反，它有 `Result<T, E>` 类型，用于处理可恢复的错误，还有 `panic!` 宏，在程序遇到不可恢复的错误时停止执行。

## 不可恢复的错误(panic!)

突然有一天，代码出问题了，而你对此束手无策。对于这种情况，{++Rust 有 `panic!`宏。当执行这个宏时，程序会**打印出一个错误信息，展开并清理栈数据**，然后接着退出。++}出现这种情况的场景通常是检测到一些类型的 `bug`，而且程序员并不清楚该如何处理它。

> **对应 panic 时的栈展开或终止**
>
> 当出现 `panic` 时，程序默认会开始 **展开**（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
>
> 另一种选择是直接 **终止**（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。
> 如果你需要项目的最终二进制文件越小越好，`panic` 时通过在 **Cargo.toml** 的 **\[profile\]** 部分增加 `panic = 'abort'`，可以由展开切换为**终止**。
> 例如，如果你想要在**release**模式中 `panic` 时直接终止：
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```

```rust
fn main() {
    // 调用 panic!
    panic!("crash and burn");
}
```

将会出现类似这样的输出：

```shell
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

# 第一行显示了 panic 提供的信息并指明了源码中 panic 出现的位置：src/main.rs:2:5 表明这是 src/main.rs 文件的第二行第五个字符。
```

在这个例子中，被指明的那一行是我们代码的一部分，而且查看这一行的话就会发现 `panic!` 宏的调用。
在其他情况下，`panic!` 可能会出现在我们的代码所调用的代码中。
**错误信息报告的文件名和行号可能指向别人代码中的 `panic!` 宏调用，而不是我们代码中最终导致 `panic!` 的那一行。**
我们可以使用 `panic!` 被调用的函数的 **backtrace** 来寻找代码中出问题的地方。下面我们会详细介绍 **backtrace** 是什么。

### 使用 panic! 的 backtrace

另一个因为代码中的 `bug` 引起的别的库中 `panic!` 的例子，而不是直接的宏调用.

```rust
// 尝试访问超越 vector 结尾的元素，这会造成 panic!

fn main() {
    let v = vec![1, 2, 3];

    // 尝试访问 vector 的第一百个元素, 实际只有3个。
    // 这种情况下 Rust 会 panic。
    v[99];
}
```

C 语言中，尝试读取数据结构之后的值是**未定义行为**（undefined behavior）。
你**会得到任何对应数据结构中这个元素的内存位置的值，甚至是这些内存并不属于这个数据结构的情况。**
这被称为 **缓冲区溢出**（buffer overread），并可能会导致**安全漏洞**，比如攻击者可以像这样操作索引来读取储存在数据结构之后不被允许的数据。

**为了保护程序远离这类漏洞，如果尝试读取一个索引不存在的元素，Rust 会停止执行并拒绝继续。**

```shell
# 尝试运行上面的程序会出现如下：
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5  # 错误指向 main.rs 的第 4 行，
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
# 下面的说明（note）行提醒我们可以设置 RUST_BACKTRACE 环境变量来得到一个 backtrace。
```

`backtrace` 是一个执行到目前位置所有被调用的函数的列表。Rust 的 `backtrace` 跟其他语言中的一样：**阅读 `backtrace` 的关键是从头开始读直到发现你编写的文件。**
这就是问题的发源地。这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。这些行可能包含核心 Rust 代码，标准库代码或用到的 `crate` 代码。
将 **RUST_BACKTRACE** 环境变量设置为任何不是 `0` 的值来获取 **backtrace** 看看。

```rust
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/std/src/panicking.rs:483
   1: core::panicking::panic_fmt
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:85
   2: core::panicking::panic_bounds_check
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/panicking.rs:62
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:255
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/slice/index.rs:15
   5: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/alloc/src/vec.rs:1982
   6: panic::main
             at ./src/main.rs:4
   7: core::ops::function::FnOnce::call_once
             at /rustc/7eac88abb2e57e752f3302f02be5f3ce3d7adfb4/library/core/src/ops/function.rs:227
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

这里有大量的输出！**实际看到的输出可能因不同的操作系统和 Rust 版本而有所不同**。为了获取带有这些信息的 `backtrace`，必须启用 `debug` 标识。当不使用 `--release` 参数运行 `cargo build` 或 `cargo run` 时 **debug** 标识会默认启用，就像这里一样。

示例的输出中，backtrace 的 12 行指向了我们项目中造成问题的行：`src/main.rs` 的第 `4` 行。如果不希望程序 `panic`，第一个提到我们编写的代码行的位置是你应该开始调查的，以便查明是什么值如何在这个地方引起了 `panic`。
在示例中，故意编写会 `panic` 的代码来演示如何使用 `backtrace`，修复这个 `panic` 的方法就是不要尝试在一个只包含三个项的 `vector` 中请求索引是 100 的元素。
当将来你的代码出现了 `panic`，你需要搞清楚在这特定的场景下代码中执行了什么操作和什么值导致了 `panic`，以及应当如何处理才能避免这个问题。

## 可恢复的错误(Result)

**大部分错误并没有严重到需要程序完全停止执行**。有时，一个函数会因为一个容易理解并做出反应的原因失败。

例如，如果因为打开一个并不存在的文件而失败，此时我们可能想要创建这个文件，而不是终止进程。

第二章 “[使用 Result 类型来处理潜在的错误](https://kaisery.github.io/trpl-zh-cn/ch02-00-guessing-game-tutorial.html#%E4%BD%BF%E7%94%A8-result-%E7%B1%BB%E5%9E%8B%E6%9D%A5%E5%A4%84%E7%90%86%E6%BD%9C%E5%9C%A8%E7%9A%84%E9%94%99%E8%AF%AF)” 部分中的那个 **Result** 枚举，它定义有如下两个成员，**Ok** 和 **Err**：

```rust
// T 和 E 是泛型类型参数
enum Result<T, E> {
    Ok(T),      // T 代表成功时返回的 Ok 成员中的数据的类型，
    Err(E),     // 而 E 代表失败时返回的 Err 成员中的错误的类型。
}
```

因为 **Result** 有这些泛型类型参数，我们可以将 **Result** 类型和标准库中为其定义的函数用于很多不同的场景，这些情况中需要返回的成功值和失败值可能会各不相同。

调用一个返回 Result 的函数，因为它可能会失败：

```rust
use std::fs::File;

fn main() {
    // 打开文件
    let f = File::open("hello.txt");
}
```

如何知道 `File::open` 返回一个 **Result** 呢？我们可以查看 [标准库 API](https://doc.rust-lang.org/std/index.html) 文档，或者可以直接问编译器！
如果给 `f` 某个我们知道 **不是** 函数返回值类型的类型注解，接着尝试编译代码，编译器会告诉我们类型不匹配。
然后错误信息会告诉我们 `f` 的类型 **应该** 是什么。

试试！知道 `File::open` 的返回值不是 u32 类型的，所以将 `let f` 语句改为如下：

```rust
let f: u32 = File::open("hello.txt");
```

```shell
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0308]: mismatched types
 --> src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |            ---   ^^^^^^^^^^^^^^^^^^^^^^^ expected `u32`, found enum `Result`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<File, std::io::Error>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `error-handling` due to previous error
```

这就告诉我们了 `File::open` 函数的返回值类型是 `Result<T, E>`。这里泛型参数 **T** 放入了成功值的类型 `std::fs::File`，它是一个文件句柄。`E` 被用在失败值上时 `E` 的类型是 `std::io::Error`。

这个返回值类型说明 `File::open` 调用可能会成功并返回一个可以进行读写的文件句柄。
这个函数也可能会失败：例如，文件可能并不存在，或者可能没有访问文件的权限。
`File::open` 需要一个方式告诉我们是成功还是失败，并同时提供给我们文件句柄或错误信息。而这些信息正是 **Result** 枚举可以提供的。

当 `File::open` 成功的情况下，变量 `f` 的值将会是一个包含文件句柄的 `Ok` 实例。在失败的情况下，`f` 的值会是一个包含更多关于出现了何种错误信息的 `Err` 实例。

需要在示例的代码中增加根据 `File::open` 返回值进行不同处理的逻辑。下面示例展示了一个使用基本工具处理 **Result** 的例子：第六章学习过的 `match` 表达式。

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // 这里告诉 Rust 当结果是 Ok 时，返回 Ok 成员中的 file 值，然后将这个文件句柄赋值给变量 f。
        // match 之后，我们可以利用这个文件句柄来进行读写。

        Err(error) => panic!("Problem opening the file: {:?}", error),
        // match 的另一个分支处理从 File::open 得到 Err 值的情况。在这种情况下，我们选择调用 panic! 宏。
    };
}

```

注意与 **Option** 枚举一样，**Result** 枚举和其成员也被导入到了 `prelude` 中，所以就不需要在 `match` 分支中的 `Ok` 和 `Err` 之前指定 `Result::`。

如果当前目录没有一个叫做 `hello.txt` 的文件，当运行这段代码时会看到如下来自 `panic!` 宏的输出：

```rust
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
    Finished dev [unoptimized + debuginfo] target(s) in 0.73s
     Running `target/debug/error-handling`
thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:8:23
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### 匹配不同的错误

示例中的代码不管 **File::open** 是因为什么原因失败都会 `panic!`。
我们真正希望的是对不同的错误原因采取不同的行为：

- 如果 **File::open** 因为文件不存在而失败，我们希望创建这个文件并返回新文件的句柄。
- 如果 **File::open** 因为任何其他原因失败，例如没有打开文件的权限，我们仍然希望像示例那样 `panic!`。

让我们看看下面的示例，其中 `match` 增加了另一个分支：

```rust
// 使用不同的方式处理不同类型的错误
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        
        // File::open 返回的 Err 成员中的值类型 io::Error，它是一个标准库中提供的结构体。
        // 这个结构体有一个返回 io::ErrorKind 值的 kind 方法可供调用。
        // io::ErrorKind 是一个标准库提供的枚举，它的成员对应 io 操作可能导致的不同错误类型。
        // 我们感兴趣的成员是 ErrorKind::NotFound，它代表尝试打开的文件并不存在。
        Err(err) => match err.kind() {
            
            // 如果 error.kind() 的返回值为 ErrorKind的 NotFound 成员。
            // 则尝试通过 File::create 创建文件。
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                // 然而因为 File::create 也可能会失败，还需要增加一个内层 match 语句。
                // 当文件不能被打开，会打印出一个不同的错误信息。
                Err(e) => panic!("创建文件失败: {:?}", e),
            },
            // 外层 match 的最后一个分支保持不变，这样对任何除了文件不存在的错误会使程序 panic。
            other_error => {
                panic!("打开文件失败: {:?}", other_error)
            }
        },
    };
}
```

> 不同于使用 **match** 和 **Result<T, E\>**
>
> 这里有好多 match！match 确实很强大，不过也非常的基础。第十三章我们会介绍闭包（closure），这可以用于很多 Result<T, E> 上定义的方法。在处理代码中的 Result<T, E> 值时这些方法可能会更加简洁。
>
> 例如，这是另一个编写与示例 9-5 逻辑相同但是使用闭包和 unwrap_or_else 方法的例子：
>
> ```rust
> use std::fs::File;
> use std::io::ErrorKind;
> 
> fn main() {
>     let f = File::open("hello.txt").unwrap_or_else(|error| {
>         if error.kind() == ErrorKind::NotFound {
>             File::create("hello.txt").unwrap_or_else(|error| {
>                 panic!("创建文件失败: {:?}", error);
>             })
>         } else {
>             panic!("打开文件失败: {:?}", error);
>         }
>     });
> }
> ```
>
> 虽然这段代码有着如上面示例中一样的行为，但并没有包含任何 `match` 表达式且更容易阅读。在阅读完第十三章后再回到这个例子，并查看标准库文档 `unwrap_or_else` 方法都做了什么操作。在处理错误时，还有很多这类方法可以消除大量嵌套的 `match` 表达式。

### unwrap 和 expect

**`match` 能够胜任它的工作，不过它可能有点冗长并且不总是能很好的表明其意图。** **Result<T, E\>**类型定义了很多辅助方法来处理各种情况。
其中之一叫做 `unwrap`，它的实现就类似于上面示例中的 `match` 语句。

- 如果 **Result** 值是成员 `Ok`，`unwrap` 会返回 `Ok` 中的值。
- 如果 **Result** 值是成员 `Err`，`unwrap` 会为我们调用 `panic!`。

这里是一个实践 `unwrap` 的例子：

```rust
use std::fs::File;

fn main() {
    // `unwrap` **调用** `panic!` 
    // 使用默认的 panic! 信息。
    let f = File::open("hello.txt").unwrap();
}
```

如果调用这段代码时不存在 `hello.txt` 文件，我们将会看到一个 `unwrap` **调用** `panic!` 时提供的错误信息：

```rust
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

还有另一个类似于 `unwrap` 的方法它还允许我们选择 `panic!` 的错误信息：`expect`。
使用 `expect` 而不是 `unwrap` 并提供一个好的错误信息可以表明你的意图并更易于追踪 `panic` 的根源。
`expect` 的语法看起来像这样：

```rust
use std::fs::File;

fn main() {
    // 返回文件句柄或调用 panic! 宏
    // expect 在调用 panic! 时使用的错误信息将是传递给 expect 的参数，
    // 而不像 unwrap 那样使用默认的 panic! 信息。
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

`expect` 与 `unwrap` 的使用方式一样：返回文件句柄或调用 `panic!` 宏。`expect` 在调用 `panic!` 时使用的错误信息将是我们传递给 `expect` 的参数，而不像 `unwrap` 那样使用默认的 `panic!` 信息。
它看起来像这样：

```rust
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

{++因为这个错误信息以指定的文本开始，**Failed to open hello.txt**，将会更容易找到代码中的错误信息来自何处。
如果在多处使用 `unwrap`，则需要花更多的时间来分析到底是哪一个 `unwrap` 造成了 `panic`，因为所有的 `unwrap` 调用都打印相同的信息。++}

### 传播错误

**当编写一个其实先会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。**
这被称为 **传播**（propagating）错误，这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑来决定应该如何处理错误。

例如，下面展示了一个从文件中读取用户名的函数。如果文件不存在或不能读取，这个函数会将这些错误返回给调用它的代码：

```rust
// 一个函数使用 match 将错误返回给代码调用者

use std::fs::File;
use std::io::{self, Read};

// 函数返回一个 Result<T, E> 类型的值，其中泛型参数 T 的具体类型是 String，而 E 的具体类型是 io::Error。
// 如果这个函数没有出任何错误成功返回，函数的调用者会收到一个包含 String 的 Ok 值 —— 函数从文件中读取到的用户名。
// 如果函数遇到任何错误，函数的调用者会收到一个 Err 值，它储存了一个包含更多这个问题相关信息的 io::Error 实例。
// 这里选择 io::Error 作为函数的返回值, 则是因为它正好是函数体中：File::open 函数和 read_to_string 方法可能会操作失败的的错误返回值。
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,           // 打开文件成功时，直接返回文件句柄
        Err(e) => return Err(e),    // 打开文件错误是，直接Return成Result的Error枚举。
    };

    let mut s = String::new();      // 创建字符串保存读取的值。

    // match结果值将返回, OK值或Err值
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),  // 将读取到的字符串作为Result的OK中的值。
        Err(e) => Err(e),   // 将Error值作为Result的Err中的值。
        // 不需要显式的调用 return，因为这是函数的最后一个表达式。
    }
}
```

调用这个函数的代码最终会得到一个包含用户名的 **Ok** 值，或者一个包含 `io::Error` 的 **Err** 值。
我们无从得知调用者会如何处理这些值。
例如，如果他们得到了一个 `Err` 值，他们可能会选择 `panic!` 并使程序崩溃、使用一个默认的用户名或者从文件之外的地方寻找用户名。
我们没有足够的信息知晓调用者具体会如何尝试，所以将所有的成功或失败信息向上传播，让他们选择合适的处理方法。

这种传播错误的模式在 Rust 是如此的常见，以至于 Rust 提供了 `?` 问号运算符来使其更易于处理。

### ?运算符

下面示例展示了一个 `read_username_from_file` 的实现，它实现了与上个示例中的代码相同的功能，不过这个实现使用了 `?` 运算符：

```rust
// 一个使用 ? 运算符向调用者返回错误的函数

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    // 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
    // 如果值是 Err，Err 中的值将作为整个函数的返回值，就好像使用了 return 关键字一样，这样错误值就被传播给了调用者。


    // ? 将会把 Ok 中的值返回给变量 f, 如果出现了错误，? 运算符会提早返回整个函数并将一些 Err 值传播给调用者
    let mut f = File::open("hello.txt")?;   
    
    let mut s = String::new();            
    
    // ？运用同上
    f.read_to_string(&mut s)?;  
    Ok(s)     // 返回Result枚举的Ok值。
}
```

{==

上个示例中的 `match` 表达式与问号运算符所做的有**一点不同**：`?` 运算符所使用的错误值被传递给了 [**from**](https://www.rustwiki.org.cn/zh-CN/std/convert/trait.From.html#tymethod.from) 函数，它定义于标准库的 [**From trait**](https://www.rustwiki.org.cn/zh-CN/std/convert/trait.From.html#) 中，其用来将错误从一种类型转换为另一种类型。
当 `?` 运算符调用 `from` 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。
这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。
只要每一个错误类型都实现了 `from` 函数来定义如何将自身转换为返回的错误类型，`?` 运算符会自动处理这些转换。

==}

{++`?` 运算符消除了大量样板代码并使得函数的实现更简单。++} 我们甚至可以在 `?` 之后直接使用链式方法调用来进一步缩短代码:

```rust
// 问号运算符之后的链式方法调用

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

### 哪里使用 ? 运算符

{++`?` 运算符只能被用于返回值与 `?` 作用的值相兼容的函数。++}
因为 `?` 运算符被定义为从函数中提早返回一个值，这与示例中的 `match` 表达式有着完全相同的工作方式。
示例中 `match` 作用于一个 **Result** 值，提早返回的分支返回了一个 **Err(e)** 值。
函数的返回值必须是 **Result** 类型才能与这个 **return** 相兼容。

看看在返回值不兼容的 **main** 函数中使用 `?` 运算符会得到什么错误：

```rust
// 尝试在返回 () 的函数中使用 ? 的代码不能编译

use std::fs::File;

// 返回类型是 () 而不是 Result
fn main() {
    let f = File::open("hello.txt")?;
}
```

当编译这些代码，会得到如下错误信息：

```rust
$ cargo run
   Compiling error-handling v0.1.0 (file:///projects/error-handling)
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:4:36
  |
3 | / fn main() {
4 | |     let f = File::open("hello.txt")?;
  | |                                    ^ cannot use the `?` operator in a function that returns `()`
5 | | }
  | |_- this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `error-handling` due to previous error
```

这个错误指出只能在返回 **Result** 或者其它实现了 **FromResidual** 的类型的函数中使用 `?` 运算符。
为了修复这个错误，有两个选择。

一个是，如果没有限制的话将函数的返回值改为 **Result<T, E\>**。

另一个是使用 `match` 或 **Result<T, E\>** 的方法中合适的一个来处理 **Result<T, E\>**。

错误信息中也提到 `?` 也可用于 **Option<T\>** 值。
如同对 **Result** 使用 `?` 一样，只能在返回 **Option** 的函数中对 **Option** 使用 `?`。
在 **Option<T\>** 上调用 `?` 运算符的行为与 **Result<T, E\>** 类似：

如果值是 **None**，此时 **None** 会从函数中提前返回。

如果值是 **Some**，**Some** 中的值作为表达式的返回值同时函数继续。

下面示例中有一个从给定文本中返回第一行最后一个字符的函数的例子：

```rust
// 在 Option<T> 值上使用 ? 运算符
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

这个函数返回 **Option<char\>** 因为它可能会在这个位置找到一个字符，也可能没有字符。
这段代码获取 `text` 字符串 `slice` 作为参数并调用其 `lines` 方法，这会返回一个字符串中每一行的迭代器。
因为函数希望检查第一行，所以调用了迭代器 `next` 来获取迭代器中第一个值。

如果 `text` 是空字符串，`next` 调用会返回 **None**，此时我们可以使用 `?` 来停止并从 `last_char_of_first_line` 返回 **None**。

如果 `text` 不是空字符串，`next` 会返回一个包含 `text` 中第一行的字符串 `slice` 的 **Some** 值。

`?` 会提取这个字符串 `slice`，然后可以在字符串 `slice` 上调用 `chars` 来获取字符的迭代器。
我们感兴趣的是第一行的最后一个字符，所以可以调用 `last` 来返回迭代器的最后一项。
这是一个 **Option**，因为有可能第一行是一个空字符串，例如 `text` 以一个空行开头而后面的行有文本，像是 "\nhi"。
不过，如果第一行有最后一个字符，它会返回在一个 `Some` 成员中。
`?` 运算符作用于其中给了我们一个简洁的表达这种逻辑的方式。

如果我们不能在 **Option** 上使用 `?` 运算符，则不得不使用更多的方法调用或者 `match` 表达式来实现这些逻辑。

> **注意**
>
> 你可以在返回 **Result** 的函数中对 **Result** 使用 `?` 运算符，可以在返回 **Option** 的函数中对 **Option** 使用 `?` 运算符，但是不可以混合搭配。
>
> `?` 运算符不会自动将 `Result` 转化为 `Option`，反之亦然；
> 在这些情况下，可以使用类似 **Result** 的 `ok` 方法或者 **Option** 的 `ok_or` 方法来显式转换。

目前为止，所使用的所有 `main` 函数都返回 `()`。`main` 函数是特殊的因为它是可执行程序的入口点和退出点，为了使程序能正常工作，其可以返回的类型是有限制的。

幸运的是 `main` 函数也可以返回 **Result<(), E\>**， 下面示例修改了 `main` 的返回值为 **Result<(), Box<dyn Error\>\>** 并在结尾增加了一个 `Ok(())` 作为返回值。
这段代码可以编译：

```rust
// 修改 main 返回 Result<(), E> 允许对 Result 值使用 ? 运算符

use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

**Box<dyn Error\>** 类型是一个 **trait** 对象（**trait** object）第十七章 “**为使用不同类型的值而设计的 trait 对象**” 部分会做介绍。
目前可以将 **Box<dyn Error\>** 理解为 “**任何类型的错误**”。
在返回 **Box<dyn Error\>** 错误类型 `main` 函数中对 **Result** 使用 `?` 是允许的，因为它允许任何 `Err` 值提前返回。

当 `main` 函数返回 **Result<(), E\>**，

- 如果 `main` 返回 **Ok(())** 可执行程序会以 `0` 值退出，
- 而如果 **main** 返回 `Err` 值则会以非零值退出；

成功退出的程序会返回整数 `0`，运行错误的程序会返回非 `0` 的整数。**Rust 也会从二进制程序中返回与这个惯例相兼容的整数。**

`main` 函数也可以返回任何实现了 **std::process::Termination trait** 的类型。
截至编写时，**Termination trait** 是一个不稳定功能（unstable feature），只能用于 **Nightly Rust** 中，
所以你不能在 **稳定版** Rust（Stable Rust）中用自己的类型去实现，不过有朝一日应该可以！

## 错误处理抉择

那么，该如何决定何时应该 `panic!` 以及何时应该返回 **Result** 呢？

- 如果代码 `panic`，就没有恢复的可能。你可以选择对任何错误场景都调用 `panic!`，不管是否有可能恢复，不过这样就是你代替调用者决定了这是不可恢复的。
- 选择返回 **Result** 值的话，就将选择权交给了调用者，而不是代替他们做出决定。调用者可能会选择以符合他们场景的方式尝试恢复，或者也可能干脆就认为 Err 是不可恢复的，所以他们也可能会调用 `panic!` 并将可恢复的错误变成了不可恢复的错误。{++因此返回 **Result** 是定义可能会失败的函数的一个好的默认选择。++}

在一些类似**示例**、**原型代码**（prototype code）和**测试**中， `panic` 比返回 **Result** 更为合适，不过他们并不常见。

### 示例、代码原型和测试都非常适合 panic

当你编写一个**示例**来展示一些概念时，**在拥有健壮的错误处理代码的同时也会使得例子不那么明确。** 例如，调用一个类似 `unwrap` 这样可能 `panic!` 的方法可以被理解为一个你实际希望程序处理错误方式的占位符，它根据其余代码运行方式可能会各不相同。

类似地，在准备好决定如何处理错误之前，`unwrap`和`expect`方法在原型设计时非常方便。{++当我们准备好让程序更加健壮时，它们会在代码中留下清晰的标记。++}

如果方法调用在测试中失败了，我们希望这个测试都失败，即便这个方法并不是需要测试的功能。因为 `panic!` 会将测试标记为失败，此时调用 `unwrap` 或 `expect` 是恰当的。

### 当我们知道更多的情况

当你有一些其他的逻辑来确保 **Result** 会是 **Ok** 值时，调用 `unwrap` 也是合适的，虽然编译器无法理解这种逻辑。
你仍然需要处理一个 **Result** 值：即使在你的特定情况下逻辑上是不可能的，你所调用的任何操作仍然有可能失败。
如果通过人工检查代码来确保永远也不会出现 `Err` 值，那么调用 `unwrap` 也是完全可以接受的，这里是一个例子：

```rust
use std::net::IpAddr;

// 永远不可能失败，但要处理Result，可以使用unwrap;
let home: IpAddr = "127.0.0.1".parse().unwrap();
```

通过解析一个硬编码的字符来创建一个 **IpAddr** 实例。可以看出 `127.0.0.1` 是一个有效的 IP 地址，所以这里使用 `unwrap` 是可以接受的。
然而，拥有一个硬编码的有效的字符串也不能改变 **parse** 方法的返回值类型：它仍然是一个 **Result** 值，而编译器仍然会要求我们处理这个 **Result**，好像还是有可能出现 **Err** 成员那样。
这是因为编译器还没有智能到可以识别出这个字符串总是一个有效的 **IP** 地址。如果 **IP** 地址字符串来源于用户而不是硬编码进程序中的话，那么就 **确实** 有失败的可能性，这时就绝对需要我们以一种更健壮的方式处理 **Result** 了。

### 错误处理指导原则

在当有可能会导致**有害状态**的情况下建议使用 `panic!` —— 在这里，{++**有害状态**是指当一些假设、保证、协议或不可变性被打破的状态++}，例如*无效的值*、*自相矛盾的值*或者*被传递了不存在的值* —— 外加如下几种情况：

- 有害状态是**非预期的行为**，与偶尔会发生的行为相对，比如用户输入了错误格式的数据。
- 在此之后**代码的运行依赖于不处于这种有害状态**，而不是在每一步都检查是否有问题。
- **没有可行的手段来将有害状态信息编码进所使用的类型中的情况**。我们会在第十七章 “[将状态和行为编码为类型](https://kaisery.github.io/trpl-zh-cn/ch17-03-oo-design-patterns.html#%E5%B0%86%E7%8A%B6%E6%80%81%E5%92%8C%E8%A1%8C%E4%B8%BA%E7%BC%96%E7%A0%81%E4%B8%BA%E7%B1%BB%E5%9E%8B)” 部分通过一个例子来说明我们的意思。

如果别人调用你的代码并传递了一个没有意义的值，最好的情况也许就是 `panic!` 并警告使用你的库的人他的代码中有 `bug` 以便他能在开发时就修复它。
类似的，如果你正在调用不受你控制的外部代码，并且它返回了一个你无法修复的无效状态，那么 `panic!` 往往是合适的。

然而当错误预期会出现时，返回 **Result** 仍要比调用 `panic!` 更为合适。
这样的例子包括解析器接收到格式错误的数据，或者 **HTTP** 请求返回了一个表明触发了限流的状态。
在这些例子中，应该通过返回 **Result** 来表明失败预期是可能的，这样将有害状态向上传播，调用者就可以决定该如何处理这个问题。
使用 **panic!** 来处理这些情况就不是最好的选择。

当代码对值进行操作时，应该首先验证值是有效的，并在其无效时 `panic!`。
这主要是出于安全的原因：尝试操作无效数据会暴露代码漏洞，这就是标准库在尝试越界访问数组时会 `panic!` 的主要原因：尝试访问不属于当前数据结构的内存是一个常见的安全隐患。

函数通常都遵循 **契约**（contracts）：{++他们的行为只有在输入满足特定条件时才能得到保证++}。当违反契约时 `panic` 是有道理的，因为这通常代表调用方的 `bug`，而且这也不是那种你希望所调用的代码必须处理的错误。
事实上所调用的代码也没有合理的方式来恢复，而是需要调用方的 **程序员** 修复其代码。函数的契约，尤其是当违反它会造成 `panic` 的契约，应该在函数的 **API** 文档中得到解释。

虽然在所有函数中都拥有许多错误检查是冗长而烦人的。
幸运的是，可以利用 **Rust** 的类型系统（以及编译器的类型检查）为你进行很多检查。

- 如果函数有一个特定类型的参数，可以在知晓编译器已经确保其拥有一个有效值的前提下进行你的代码逻辑。

例如，如果你使用了一个并不是 **Option** 的类型，则程序期望它是 **有值** 的并且不是 **空值**,
你的代码无需处理 **Some** 和 **None** 这两种情况，它只会有一种情况就是绝对会有一个值。
{++尝试向函数传递空值的代码甚至根本不能编译，所以你的函数在运行时没有必要判空。++}
另外一个例子是使用像 `u32` 这样的无符号整型，也会确保它永远不为负。

### 创建自定义类型进行有效性验证

让我们使用 **Rust** 类型系统的思想来进一步确保值的有效性，并尝试创建一个自定义类型以进行验证。
回忆一下第二章的猜猜看游戏，我们的代码要求用户猜测一个 `1` 到 `100` 之间的数字，在将其与秘密数字做比较之前我们从未验证用户的猜测是位于这两个数字之间的，我们只验证它是否为正。
在这种情况下，其影响并不是很严重：“Too high” 或 “Too low” 的输出仍然是正确的。
但是这是一个很好的引导用户得出有效猜测的辅助，例如当用户猜测一个超出范围的数字或者输入字母时采取不同的行为。

一种实现方式是将猜测解析成 `i32` 而不仅仅是 `u32`，来默许输入负数，接着检查数字是否在范围内：

```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    // if 表达式检查了值是否超出范围，告诉用户出了什么问题，并调用 continue 开始下一次循环，请求另一个猜测。
    // if 表达式之后，就可以在知道 guess 在 1 到 100 之间的情况下与秘密数字作比较了。
    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
}
```

然而，这并不是一个理想的解决方案：如果让程序仅仅处理 `1` 到 `100` 之间的值是一个绝对需要满足的要求，而且程序中的很多函数都有这样的要求，{++在每个函数中都有这样的检查将是非常冗余的++}（并可能潜在的影响性能）。

**相反我们可以创建一个新类型来将验证放入创建其实例的函数中，而不是到处重复这些检查。这样就可以安全的在函数签名中使用新类型并相信他们接收到的值。**
下面示例中展示了一个定义 **Guess** 类型的方法，只有在 `new` 函数接收到 `1` 到 `100` 之间的值时才会创建 **Guess** 的实例：

```rust
// 一个 Guess 类型，它只在值位于 1 和 100 之间时才继续

// 首先，定义了一个包含 i32 类型字段 value 的结构体 Guess。这里是储存猜测值的地方。
pub struct Guess {
    value: i32,
}

impl Guess {
    // 接着在 Guess 上实现了一个叫做 new 的关联函数来创建 Guess 的实例。
    // new 定义为接收一个 i32 类型的参数 value 并返回一个 Guess。
    pub fn new(value: i32) -> Guess {

        // new 函数中代码的测试确保了其值是在 1 到 100 之间的。
        if value < 1 || value > 100 {

            // 如果 value 没有通过测试则调用 panic!，这会警告调用这个函数的程序员有一个需要修改的 bug，
            // 因为创建一个 value 超出范围的 Guess 将会违反 Guess::new 所遵循的契约。

            // Guess::new 会出现 panic 的条件应该在其公有 API 文档中被提及；
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        // 如果 value 通过了测试，我们新建一个 Guess，其字段 value 将被设置为参数 value 的值，接着返回这个 Guess。
        Guess { value }
    }

    // 接着, 实现了一个借用了 self 的方法 value，它没有任何其他参数并返回一个 i32。
    // 这类方法有时被称为 getter，因为它的目的就是返回对应字段的数据。
    // 这样的公有方法是必要的，因为 Guess 结构体的 value 字段是私有的。
    pub fn value(&self) -> i32 {
        self.value
    }
}
```

{++私有的字段 **value** 是很重要的，这样使用 **Guess** 结构体的代码将不允许直接设置 **value** 的值：调用者 **必须** 使用 `Guess::new` 方法来创建一个 **Guess** 的实例，这就确保了不会存在一个 **value** 没有通过 **Guess::new** 函数的条件检查的 Guess。++}

于是，一个接收（或返回） 1 到 100 之间数字的函数就可以声明为接收（或返回） **Guess**的实例，而不是 `i32`，同时其函数体中也无需进行任何额外的检查。

## 总结

Rust 的错误处理功能被设计为帮助你编写更加健壮的代码。`panic!` 宏代表一个程序无法处理的状态，并停止执行而不是使用无效或不正确的值继续处理。
Rust 类型系统的 **Result** 枚举代表操作可能会在一种可以恢复的情况下失败。
可以使用 **Result** 来告诉代码调用者他需要处理潜在的成功或失败。
在适当的场景使用 `panic!` 和 **Result** 将会使你的代码在面对不可避免的错误时显得更加可靠。
