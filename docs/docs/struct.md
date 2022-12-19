# 结构体

**struct**，或者 **structure**，是一个自定义数据类型，允许你包装和命名多个相关的值，从而形成一个有意义的组合。

## 定义和实例化

和元组一样，结构体的每一部分可以是不同类型。但不同于元组，**结构体需要命名各部分数据以便能清楚的表明其值的意义**。由于有了这些名字，结构体比元组更灵活：{++不需要依赖顺序来指定或访问实例中的值++}。

定义结构体，需要使用 **struct** 关键字并为整个结构体提供一个名字。结构体的名字需要描述它所组合的数据的意义。接着，在大括号中，定义每一部分数据的名字和类型，称为 **字段**（field）。

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

一旦定义了结构体后，为了使用它，通过为每个字段指定具体值来创建这个结构体的 **实例**。创建一个实例需要以结构体的名字开头，接着在大括号中使用 **key: value** 键-值对的形式提供字段，其中 **key** 是字段的名字，**value** 是需要存储在字段中的数据值。{++实例中字段的顺序**不需要**和它们在结构体中声明的顺序一致。++}

为了从结构体中获取某个特定的值，可以使用点号。

```rust
fn main() {
    // 实例化一个结构体，其中字段顺序不需要和定义时的一直。
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 使用点号获取结构体中的某个特定的值
    user1.email = String::from("anotheremail@example.com");
}
```

{++注意整个实例必须是可变的；Rust 并**不允许**只将某个字段标记为可变。++}

另外需要注意同其他任何表达式一样，可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例。

```rust
// 创建一个结构体的实例并返回，
// 有点类似于其他语言中的初始化一个结构体。
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,       // 默认值为true
        sign_in_count: 1,   // 默认值为1
    }
}
```

### 字段初始化简写

因为示例中的参数名与字段名都完全相同，则可以使用 **字段初始化简写语法**（field init shorthand）来重写 `build_user`，这样其行为与之前完全相同，不过无需重复 `email` 和 `username`.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,          // 字段初始化简写语法
        username,       // 字段初始化简写语法
        active: true,
        sign_in_count: 1,
    }
}
```

### 从其他实例创建实例

使用旧实例的大部分值但改变其部分值来创建一个新的结构体实例通常是很有用的。这可以通过 **结构体更新语法**（struct update syntax）实现。

```rust
fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,       // 从另一个结构体的字段值来初始化
        username: user1.username,   // 从另一个结构体的字段值来初始化
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

同时，可以使用 `..` 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值。

```rust
fn main() {
    // --snip--

    let user2 = User {
        // 可以选择以任何顺序为任意字段指定值，而不用考虑结构体定义中字段的顺序。
        email: String::from("another@example.com"),
        ..user1     // ..语法指定剩余未显式设置值的字段
                    // ..user1 必须放在最后，以指定其余的字段应从 user1 的相应字段中获取其值
    };
}
```

