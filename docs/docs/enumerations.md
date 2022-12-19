# 枚举

**枚举**（enumerations），也被称作 `enums`。
枚举允许你通过列举可能的 **成员**（variants） 来定义一个类型。

首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。

接下来，我们会探索一个特别有用的枚举，叫做 Option，它代表一个值要么是某个值要么什么都不是。

然后会讲到在 match 表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。

最后会介绍 if let，另一个简洁方便处理代码中枚举的结构。

## 定义

**结构体给予你将字段和数据聚合在一起的方法**，像 `Rectangle` 结构体有 `width` 和 `height` 两个字段。
**而枚举给予你将一个值成为一个集合之一的方法**。比如，我们想让 `Rectangle` 是一些形状的集合，包含 `Circle` 和 `Triangle` 。
为了做到这个，Rust提供了枚举类型。

假设我们要处理 IP 地址。目前被广泛使用的两个主要 IP 标准：`IPv4`（version four）和 `IPv6`（version six）。所以可以 `枚举` 出所有可能的值，这也正是此枚举名字的由来。

任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是。
IP 地址的这个特性使得枚举数据结构非常适合这个场景，因为枚举值只可能是其中一个成员。
IPv4 和 IPv6 从根本上讲仍是 IP 地址，所以当代码在处理适用于任何类型的 IP 地址的场景时应该把它们当作相同的类型。

通过在代码中定义一个 `IpAddrKind` 枚举来表现这个概念并列出可能的 `IP` 地址类型，`V4` 和 `V6`。这被称为枚举的 **成员**（variants）：

```rust
enum IpAddrKind {
    V4,
    V6,
}

// 现在 IpAddrKind 就是一个可以在代码中使用的自定义数据类型了。
```

### 枚举值

可以像这样创建 IpAddrKind 两个不同成员的实例：

```rust
let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

注意枚举的成员位于其标识符的命名空间中，并使用**两个冒号**分开。

接着可以定义一个函数来获取任何 `IpAddrKind`：

```rust
fn route(ip_kind: IpAddrKind) {}
```

现在可以使用任一成员来调用这个函数：

```rust
route(IpAddrKind::V4);
route(IpAddrKind::V6);
```

使用枚举甚至还有更多优势。进一步考虑一下 IP 地址类型，目前没有一个存储实际 IP 地址 数据 的方法；只知道它是什么 **类型** 的。
考虑到已经学习过结构体了，你可能会如下示例那样处理这个问题：

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

**可以使用一种更简洁的方式来表达相同的概念**，仅仅使用枚举并将数据直接放进每一个枚举成员，而不是将枚举作为结构体的一部分。`IpAddr` 枚举的新定义表明了 `V4` 和 `V6` 成员都关联了 `String` 值：

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

这样直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。

另一个细节：{++每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数++}。也就是说，`IpAddr::V4()` 是一个获取 `String` 参数并返回 `IpAddr` 类型实例的函数调用。
作为定义枚举的结果，这些**构造函数会自动被定义**。

用枚举替代结构体还有另一个优势：{++每个成员可以处理不同类型和数量的数据++}。 如下:

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

事实证明存储和编码 IP 地址实在是太常见了[以致标准库提供了一个开箱即用的定义！](https://doc.rust-lang.org/std/net/enum.IpAddr.html)

标准库是如何定义 `IpAddr` 的：它正有着跟我们定义和使用的一样的枚举和成员，不过它将成员中的地址数据嵌入到了两个不同形式的结构体中，它们对不同的成员的定义是不同的：

```rust
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

{++这些代码展示了可以将任意类型的数据放入枚举成员中：例如**字符串**、**数字类型**或者**结构体**。甚至可以包含另一个**枚举**！++}

虽然标准库中包含一个 `IpAddr` 的定义，仍然可以创建和使用我们自己的定义而不会有冲突，因为我们并没有将标准库中的定义引入作用域。

```rust
enum Message {
    Quit,                           // Quit 没有关联任何数据。
    Move { x: i32, y: i32 },        // Move 类似结构体包含命名字段。
    Write(String),                  // Write 包含单独一个 String。
    ChangeColor(i32, i32, i32),     // ChangeColor 包含三个 i32。
}
```

