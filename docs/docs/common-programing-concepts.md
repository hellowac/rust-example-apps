# 常见编程概念

## 变量和可变性

变量默认是不可改变的（immutable）。

当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值。

文件名: src/main.rs

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;      // 错误, 不能对不可变变量 x 二次赋值（cannot assign twice to immutable variable `x` ）
    println!("The value of x is: {x}");
}
```

不过可变性也是非常有用的，可以用来更方便地编写代码。尽管变量默认是不可变的，你仍然可以在变量名前添加 `mut` 来使其可变:

```rust
fn main() {
    let mut x = 5;  // 添加mut，使其可变。（进行多次赋值）
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

### 常量

类似于不可变变量，常量(constants) 是绑定到一个名称的不允许改变的值，不过常量与变量还是有一些区别。

1. 不允许对常量使用 mut。常量不光默认不能变，它总是不能变。
2. 常量使用 `const` 关键字而不是 `let`，并且 必须 注明值的类型。
3. 常量可以在任何作用域中声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
4. 常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值。

例如：

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

> Rust 对常量的命名约定是在单词之间使用全大写加下划线

有关声明常量时可以使用哪些操作的详细信息，请参阅 [Rust Reference 的常量求值部分](https://doc.rust-lang.org/reference/const_eval.html)。

### 隐藏

可以定义一个与之前变量同名的新变量。称之为第一个变量被第二个 **隐藏**（Shadowing）了,
这意味着在使用变量的名称时，编译器将看到第二个变量。
实际上，第二个变量“遮蔽”了第一个变量，此时任何使用该变量名的行为中都会视为是在使用第二个变量，直到第二个变量自己也被隐藏或第二个变量的作用域结束。
可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏，如下所示：

```rust
fn main() {
    let x = 5;  // 首先将 x 绑定到值 5 上

    let x = x + 1;  // 接着通过 let x = 创建了一个新变量 x，获取初始值并加 1，这样 x 的值就变成 6 了

    {       // 然后，在使用花括号创建的内部作用域内
        let x = x * 2;  // 第三个 let 语句也隐藏了 x 并创建了一个新的变量，将之前的值乘以 2，x 得到的值是 12
        println!("The value of x in the inner scope is: {x}");
    }       
    // 当该作用域结束时，内部 shadowing 的作用域也结束了，x 又返回到 6

    println!("The value of x is: {x}");
}
```

隐藏与将变量标记为 `mut` 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 `let` 关键字，就会导致编译时错误。
通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。

`mut` 与隐藏的另一个区别是，当再次使用 `let` 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。
例如，假设程序请求用户输入空格字符来说明希望在文本之间显示多少个空格，接下来我们想将输入存储成数字（多少个空格）：

```rust
let spaces = "   ";             // 第一个 spaces 变量是字符串类型
let spaces = spaces.len();      // 第二个 spaces 变量是数字类型
```

隐藏使我们不必使用不同的名字，如 spaces_str 和 spaces_num；

相反，我们可以复用 spaces 这个更简单的名字。然而，如果尝试使用 `mut`，将会得到一个编译时错误，如下所示：

```rust
let mut spaces = "   ";  // string类型
spaces = spaces.len();   // int 类型
```

错误提示:

```rust
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`  

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

> 不能改变变量的类型

## 数据类型

在 Rust 中，每一个值都属于某一个 数据类型（data type），这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。
我们将看到两类数据类型子集：**标量**（scalar）和 **复合**（compound）。

> **Rust 是 静态类型（statically typed）语言，也就是说在编译时就必须知道所有变量的类型。**

使用 `parse` 将 `String` 转换为数字时，必须增加类型注解，像这样：

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

如果不像上面这样添加类型注解 : u32，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：

```shell
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error
```

### 标量类型

**标量**（scalar）类型代表一个单独的值。Rust 有四种基本的标量类型：**整型**、**浮点型**、**布尔类型**和**字符类型**。

#### 整型

**整数** 是一个没有小数部分的数字。

| 长度    | 有符号 | 无符号 |
| ------- | ------ | ------ |
| 8-bit   | i8     | u8     |
| 16-bit  | i16    | u16    |
| 32-bit  | i32    | u32    |
| 64-bit  | i64    | u64    |
| 128-bit | i128   | u128   |
| arch    | isize  | usize  |

`isize` 和 `usize` 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

可以使用下面表格中的任何一种形式编写数字字面值。

请注意可以是多种数字类型的数字字面值允许使用类型后缀，例如 **57u8** 来指定类型，同时也允许使用 `_` 做为分隔符以方便读数，例如`1_000`，它的值与你指定的 `1000` 相同。

| 数字字面值                  | 例子        |
| --------------------------- | ----------- |
| Decimal (十进制)            | 98_222      |
| Hex (十六进制)              | 0xff        |
| Octal (八进制)              | 0o77        |
| Binary (二进制)             | 0b1111_0000 |
| Byte (单字节字符)(仅限于u8) | b'A'        |

Rust 的默认类型通常是个不错的起点，数字类型默认是 `i32`。`isize` 或 `usize` 主要作为某些集合的索引。

其他 参考: [整形溢出](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E6%95%B4%E5%9E%8B%E6%BA%A2%E5%87%BA)

#### 浮点型

Rust 也有两个原生的 浮点数（floating-point numbers）类型，它们是带小数点的数字。Rust 的浮点数类型是 f32 和 f64，分别占 32 位和 64 位。默认类型是 f64，因为在现代 CPU 中，它与 f32 速度几乎一样，不过精度更高。所有的浮点型都是有符号的。

这是一个展示浮点数的实例：

文件名: src/main.rs

```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

> 浮点数采用 IEEE-754 标准表示。f32 是单精度浮点数，f64 是双精度浮点数。

#### 数值运算

Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余。整数除法会向下舍入到最接近的整数。下面的代码展示了如何在 let 语句中使用它们：

文件名: src/main.rs

```rust
fn main() {
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // 结果为0

    // 取余
    let remainder = 43 % 5;
}
```

这些语句中的每个表达式使用了一个数学运算符并计算出了一个值，然后绑定给一个变量。[附录 B](https://kaisery.github.io/trpl-zh-cn/appendix-02-operators.html) 包含 Rust 提供的所有运算符的列表。

#### 布尔型

正如其他大部分编程语言一样，Rust 中的布尔类型有两个可能的值：true 和 false。Rust 中的布尔类型使用 bool 表示。例如：

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

#### 字符类型

Rust的 char 类型是语言中最原生的字母类型。下面是一些声明 char 值的例子：

文件名: src/main.rs

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```

注意，这里用单引号声明 `char` 字面量，而与之相反的是，使用**双引号**声明字符串字面量。Rust 的 `char` 类型的大小为四个字节(four bytes)，并代表了一个 `Unicode` 标量值（Unicode Scalar Value），这意味着它可以比 `ASCII` 表示更多内容。
在 Rust 中，带变音符号的字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的 char 值。
`Unicode` 标量值包含从 **U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF 在内的值**。
不过，“字符” 并不是一个 Unicode 中的概念，所以人直觉上的 “字符” 可能与 Rust 中的 char 并不符合。
第八章的 “[使用字符串存储 UTF-8 编码的文本](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html#%E4%BD%BF%E7%94%A8%E5%AD%97%E7%AC%A6%E4%B8%B2%E5%AD%98%E5%82%A8-utf-8-%E7%BC%96%E7%A0%81%E7%9A%84%E6%96%87%E6%9C%AC)” 中将详细讨论这个主题。

### 复合类型

**复合类型**（Compound types）可以将多个值组合成一个类型。Rust 有两个原生的复合类型：**元组**（tuple）和**数组**（array）。

#### 元组类型

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。

**元组长度固定：一旦声明，其长度不会增大或缩小。**

使用包含在圆括号中的逗号分隔的值列表来创建一个元组。
元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。这个例子中使用了可选的类型注解：

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。
为了从元组中获取单个值，可以使用**模式匹配**（pattern matching）来**解构**（destructure）元组值，像这样：

```rust
fn main() {
    let tup = (500, 6.4, 1);   
        // 程序首先创建了一个元组并绑定到 tup 变量上

    let (x, y, z) = tup;   
        // 接着使用了 let 和一个模式将 tup 分成了三个不同的变量，x、y 和 z。
        // 这叫做 解构（destructuring）因为它将一个元组拆成了三个部分。

    println!("The value of y is: {y}");
        // 最后，程序打印出了 y 的值，也就是 6.4。
}
```

也可以使用点号（.）后跟值的索引来直接访问它们。例如：

文件名: src/main.rs

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

不带任何值的元组有个特殊的名称，叫做 **单元**（unit） 元组。这种值以及对应的类型都写作 `()`，**表示空值或空的返回类型**。如果表达式不返回任何其他值，则会隐式返回单元值。

#### 数组类型

另一个包含多个值的方式是 **数组**（array）。与元组不同，**数组中的每个元素的类型必须相同**。Rust 中的数组与一些其他语言中的数组不同，**Rust中的数组长度是固定的**。

将数组的值写成在方括号内，用逗号分隔：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

当你想要在**栈**（stack）而不是在**堆**（heap）上为数据分配空间，或者是想要确保总是有固定数量的元素时，数组非常有用。
但是数组并不如 `vector` 类型灵活。`vector` 类型是标准库提供的一个 允许 增长和缩小长度的类似数组的集合类型。
当不确定是应该使用数组还是 `vector` 的时候，那么很可能应该使用 `vector`。[第八章](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)会详细讨论 `vector`。

然而，当你确定元素个数不会改变时，数组会更有用。例如，当你在一个程序中使用月份名字时，你更应趋向于使用数组而不是 vector，因为你确定只会有12个元素。

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

可以像这样编写数组的类型：在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
// i32 是每个元素的类型。分号之后，数字 5 表明该数组包含五个元素。

let a = [3; 5];
// 通过在方括号中指定初始值加分号再加元素个数的方式来创建一个每个元素都为相同值的数组：
// 变量名为 a 的数组将包含 5 个元素，这些元素的值最初都将被设置为 3。
// 这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。
```

##### 访问数组元素

数组是可以在栈(stack)上分配的已知固定大小的单个内存块。可以使用索引来访问数组的元素，像这样：

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

##### 数组越界访问

让我们看看如果我们访问数组结尾之后的元素会发生什么呢？比如你执行以下代码，它使用类似于第 2 章中的猜数字游戏的代码从用户那里获取数组索引：

```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("请输入数组索引");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("读取数据失败");

    let index: usize = index
        .trim()
        .parse()
        .expect("请输入数字");

    let element = a[index];

    println!("数组中索引为 {index} 的值时: {element}");
}
```

此代码编译成功。如果您使用 `cargo run` 运行此代码并输入 0、1、2、3 或 4，程序将在数组中的索引处打印出相应的值。如果你输入一个超过数组末端的数字，如 10，你会看到这样的输出：

```shell
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

程序在索引操作中使用一个无效的值时导致 运行时 错误。
程序带着错误信息退出，并且没有执行最后的 println! 语句。
当尝试用索引访问一个元素时，Rust 会检查指定的索引是否小于数组的长度。
如果索引超出了数组长度，Rust 会 panic，这是 Rust 术语，**它用于程序因为错误而退出的情况**。
这种检查必须在运行时进行，特别是在这种情况下，因为编译器不可能知道用户在以后运行代码时将输入什么值。

这是第一个在实战中遇到的 Rust 安全原则的例子。
在很多底层语言中，并没有进行这类检查，这样当提供了一个不正确的索引时，就会访问无效的内存。
通过立即退出而不是允许内存访问并继续执行，Rust 让你避开此类错误。
[第九章](https://kaisery.github.io/trpl-zh-cn/ch09-00-error-handling.html)会更详细地讨论 Rust 的错误处理机制，以及如何编写可读性强而又安全的代码，使程序既不会 panic 也不会导致非法内存访问。

### 函数

函数在 Rust 代码中非常普遍。你已经见过语言中最重要的函数之一：`main` 函数，它是很多程序的入口点。你也见过 `fn` 关键字，它用来声明新函数。

Rust 代码中的函数和变量名使用 `snake case` 规范风格。在 `snake case` 中，**所有字母都是小写并使用下划线分隔单词**。这是一个包含函数定义示例的程序：

文件名: src/main.rs

```rust
fn main() {
    println!("Hello, world!");

    another_function();
}

// 通过输入 fn 后面跟着函数名和一对圆括号来定义函数。大括号告诉编译器哪里是函数体的开始和结尾。
fn another_function() {
    println!("Another function.");
}

// main 函数中的代码会按顺序执行。首先，打印 “Hello, world!” 信息，然后调用 another_function 函数并打印它的信息。
```

可以使用函数名后跟圆括号来调用我们定义过的任意函数。
因为程序中已定义 another_function 函数，所以可以在 main 函数中调用它。注意，
{++源码中 another_function 定义在 main 函数 之后；也可以定义在之前++}。
{==Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行。==}

#### 参数

我们可以定义为拥有 **参数**（parameters）的函数，参数是特殊变量，是函数签名的一部分。当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。
技术上讲，这些具体值被称为参数（arguments），但是在日常交流中，人们倾向于不区分使用 parameter 和 argument 来表示函数定义中的变量或调用函数时传入的具体值。

在这版 **another_function** 中，我们增加了一个参数：

```rust
fn main() {
    another_function(5);
    // 我们将 5 传给 another_function 时，
    // println! 宏会把 5 放在格式字符串中包含 x 的那对花括号的位置。
}

// another_function 的声明中有一个命名为 x 的参数。
// x 的类型被指定为 i32。
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
```

{==在函数签名中，**必须** 声明每个参数的类型。==}
这是 Rust 设计中一个经过慎重考虑的决定：{==要求在函数定义中提供类型注解==}，意味着编译器再也不需要你在代码的其他地方注明类型来指出你的意图。
而且，在知道函数需要什么类型后，编译器就能够给出更有用的错误消息。

当定义多个参数时，使用逗号分隔，像这样：

```rust
fn main() {
    print_labeled_measurement(5, 'h');
}

// 它有两个参数。第一个参数名为 value， 类型是 i32。
// 第二个参数是 unit_label ，类型是 char。
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
```

#### 语句和表达式

{==函数体由一系列的**语句**和一个可选的**结尾表达式**构成。==}
目前为止，我们提到的函数还不包含结尾表达式，不过你已经见过作为语句一部分的表达式。
因为 Rust 是一门基于**表达式**（expression-based）的语言，这是一个需要理解的（不同于其他语言）重要区别。
其他语言并没有这样的区别，所以让我们看看语句与表达式有什么区别以及这些区别是如何影响函数体的。

{++**语句**（Statements）是执行一些操作但不返回值的指令++}。
{++**表达式**（Expressions）计算并产生一个值。++} 让我们看一些例子。

实际上，我们已经使用过语句和表达式。使用 let 关键字创建变量并绑定一个值是一个语句。在下面示例 中，`let y = 6;` 是一个语句。

```rust
fn main() {
    let y = 6;
}
// 函数定义也是语句，上面整个例子本身就是一个语句。
```

语句不返回值。因此，不能把 let 语句赋值给另一个变量，比如下面的例子尝试做的，会产生一个错误：

```rust
fn main() {
    let x = (let y = 6);
}
```

当运行这个程序时，会得到如下错误：

```shell
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  | 

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 2 previous errors; 1 warning emitted
```

let y = 6 语句并不返回值，所以没有可以绑定到 x 上的值。
这与其他语言不同，例如 C 和 Ruby，它们的赋值语句会返回所赋的值。
在这些语言中，可以这么写 x = y = 6，这样 x 和 y 的值都是 6；Rust 中不能这样写。

表达式会计算出一个值，并且你将编写的大部分 Rust 代码是由表达式组成的。
考虑一个数学运算，比如 5 + 6，这是一个表达式并计算出值 11。
表达式可以是语句的一部分：在上面中，语句 `let y = 6;` 中的 6 是一个表达式，它计算出的值是 6。
**函数调用是一个表达式**。**宏调用是一个表达式**。**用大括号创建的一个新的块作用域也是一个表达式**，例如：

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };
        // 是一个代码块，它的值是 4
        // 这个值作为 let 语句的一部分被绑定到 y 上。
        // 注意 x+1 这一行在结尾没有分号，与你见过的大部分代码行不同。
        // 表达式的结尾没有分号。
        // 如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
        // 在接下来探索具有返回值的函数和表达式时要谨记这一点。

    println!("The value of y is: {y}");
}
```

#### 具有返回值的函数

函数可以向调用它的代码返回值。我们{++并不对返回值命名，但要在箭头（->）后声明它的类型++}。
在 Rust 中，**函数的返回值等同于函数体最后一个表达式的值**。
使用 `return` 关键字和指定值，可从函数中提前返回；
但大部分函数隐式的返回最后的表达式。
这是一个有返回值的函数的例子：

```rust
// 在 five 函数中没有函数调用、宏、甚至没有 let 语句 —— 只有数字 5。
// 这在 Rust 中是一个完全有效的函数。
// 注意，也指定了函数返回值的类型，就是 -> i32。
fn five() -> i32 {
    5
    // five 函数没有参数并定义了返回值类型，不过函数体只有单单一个 5 也没有分号，因为这是一个表达式，我们想要返回它的值。
}

fn main() {
    // 这一行表明我们使用函数的返回值初始化一个变量。
    // 因为 five 函数返回 5，这一行与代码 let x = 5; 相同
    let x = five();

    println!("The value of x is: {x}");
}
```

让我们看看另一个例子：

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
    // 如果在包含 x + 1 的行尾加上一个分号，把它从表达式变成语句，我们将看到一个错误。
}
```