> **注意**
> 结构更新语法就像带有 = 的赋值，因为它移动了数据，就像在“[变量与数据交互的方式（一）：移动](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html#%E5%8F%98%E9%87%8F%E4%B8%8E%E6%95%B0%E6%8D%AE%E4%BA%A4%E4%BA%92%E7%9A%84%E6%96%B9%E5%BC%8F%E4%B8%80%E7%A7%BB%E5%8A%A8)”部分讲到的一样。
> 在这个例子中，在创建 `user2` 后不能再使用 `user1`，因为 `user1` 的 `username` 字段中的 `String` 被移到 `user2` 中。
> 如果我们给 `user2` 的 `email` 和 `username` 都赋予新的 `String` 值，从而只使用 `user1` 的 `active` 和 `sign_in_count` 值，那么 `user1` 在创建 `user2` 后仍然有效。
> `active` 和 `sign_in_count` 的类型是实现 `Copy trait` 的类型，所以在“[变量与数据交互的方式（二）：克隆](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html#%E5%8F%98%E9%87%8F%E4%B8%8E%E6%95%B0%E6%8D%AE%E4%BA%A4%E4%BA%92%E7%9A%84%E6%96%B9%E5%BC%8F%E4%BA%8C%E5%85%8B%E9%9A%86)” 部分讨论的行为同样适用。

### 元组结构体

定义与[元组](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E5%85%83%E7%BB%84%E7%B1%BB%E5%9E%8B)类似的结构体，称为 **元组结构体**（tuple structs）。{++元组结构体有着结构体名称提供的含义，但没有具体的字段名，只有字段的类型。++}
当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时，元组结构体是很有用的，这时像常规结构体那样为每个字段命名就显得多余和形式化了。

要定义元组结构体，以 **struct** 关键字和结构体名开头并后跟元组中的类型。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

> **注意**
>
> `black` 和 `origin` 值的类型不同，因为它们是不同的元组结构体的实例。
>
> {++定义的每一个结构体有其自己的类型，即使结构体中的字段可能有着相同的类型++}。
>
> 例如，一个获取 `Color` 类型参数的函数不能接受 `Point` 作为参数，即便这两个类型都由三个 i32 值组成。
>
> 在其他方面，元组结构体实例类似于元组，可以将它们解构为单独的部分，也可以使用 `.` 后跟索引来访问单独的值。

### 类单元结构体

也可以定义一个没有任何字段的结构体！它们被称为 **类单元结构体**（unit-like structs）因为它们类似于 ()，即“[元组类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E5%85%83%E7%BB%84%E7%B1%BB%E5%9E%8B)”一节中提到的 `unit` 类型。
类单元结构体常常在想要在某个类型上实现 `trait` 但不需要在类型中存储数据的时候发挥作用。

```rust

// 类单元结构体
struct AlwaysEqual; // 不需要花括号或圆括号！

fn main() {

    // 以类似的方式在 subject 变量中获得 AlwaysEqual 的实例
    let subject = AlwaysEqual;      // 使用定义的名称，不需要任何花括号或圆括号。

    // 想象一下，实现这个类型的行为，即每个实例始终等于每一个其他类型的实例，也许是为了获得一个已知的结果以便进行测试。
}
```

> **结构体数据的所有权**
>
> 在示例 `User` 结构体的定义中，我们使用了自身拥有所有权的 `String` 类型而不是 `&str` 字符串 `slice` 类型。这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
>
> 可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上 **生命周期**（lifetimes），这是一个[第十章](https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html)会讨论的 Rust 功能。
>
> 生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，比如这样：
>
> ```rust
> struct User {
>     active: bool,
>     username: &str,
>     email: &str,
>     sign_in_count: u64,
> }
> 
> fn main() {
>     let user1 = User {
>         email: "someone@example.com",
>         username: "someusername123",
>         active: true,
>         sign_in_count: 1,
>     };
> }
> ```
>
> 编译器会抱怨它需要生命周期标识符：
>
> ```shell
> $ cargo run
>    Compiling structs v0.1.0 (file:///projects/structs)
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:3:15
>   |
> 3 |     username: &str,
>   |               ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 ~     username: &'a str,
>   |
> 
> error[E0106]: missing lifetime specifier
>  --> src/main.rs:4:12
>   |
> 4 |     email: &str,
>   |            ^ expected named lifetime parameter
>   |
> help: consider introducing a named lifetime parameter
>   |
> 1 ~ struct User<'a> {
> 2 |     active: bool,
> 3 |     username: &str,
> 4 ~     email: &'a str,
>   |
> 
> For more information about this error, try `rustc --explain E0106`.
> error: could not compile `structs` due to 2 previous errors
> ```
>
> [第十章](https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html)会讲到如何修复这个问题以便在结构体中存储引用，不过现在，会使用像 `String` 这类拥有所有权的类型来替代 `&str` 这样的引用以修正这个错误。

## 示例程序

为了理解何时会需要使用结构体，这里编写一个计算长方形面积的程序。

使用 `Cargo` 新建一个叫做 `rectangles` 的二进制程序，它获取以像素为单位的长方形的宽度和高度，并计算出长方形的面积。

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("矩形的面积是{}个像素", area(width1, height1))
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

修改这段代码来使它的意义更加明确:

```rust
// 函数 area 本应该计算一个长方形的面积，不过函数却有两个参数。
// 这两个参数是相关联的，不过程序本身却没有表现出这一点。
// 将长度和宽度组合在一起将更易懂也更易处理。
fn area(width: u32, height: u32) -> u32 {
    // ...
}
```

### 元组重构

使用元组的另一个程序版本:

```rust
fn main() {
    let rect = (30, 50);

    println!("矩形的面积是{}个像素", area(rect))
}

// 元组帮助我们增加了一些结构性，并且现在只需传一个参数。
// 不过在另一方面，这个版本却有一点不明确了：元组并没有给出元素的名称，所以计算变得更费解了，因为不得不使用索引来获取元组的每一部分：
fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

### 结构体重构

可以使用结构体为数据命名来为其赋予意义。将我们正在使用的元组转换成一个有整体名称而且每个部分也有对应名字的结构体，

```rust
// 定义一个矩形的结构体
// 在大括号中定义了字段 width 和 height，类型都是 u32。
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 创建了一个具体的 Rectangle 实例
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形的面积是{}个像素", area(&rect1));
}

// 计算一个矩形的面积
// 被定义为接收一个名叫 rectangle 的参数，其类型是一个结构体 Rectangle 实例的不可变借用(引用)。
// 借用结构体而不是获取它的所有权，这样 main 函数就可以保持 rect1 的所有权并继续使用它，所以这就是为什么在函数签名和调用的地方会有 &。
fn area(rectangle: &Rectangle) -> u32 {
    // 访问 Rectangle 实例的 width 和 height 字段
    // 注意，访问对结构体的引用的字段不会移动字段的所有权, 所以会经常看到对结构体的引用
    rectangle.width * rectangle.height
}
```

使用 `Rectangle` 的 `width` 和 `height` 字段，计算 `Rectangle` 的面积。这表明宽高是相互联系的，并为这些值提供了描述性的名称而不是使用元组的索引值 `0` 和 `1` 。{++结构体胜在更清晰明了++}。

### 派生Trait

在调试程序时打印出 Rectangle 实例来查看其所有字段的值非常有用。如果能像前面那样尝试使用 [println! 宏](https://doc.rust-lang.org/std/macro.println.html)就好了。但这并不行。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形是{}", rect1);
}
```

出现带有如下核心信息的错误：

```shell
error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
```

`println! 宏`能处理很多类型的格式，不过，`{}` 默认告诉 `println!` 使用被称为 **Display** 的格式：**意在提供给直接终端用户查看的输出**。

目前为止见过的基本类型都默认实现了 `Display`格式，因为它就是向用户展示 `1` 或其他任何**基本类型**的唯一方式。

不过对于结构体，`println!` 应该用来输出的格式是不明确的，因为这有更多显示的可能性：是否需要逗号？需要打印出大括号吗？所有字段都应该显示吗？
由于这种不确定性，Rust 不会尝试猜测我们的意图，所以结构体并没有提供一个 **Display** 实现来使用 `println!` 与 `{}` 占位符。

如果我们继续阅读错误，将会发现这个有帮助的信息：

```shell
help: the trait `std::fmt::Display` is not implemented for `Rectangle`
note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
```

现在 `println!` 宏调用看起来像 `println!("rect1 is {:?}", rect1);` 这样。在 `{}` 中加入 `:?` 指示符告诉 `println!` 我们想要使用叫做 `Debug` 的输出格式。
`Debug` 是一个 `trait`，它允许我们以一种对开发者有帮助的方式打印结构体，以便当我们调试代码时能看到它的值。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形是{:?}", rect1); // 添加 :? 尝试使用Debug的输出格式
}
```

这样调整后再次运行程序。见鬼了！仍然能看到一个错误：

```shell
error[E0277]: `Rectangle` doesn't implement `Debug`
```

不过编译器又一次给出了一个有帮助的信息：

```shell
help: the trait `Debug` is not implemented for `Rectangle`
note: add `#[derive(Debug)]` to `Rectangle` or manually `impl Debug for Rectangle`
```

#### Debug输出结构体

Rust **确实** 包含了打印出调试信息的功能，不过我们必须为结构体{++**显式**++}选择这个功能。为此，在结构体定义之前加上外部属性 `#[derive(Debug)]`