定义一个有关联值的枚举的方式和定义多个不同类型的结构体的方式很相像，除了枚举不使用 `struct` 关键字以及其所有成员都被组合在一起位于 `Message` 类型下。
如下这些结构体可以包含与之前枚举成员中相同的数据：

```rust
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体
```

不过，如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像使用 `Message` 枚举那样，轻易的定义一个能够处理这些不同类型的结构体的函数，因为**枚举是单独一个类型**。

结构体和枚举还有另一个相似点：**就像可以使用 impl 来为结构体定义方法那样，也可以在枚举上定义方法**。这是一个定义于我们 `Message` 枚举上的叫做 `call` 的方法：

```rust
impl Message {
    // 方法体使用了 self 来获取调用方法的值。
    fn call(&self) {
        // 在这里定义方法体
    }
}

let m = Message::Write(String::from("hello"));
m.call();   // 变量 m，就是当 m.call() 运行时 call 方法中的 self 的值。
```

### Option 枚举

`Option` 是标准库定义的另一个枚举。`Option` 类型应用广泛因为它编码了一个非常普遍的场景，即一个值要么有值要么没值。

例如，如果请求一个包含项的列表的第一个值，会得到一个值，如果请求一个空的列表，就什么也不会得到。从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的 bug。

编程语言的设计经常要考虑包含哪些功能，但考虑排除哪些功能也很重要。Rust 并没有很多其他语言中有的空值功能。**空值**（Null ）是一个值，它代表没有值。在有空值的语言中，变量总是这两种状态之一：空值和非空值。

空值的问题在于当你尝试像一个非空值那样使用一个空值，会出现某种形式的错误。因为空和非空的属性无处不在，非常容易出现这类错误。

然而，空值尝试表达的概念仍然是有意义的：**空值是一个因为某种原因目前无效或缺失的值**。

问题不在于概念而在于具体的实现。为此，Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 `Option<T>`，而且它[定义于标准库中](https://doc.rust-lang.org/std/option/enum.Option.html)，如下:

```rust
enum Option<T> {
    None,
    Some(T),
}
```

`Option<T>` 枚举是如此有用以至于它甚至被包含在了 **prelude** 之中，你不需要将其显式引入作用域。
另外，它的成员也是如此，可以不需要 `Option::` 前缀来直接使用 `Some` 和 `None`。
即便如此 `Option<T>` 也仍是常规的枚举，`Some(T)` 和 `None`仍是 `Option<T>` 的成员。

`<T>` 语法是一个我们还未讲到的 Rust 功能。它是一个泛型类型参数，第十章会更详细的讲解泛型。

```rust
let some_number = Some(5);      // 类型是 Option<i32>
let some_char = Some('e');      // 类型是 Option<char>

let absent_number: Option<i32> = None;  // 对于 absent_number， Rust 需要我们指定 Option 整体的类型
// 因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型。
// 这里我们告诉 Rust 希望 absent_number 是 Option<i32> 类型的。
```

当有一个 `Some` 值时，我们就知道存在一个值，而这个值保存在 `Some` 中。当有个 `None` 值时，在某种意义上，它跟空值具有相同的意义：**并没有一个有效的值**。那么，`Option<T>` 为什么就比空值要好呢？

简而言之，因为 `Option<T>` 和 `T`（这里 `T` 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 `Option<T>`。例如，这段代码不能编译，因为它尝试将 `Option<i8>` 与 `i8` 相加：

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

如果运行这些代码，将得到类似这样的错误信息：

```shell
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` due to previous error
```

**错误信息意味着 Rust 不知道该如何将 `Option<i8>` 与 `i8` 相加，因为它们的类型不同。**

当在 Rust 中拥有一个像 `i8` 这样类型的值时，编译器确保它总是有一个有效的值。我们可以自信使用而无需做空值检查。
只有当使用 `Option<i8>`（或者任何用到的类型）的时候需要担心可能没有值，而**编译器会确保我们在使用值之前处理了为空的情况。**

换句话说，在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`。通常这能帮助我们捕获到空值最常见的问题之一：**假设某值不为空但实际上为空的情况。**

消除了错误地假设一个非空值的风险，会让你对代码更加有信心。
为了拥有一个可能为空的值，你必须要显式的将其放入对应类型的 `Option<T>` 中。
接着，当使用这个值时，必须明确的处理值为空的情况。只要一个值不是 `Option<T>` 类型，你就 **可以** 安全的认定它的值不为空。
这是 Rust 的一个经过深思熟虑的设计决策，来限制空值的泛滥以增加 Rust 代码的安全性。

那么当有一个 `Option<T>` 的值时，如何从 `Some` 成员中取出 `T` 的值来使用它呢？`Option<T>` 枚举拥有大量用于各种情况的方法：你可以查看[它的文档](https://doc.rust-lang.org/std/option/enum.Option.html)。
熟悉 `Option<T>` 的方法将对你的 Rust 之旅非常有用。

总的来说，为了使用 `Option<T>` 值，{++需要编写处理每个成员的代码。++}
你想要一些代码只当拥有 `Some(T)` 值时运行，允许这些代码使用其中的 `T`。
也希望一些代码在值为 `None` 时运行，这些代码并没有一个可用的 `T` 值。
`match` 表达式就是这么一个处理枚举的控制流结构：**它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据**。

## match控制流

Rust 有一个叫做 `match` 的极为强大的控制流运算符，它允许我们**将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码**。模式可由`字面值`、`变量`、`通配符`和许多`其他内容`构成；
{++match 的力量来源于模式的表现力以及编译器检查，它确保了所有可能的情况都得到处理。++}

编写一个函数来获取一个未知的硬币，并以一种类似验钞机的方式，确定它是何种硬币并返回它的美分值:

```rust
// 一个枚举和一个以枚举成员作为模式的 match 表达式

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // 列出 match 关键字后跟一个表达式
    // 看起来非常像 if 使用的表达式，不过这里有一个非常大的区别：对于 if，表达式必须返回一个布尔值，而这里它可以是任何类型
    // coin 的类型是定义的 Coin 枚举。
    match coin {
        // 接下来是 match 的分支
        // 一个分支有两个部分：一个模式和一些代码。
        // => 运算符将模式和将要运行的代码分开
        // 每一个分支之间使用逗号分隔。
        Coin::Penny => 1,       // 1美分
        Coin::Nickel => 5,      // 5美分
        Coin::Dime => 10,       // 10美分
        Coin::Quarter => 25,    // 25美分

        // 如果模式匹配了这个值，这个模式相关联的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支
        // 可以拥有任意多的分支, 这里有4个分支
        // 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值。
    }
}
```

**如果分支代码较短的话通常不使用大括号**，上面的示例每个分支都只是返回一个值。
如果想要在分支中运行多行代码，**可以使用大括号，而分支后的逗号是可选的**。
例如，如下代码在每次使用`Coin::Penny` 调用时都会打印出 `“Lucky penny!”`，同时仍然返回代码块最后的值，1：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定值的模式

匹配分支的另一个有用的功能是可以**绑定匹配的模式的部分值**。这也就是如何从枚举成员中提取值的。

作为一个例子，让我们修改枚举的一个成员来存放数据。
1999 年到 2008 年间，美国在 25 美分的硬币的一侧为 50 个州的每一个都印刷了不同的设计。
其他的硬币都没有这种区分州的设计，所以只有这些 25 美分硬币有特殊的价值。
可以将这些信息加入我们的 `enum`，通过改变 **Quarter** 成员来包含一个 `State` 值，示例完成了这些修改：

```rust
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {}
```

想象一下我们的一个朋友尝试收集所有 50 个州的 25 美分硬币。在根据硬币类型分类零钱的同时，也可以报告出每个 25 美分硬币所对应的州名称，这样如果我们的朋友没有的话，他可以将其加入收藏。

在这些代码的匹配表达式中，我们在匹配 `Coin::Quarter` 成员的分支的模式中增加了一个叫做 `state` 的变量。当匹配到 `Coin::Quarter` 时，变量 `state` 将会绑定 25 美分硬币所对应州的值。
接着在那个分支的代码中使用 `state`，如下：

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

value_in_cents(Coin::Quarter(UsState::Alaska)) // state 绑定的将会是值 UsState::Alaska
```

### 匹配 Option\<T\>

在之前的部分中使用 `Option<T>` 时，是为了从 `Some` 中取出其内部的 `T` 值；我们还可以像处理 `Coin` 枚举那样使用 `match` 处理 `Option<T>`！只不过这回比较的不再是硬币，而是 `Option<T>` 的成员，但 `match` 表达式的工作方式保持不变。

比如我们想要编写一个函数，它获取一个 `Option<i32>` ，如果其中含有一个值，将其加一。如果其中没有值，函数应该返回 `None` 值，而不尝试执行任何操作。

```rust
// 一个在 Option<i32> 上使用 match 表达式的函数

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

#### 匹配 Some(T)

更仔细地检查 `plus_one` 的第一行操作。当调用 `plus_one(five)` 时，`plus_one` 函数体中的 x 将会是值 `Some(5)`。接着将其与每个分支比较。

值 `Some(5)` 并不匹配模式 `None`，所以继续进行下一个分支。

```rust
Some(i) => Some(i + 1),
```

**Some(5)** 与 **Some(i)** 匹配吗？当然匹配！它们是相同的成员。`i` 绑定了 `Some` 中包含的值，所以 `i` 的值是 `5`。
接着匹配分支的代码被执行，所以我们将 `i` 的值加一并返回一个含有值 `6` 的新 `Some`。

接着考虑下示例中 `plus_one` 的第二个调用，这里 `x` 是 `None`。我们进入 `match` 并与第一个分支相比较。

```rust
 None => None,
```

匹配上了！这里没有值来加一，所以程序结束并返回 `=>` 右侧的值 `None`，因为第一个分支就匹配到了，{++其他的分支将不再比较++}。

将 `match` 与枚举相结合在很多场景中都是有用的。你会在 Rust 代码中看到很多这样的模式：**match 一个枚举，绑定其中的值到一个变量，接着根据其值执行代码**。
这在一开始有点复杂，不过一旦习惯了，你会希望所有语言都拥有它！

### 匹配是穷尽的

`match` 还有另一方面需要讨论：{++这些分支必须覆盖了所有的可能性++}。考虑一下 `plus_one` 函数的这个版本，它有一个 bug 并不能编译：

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
    }
}
```

我们没有处理 `None` 的情况，所以这些代码会造成一个 `bug`。幸运的是，这是一个 Rust 知道如何处理的 bug。如果尝试编译这段代码，会得到这个错误：

```shell
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0004]: non-exhaustive patterns: `None` not covered
   --> src/main.rs:3:15
    |