```rust
// 添加打印调试的功能
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形是{:?}", rect1); // 尝试使用Debug的输出格式
}
```

再运行这个程序时，就不会有任何错误，并会出现如下输出：

```shell
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
矩形是Rectangle { width: 30, height: 50 }
```

这并不是最漂亮的输出，不过它显示这个实例的所有字段，毫无疑问这对调试有帮助。

当我们有一个更大的结构体时，能有更**易读**一点的输出就好了，为此可以使用 `{:#?}` 替换 `println!` 字符串中的 `{:?}`。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形是{:#?}", rect1); // 尝试使用Debug的输出格式
}
```

在这个例子中使用 `{:#?}` 风格将会输出：

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.48s
     Running `target/debug/rectangles`
矩形是Rectangle {
    width: 30,
    height: 50,
}
```

#### dbg!宏

另一种使用 **Debug** 格式打印数值的方法是使用 [dbg! 宏](https://doc.rust-lang.org/std/macro.dbg.html)。{++`dbg! 宏`可以接收一个**表达式**的所有权，打印出代码中调用 `dbg!` 宏时所在的文件和行号，以及该表达式的结果值，并返回该值的所有权。++} （与 `println!` 宏相反，其接收的是引用）

> **注意**
>
> 调用 `dbg! 宏`会打印到标准错误控制台流（stderr），与 println! 不同，后者会打印到标准输出控制台流（stdout）。我们将在[第十二章 “将错误信息写入标准错误而不是标准输出” 一节](https://kaisery.github.io/trpl-zh-cn/ch12-06-writing-to-stderr-instead-of-stdout.html)中更多地讨论 `stderr` 和 `stdout`。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        // 可以把 dbg! 放在表达式 30 * scale 周围，因为 dbg! 会返回表达式的值的所有权，
        // 所以 width 字段将获得相同的值，就像我们在那里没有 dbg! 调用一样。
        width: dbg!(scale * 30),
        height: 50,
    };

    // 不希望 dbg! 拥有 rect1 的所有权，所以在这里调用 dbg! 时仅传递一个引用。
    dbg!(&rect1);
}
```

```rust
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] scale * 30 = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

这个输出使用了更为易读的 **Debug** 格式。当你试图弄清楚你的代码在做什么时，`dbg! 宏`可能真的很有帮助!

## 方法

**方法（method）**与函数类似：{++它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码++}。
不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是**枚举**或 **trait** 对象的上下文），并且它们第一个参数总是 `self`，它代表调用该方法的结构体实例。

### 定义方法

把前面实现的获取一个 `Rectangle` 实例作为参数的 `area` 函数，改写成一个定义于 `Rectangle` 结构体上的 `area` 方法:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl 是 implementation 的缩写
// impl 块中的所有内容都将与 Rectangle 类型相关联
impl Rectangle {
    // 定义 与 Rectangle 相关联的 area方法
    // 使用 &self 来替代 rectangle: &Rectangle
    // &self 实际上是 self: &Self 的缩写
    // 这里选择 `&self` 的理由跟在函数版本中使用 `&Rectangle` 是相同的：并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
    // 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
    // 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时想要防止调用者在转换之后使用原始的实例。
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形的面积是: {} 像素", rect1.area())  // 调用Rectangle的关联方法area
}
```

{++在一个 impl 块中，Self 类型是 impl 块的类型的别名。方法的第一个参数必须有一个名为 self 的Self 类型的参数++}，所以 Rust 让你在第一个参数位置上只用 self 这个名字来缩写。

> **注意**
>
> 我们仍然需要在 `self` 前面使用 `&` 来表示这个方法借用了 `Self` 实例，就像我们在 `rectangle: &Rectangle` 中做的那样。
> 方法可以选择获得 `self` 的所有权，或者像这里一样**不可变地借用** `self`，或者**可变地借用** `self`，就跟其他参数一样。

使用**方法**替代**函数**，除了可使用方法语法和不需要在每个函数签名中重复 `self` 的类型之外，其主要好处在于组织性。
我们将某个类型实例能做的所有事情都一起放入 `impl` 块中，而不是让将来的用户在我们的库中到处寻找 `Rectangle` 的功能。

#### 注意

我们可以选择将方法的名称与结构中的一个字段相同。例如，我们可以在 Rectangle 上定义一个方法，并命名为 width：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形的宽度不为零: {}", rect1.width())
}
```

编程时**可以出于任何目的，在同名的方法中使用同名的字段**

在 main 中，当我们在 `rect1.width` 后面加上括号时。Rust 知道我们指的是方法 `width`。当我们不使用圆括号时，Rust 知道我们指的是字段 `width`。

通常，但并不总是如此，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为 `getters`，
Rust 并不像其他一些语言那样为结构字段自动实现它们。
Getters 很有用，因为你可以把字段变成私有的，但方法是公共的，这样就可以把对字段的只读访问作为该类型公共 API 的一部分。
我们将在[第七章](https://kaisery.github.io/trpl-zh-cn/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)中讨论什么是公有和私有，以及如何将一个字段或方法指定为公有或私有。

### 参数方法

通过实现 `Rectangle` 结构体上的另一方法来练习使用方法

让一个 `Rectangle` 的实例获取另一个 `Rectangle` 实例，如果 self （第一个 `Rectangle`）能完全包含第二个长方形则返回 true；否则返回 false。

```rust
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("矩形rect1可以包含rect2: {}", rect1.can_hold(&rect2));
    println!("矩形rect2可以包含rect3: {}", rect2.can_hold(&rect3));
}
```

同时希望看到如下输出，因为 rect2 的两个维度都小于 rect1，而 rect3 比 rect1 要宽：

```shell
矩形rect1可以包含rect2: true
矩形rect2可以包含rect3: false
```

```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 希望 main 保持 other 的所有权，这样就可以在调用这个方法后继续使用它。
    // can_hold 的返回值是一个布尔值，其实现会分别检查 self 的宽高是否都大于另一个 Rectangle。
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