3   |         match x {
    |               ^ pattern `None` not covered
    |
    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
    = note: the matched value is of type `Option<i32>`

For more information about this error, try `rustc --explain E0004`.
error: could not compile `enums` due to previous error
```

Rust 知道我们没有覆盖所有可能的情况甚至知道哪些模式被忘记了！
Rust 中的匹配是 **穷尽的**（exhaustive）：{++必须穷举到最后的可能性来使代码有效++}。
特别的在这个 `Option<T>` 的例子中，Rust 防止我们忘记明确的处理 `None` 的情况，这让我们免于假设拥有一个实际上为空的值，从而使之前提到的价值亿万的错误不可能发生。

### 通配模式和 _ 占位符

```rust
let dice_roll = 9;
match dice_roll {
    // 对于前两个分支，匹配模式是字面值 3 和 7
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    // 最后一个分支则涵盖了所有其他可能的值
    other => move_player(other),
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
```

模式是我们命名为 `other` 的一个变量。`other` 分支的代码通过将其传递给 `move_player` 函数来使用这个变量。

即使我们没有列出 `u8` 所有可能的值，这段代码依然能够编译，因为最后一个模式将匹配所有未被特殊列出的值。
这种通配模式满足了 `match` 必须被穷尽的要求。
请注意，我们{++必须将通配分支放在最后，因为模式是按顺序匹配的++}。
如果我们在通配分支后添加其他分支，Rust 将会警告我们，因为此后的分支永远不会被匹配到。

Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 `_` ，这是一个特殊的模式，可以匹配任意值而不绑定到该值。
这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => reroll(),          // 使用 _ 丢弃值。
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}
```

最后，如果将无事发生。我们可以使用**单元值**（在“[元组类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E5%85%83%E7%BB%84%E7%B1%BB%E5%9E%8B)”一节中提到的空元组）作为 `_` 分支的代码：

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (),        // 单元值，无事发生。
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

在这里，明确告诉 Rust 我们{++不会使用与前面模式不匹配的值，并且这种情况下我们不想运行任何代码++}。

## if let控制流