### 关联函数

所有在 `impl` 块中定义的函数被称为 **关联函数**（associated functions），因为它们与 `impl` 后面命名的类型相关。

可以定义不以 `self` 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例。我们已经使用了一个这样的函数：**在 String 类型上定义的 String::from 函数**。

{++不是方法的关联函数经常被用作返回一个结构体新实例的构造函数++}。这些函数的名称通常为 `new` ，但 `new` 并不是一个关键字。
例如我们可以提供一个叫做 `square` 关联函数，它接受一个维度参数并且同时作为宽和高，这样可以更轻松的创建一个正方形 `Rectangle` 而不必指定两次同样的值：

```rust
impl Rectangle {
    // 创建一个正方形的关联函数，不是方法！！！
    // 关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，在这里是 Rectangle
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
```

使用结构体名和 `::` 语法来调用这个关联函数：比如 `let sq = Rectangle::square(3);`。
这个函数位于结构体的命名空间中：`::` 语法用于**关联函数**和**模块创建的命名空间**。[第七章](https://kaisery.github.io/trpl-zh-cn/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)会讲到模块。

### 多个 impl 块

每个结构体都允许拥有多个 `impl` 块。例如，下面示例中的代码等同于上面示例中代码，但每个方法有其自己的 impl 块。

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

这里没有理由将这些方法分散在多个 `impl` 块中，不过这是有效的语法。[第十章](https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html)讨论泛型和 trait 时会看到实用的多 `impl` 块的用例。

## 总结

{++结构体让你可以创建出在你的领域中有意义的自定义类型++}。
通过结构体，我们可以**将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰**。
在 `impl` 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的**实例**所具有的行为。

但结构体并不是创建自定义类型的唯一方法： Rust 的枚举功能也可以！
