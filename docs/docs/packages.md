# 包、crate、模块

**当编写大型程序时，组织你的代码显得尤为重要。**

概念 “**作用域（scope）**”：代码所在的嵌套上下文有一组定义为 “**in scope**” 的名称。

当阅读、编写和编译代码时，程序员和编译器需要知道特定位置的特定名称是否引用了**变量**、**函数**、**结构体**、**枚举**、**模块**、**常量**或者其他有意义的**项**。
你可以创建作用域，以及改变哪些名称在作用域内还是作用域外。{++同一个作用域内不能拥有两个相同名称的项++}。

Rust 有许多功能可以**管理代码的组织**，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。这些功能。这有时被称为 “**模块系统**（the module system）”，包括：

- **包（Packages）**： Cargo 的一个功能，它允许你构建、测试和分享 crate。
- **Crates** ：一个模块的树形结构，它形成了库或二进制项目。
- **模块（Modules）和 use**： 允许你控制作用域和路径的私有性。
- **路径（path）**：一个命名例如结构体、函数或模块等项的方式

## 包和crate

`crate` 是 Rust 在编译时最小的代码单位。如果用 `rustc` 而不是 `cargo` 来编译一个文件，编译器还是会将那个文件认作一个 `crate`。
`crate` 可以包含模块，模块可以定义在其他文件，然后和 `crate` 一起编译，我们会在接下来的章节中遇到。

`crate` 有两种形式：**二进制项**和**库**。
二进制项 可以被编译为可执行程序，比如一个命令行程序或者一个服务器。{++它们必须有一个 `main` 函数来定义当程序被执行的时候所需要做的事情。++}
目前为止示例中所创建的 `crate` 都是二进制项。

{++**库** 并没有 main 函数，它们也不会编译为可执行程序，它们提供一些诸如函数之类的东西++}，使其他项目也能使用这些东西。
比如 `rand crate` 就提供了生成随机数的东西。大多数时间 Rustaceans 说的 `crate` 指的都是**库**，这与其他编程语言中 `library` 概念一致。

`crate root` 是一个源文件，Rust 编译器以它为起始点，并构成你的 `crate` 的根模块（我们将在 “[定义模块来控制作用域与私有性](https://kaisery.github.io/trpl-zh-cn/ch07-02-defining-modules-to-control-scope-and-privacy.html)” 一节深入解读）。

**包（package）** 是{++提供一系列功能的一个或者多个 crate++}。一个包会包含一个 `Cargo.toml` 文件，阐述如何去构建这些 `crate`。
**Cargo** 就是一个包含构建你代码的二进制项的包。Cargo 也包含这些二进制项所依赖的**库**。其他项目也能用 **Cargo** 库来实现与 **Cargo** 命令行程序一样的逻辑。

{++包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 `crate`(binary crate)，但是必须至少包含一个 `crate`（无论是库的还是二进制的）。++}

看看创建包的时候会发生什么。首先，输入命令 `cargo new`：

```shell
$ cargo new my-project
     Created binary (application) `my-project` package
$ ls my-project
Cargo.toml
src

# Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate 根。
$ ls my-project/src
main.rs

# 同样的，Cargo 知道如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate 根。

# crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。
```

在此，我们有了一个只包含 `src/main.rs` 的包，**意味着它只含有一个名为 my-project 的二进制 crate**。

如果一个包同时含有 `src/main.rs` 和 `src/lib.rs`，则它有**两个 crate**：一个二进制的和一个库的，且名字都与包相同。
{++通过将文件放在 `src/bin` 目录下，一个包可以拥有多个二进制 `crate`：**每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。**++}

## 定义模块

一个简单的参考，用来解释模块、路径、use关键词和pub关键词如何在编译器中工作，以及大部分开发者如何组织他们的代码。

{==

- **从crate根节点开始**: 当编译一个crate, 编译器首先在crate根文件（通常，对于一个库crate而言是`src/lib.rs`，对于一个二进制crate而言是`src/main.rs`）中寻找需要被编译的代码。
- **声明模块**: 在crate根文件中，你可以声明一个新模块；比如，你用`mod garden`声明了一个叫做garden的模块。编译器会在下列路径中寻找模块代码：
    - 内联，在大括号中，当`mod garden`后方不是一个分号而是一个大括号
    - 在文件 `src/garden.rs`
    - 在文件 `src/garden/mod.rs`
- **声明子模块**: 在除了crate根节点以外的其他文件中，你可以定义**子模块**。比如，你可能在`src/garden.rs`中定义了`mod vegetables`;。编译器会在以父模块命名的目录中寻找子模块代码：
    - 内联, 在大括号中，当`mod vegetables`后方不是一个分号而是一个大括号
    - 在文件 `src/garden/vegetables.rs`
    - 在文件 `src/garden/vegetables/mod.rs`
- **模块中的代码路径**: 一旦一个模块是你crate的一部分， 你可以在隐私规则允许的前提下，从同一个crate内的任意地方，通过代码路径引用该模块的代码。举例而言，一个`garden vegetables`模块下的Asparagus类型可以在`crate::garden::vegetables::Asparagus`被找到。
- **私有 vs 公用**: 一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用`pub mod`替代`mod`。为了使一个公用模块内部的成员公用，应当在声明前使用`pub`。
- **use 关键字**: 在一个作用域内，`use`关键字创建了一个成员的快捷方式，用来减少长路径的重复。在任何可以引用`crate::garden::vegetables::Asparagus`的作用域, 你可以通过 `use crate::garden::vegetables::Asparagus;`创建一个快捷方式，然后你就可以在作用域中只写Asparagus来使用该类型。

==}

这里创建一个名为`backyard`的**二进制`crate`**来说明这些规则。该`crate`的路径同样命名为backyard，该路径包含了这些文件和目录：

```shell
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs     # crate根文件
```

文件名: src/main.rs

```rust
use crate::garden::vegetables::Asparagus;

pub mod garden;  // 告诉编译器应该包含在src/garden.rs文件中发现的代码

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

文件名: src/garden.rs

```rust
pub mod vegetables;  // 意味着在src/garden/vegetables.rs中的代码也应该被包括。
```

文件名: src/garden/vegetables.rs

```rust
#[derive(Debug)]
pub struct Asparagus {}
```

### 对相关代码进行分组

**模块** 可以将一个 `crate` 中的代码进行分组，以提高可读性与重用性。
因为一个{++模块中的代码默认是私有的++}，所以还可以利用模块控制项的 **私有性**。
私有项是不可为外部使用的内在详细实现。也可以将模块和它其中的项标记为公开的，这样，外部代码就可以使用并依赖与它们。

在餐饮业，餐馆中会有一些地方被称之为 **前台**（front of house），还有另外一些地方被称之为 **后台**（back of house）。
前台是招待顾客的地方，在这里，店主可以为顾客安排座位，服务员接受顾客下单和付款，调酒师会制作饮品。
后台则是由厨师工作的厨房，洗碗工的工作地点，以及经理做行政工作的地方组成。

可以将函数放置到嵌套的模块中，来使我们的 crate 结构与实际的项目业务结构相同。通过执行 `cargo new --lib restaurant`，
来创建一个新的名为 `restaurant` 的库。然后将示例中所罗列出来的代码放入 `src/lib.rs` 中，来定义一些模块和函数。

文件名: src/lib.rs

```rust
// 一个包含了其他内置了函数的模块的 front_of_house 模块

// 定义一个模块，是以 mod 关键字为起始，然后指定模块的名字（叫做 front_of_house），并且用花括号包围模块的主体。
mod front_of_house {

    // 在模块内，我们还可以定义其他的模块，
    // hosting 模块
    mod hosting {

        // 模块还可以保存一些定义的其他项
        // 比如结构体、枚举、常量、traits、或者函数。
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // serving 模块
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

{++通过使用模块，可以将相关的定义分组到一起，并指出他们为什么相关++}。
程序员可以通过使用这段代码，更加容易地找到他们想要的定义，因为他们可以基于分组来对代码进行导航，而不需要阅读所有的定义。
程序员向这段代码中添加一个新的功能时，他们也会知道代码应该放置在何处，可以保持程序的组织性。

在前面提到了，`src/main.rs` 和 `src/lib.rs` 叫做 `crate` 根。
之所以这样叫它们是因为这两个文件的内容都分别在 `crate` 模块结构的根组成了一个名为 `crate` 的模块，该结构被称为 **模块树**（module tree）。

```shell
# 示例的模块树

# 这个树展示了一些模块是如何被嵌入到另一个模块的
# 还展示了一些模块是互为 兄弟（siblings） 的，这意味着它们定义在同一模块中
# （hosting 和 serving 被一起定义在 front_of_house 中）。
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

沿用家庭关系的比喻，如果一个模块 A 被包含在模块 B 中，我们将模块 A 称为模块 B 的 **子模块**（child），模块 B 则是模块 A 的 **父模块**（parent）。注意，整个模块树都植根于名为 `crate` 的隐式模块下。

这个模块树很像电脑上文件系统的**目录树**；这是一个非常恰当的类比！就像文件系统的目录，可以使用模块来组织你的代码。

## 引用模块

**使用路径的方式，就像在文件系统使用路径一样。**

路径有两种形式：

- **绝对路径**（absolute path）从 crate 根开始，以 `crate` 名或者`字面值 crate` 开头。
- **相对路径**（relative path）从当前模块开始，以 `self`、`super` 或`当前模块的标识符`开头。

绝对路径和相对路径都后跟一个或多个由双冒号（`::`）分割的标识符。

文件名: src/lib.rs

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    // 被定义在同一 crate 中，这意味着我们可以使用 crate 关键字为起始的绝对路径。
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    // 以 front_of_house 为起始
    // 以名称为起始，意味着该路径是相对路径。
    front_of_house::hosting::add_to_waitlist();
}

// 这个例子无法编译通过, 因为 hosting 子模块是私有的（默认）
```

选择使用相对路径还是绝对路径，还是要取决于你的项目。取决于你是更倾向于将项的定义代码与使用该项的代码分开来移动，还是一起移动。

build 报错:

```shell
$ cargo build
   Compiling restaurant v0.1.0 (file:///projects/restaurant)
error[E0603]: module `hosting` is private
 --> src/lib.rs:9:28
  |
9 |     crate::front_of_house::hosting::add_to_waitlist();
  |                            ^^^^^^^ private module
  |
note: the module `hosting` is defined here
 --> src/lib.rs:2:5
  |
2 |     mod hosting {
  |     ^^^^^^^^^^^

error[E0603]: module `hosting` is private
  --> src/lib.rs:12:21
   |
12 |     front_of_house::hosting::add_to_waitlist();
   |                     ^^^^^^^ private module
   |
note: the module `hosting` is defined here
  --> src/lib.rs:2:5
   |
2  |     mod hosting {
   |     ^^^^^^^^^^^

For more information about this error, try `rustc --explain E0603`.
error: could not compile `restaurant` due to 2 previous errors
```

错误信息说 `hosting` 模块是私有的。换句话说，示例拥有 `hosting` 模块和 `add_to_waitlist` 函数的的正确路径，但是 Rust 不让我们使用，因为它**不能访问私有片段**。

模块不仅对于组织代码很有用。还定义了 Rust 的 **私有性边界**（privacy boundary）：这条界线不允许外部代码了解、调用和依赖被封装的实现细节。所以，如果希望创建一个私有函数或结构体，可以将其放入模块。

{++Rust 中**默认**所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。++}
这是因为子模块封装并隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文。
继续拿餐馆作比喻，把私有性规则想象成餐馆的后台办公室：餐馆内的事务对餐厅顾客来说是不可知的，但办公室经理可以洞悉其经营的餐厅并在其中做任何事情。

Rust 选择以这种方式来实现模块系统功能，因此默认隐藏内部实现细节。这样一来，就知道可以更改内部代码的哪些部分而不会破坏外部代码。还可以通过使用 `pub` 关键字来创建公共项，使子模块的内部部分暴露给上级模块。

### pub关键字

在上面示例的报错信息中，它告诉我们 `hosting` 模块是私有的。想让父模块中的 `eat_at_restaurant` 函数可以访问子模块中的 `add_to_waitlist` 函数，因此我们使用 `pub` 关键字来标记 `hosting` 模块，

文件名: src/lib.rs

```rust
mod front_of_house {
    // 添加pub关键字使模块公开
    pub mod hosting {
        // 添加pub关键字使函数公开
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

// 现在代码可以编译通过了！
```

{++私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。++}

### super相对路径

{++还可以使用 `super` 开头来构建从父模块开始的相对路径。++} 似于文件系统中以 `..` 开头的语法。

文件名: src/lib.rs

```rust
// 使用以 super 开头的相对路径从父目录开始调用函数

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super表示父级的模块
        super::serve_order();
    }

    fn cook_order() {}
}
```

### 公有的结构体和枚举

还可以使用 `pub` 来设计公有的**结构体**和**枚举**，不过有一些额外的细节需要注意。
{++如果我们在一个结构体定义的前面使用了 `pub` ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。可以根据情况决定每个字段是否公有。++}

在示例中，定义了一个公有结构体 `back_of_house:Breakfast`，其中有一个公有字段 `toast` 和私有字段 `seasonal_fruit`。

这个例子模拟的情况是，在一家餐馆中，顾客可以选择随餐附赠的面包类型，但是厨师会根据季节和库存情况来决定随餐搭配的水果。餐馆可用的水果变化是很快的，所以顾客不能选择水果，甚至无法看到他们将会得到什么水果。

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // toast 字段是公有的, 所以可以使用点号来随意的读写 toast 字段。
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // 因为 seasonal_fruit 是私有的, 所以不能用点号来随意的读写 seasonal_fruit 字段。
    // meal.seasonal_fruit = String::from("blueberries");
}
```

{==

> **注意**
>
> 因为 `back_of_house::Breakfast` 具有私有字段，所以这个结构体**需要提供一个公共的关联函数来构造 Breakfast 的实例**(这里命名为 `summer`)。
> 如果 **Breakfast** 没有这样的函数，将无法在 **eat_at_restaurant** 中创建 **Breakfast** 实例，
> 因为我们不能在 **eat_at_restaurant** 中设置私有字段 `seasonal_fruit` 的值。

==}

与之相反，如果将**枚举**设为公有，则它的所有成员都将变为公有。只需要在 `enum` 关键字前面加上 **pub**

```rust
// 设计公有枚举，使其所有成员公有

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

**如果枚举成员不是公有的，那么枚举会显得用处不大；**
给枚举的所有成员挨个添加 `pub` 是很令人恼火的，因此枚举成员默认就是公有的。
结构体通常使用时，不必将它们的字段公有化，因此结构体遵循常规，内容全部是私有的，除非使用 `pub` 关键字。

## use关键字

可以使用 `use` 关键字将路径一次性引入作用域，然后调用该路径中的项，就如同它们是本地项一样。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 使用 use 将模块引入作用域
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

```

作用域中增加 `use` 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）

通过在 `crate` 根增加 `use crate::front_of_house::hosting`，现在 `hosting` 在作用域中就是有效的名称了，
如同 `hosting` 模块**被定义**于 `crate` 根一样。通过 `use` 引入作用域的路径**也会检查私有性**，同其它路径一样。

```rust
// 引入作用域的习惯用法

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 使用 use 和相对路径来将一个项引入作用域
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

### 惯用的 use 路径

 `use` 将函数引入作用域的习惯用法。就是使用 use 将函数的父模块引入作用域，必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。
 如下示例中的代码不清楚 `add_to_waitlist` 是在哪里被定义的。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // 此时容易不知 add_to_waitlist 是在哪里被定义的。
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

使用 `use` 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。

```rust
// 将 HashMap 引入作用域的习惯用法
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

这种习惯用法背后没有什么硬性要求：它只是一种惯例，人们已经习惯了以这种方式阅读和编写 Rust 代码。

这个习惯用法有一个例外，那就是我们想使用 `use` 语句将两个具有相同名称的项带入作用域，因为 Rust 不允许这样做。

```rust
// 展示了如何将两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们。

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// 使用父模块可以区分这两个 Result 类型。
```

### as 关键字

使用 `use` 将两个同名类型引入同一作用域这个问题还有另一个解决办法：**在这个类型的路径后面，使用 `as` 指定一个新的本地名称或者别名。**

```rust
// 通过 as 重命名其中一个 Result 类型
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### pub use重导出

使用 `use` 关键字，**将某个名称导入当前作用域后，这个名称在此作用域中就可以使用了，但它对此作用域之外还是私有的**。
{++如果想让其他人调用我们的代码时，也能够正常使用这个名称，就好像它本来就在当前作用域一样，那我们可以将 `pub` 和 `use` 合起来使用。++}
这种技术被称为 “**重导出**（re-exporting）”：我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

通过 `pub use`重导出，外部代码现在可以通过新路径 `restaurant::hosting::add_to_waitlist` 来调用 `add_to_waitlist` 函数。
如果没有指定 `pub use`，外部代码需在其作用域中调用 `restaurant::front_of_house::hosting::add_to_waitlist`。

当你代码的内部结构与调用你代码的程序员所想象的结构不同时，重导出会很有用。
例如，在这个餐馆的比喻中，经营餐馆的人会想到“前台”和“后台”。但顾客在光顾一家餐馆时，可能不会以这些术语来考虑餐馆的各个部分。
使用 `pub use`，我们可以使用一种结构编写代码，却将不同的结构形式暴露出来。这样做使我们的库**井井有条**，**也使开发这个库的程序员和调用这个库的程序员都更加方便。**

### 使用外部包

在编写猜猜看游戏时。项目使用了一个外部包，`rand`，来生成随机数。为了在项目中使用 `rand`，在 `Cargo.toml` 中加入了如下行：

```rust
rand = "0.8.3" // 告诉了 Cargo 要从 crates.io 下载 rand 和其依赖，并使其可在项目代码中使用。
```

接着，为了将 `rand` 定义引入项目包的作用域，加入一行 `use` 起始的包名，它以 `rand` 包名开头并列出了需要引入作用域的项。曾将 `Rng trait` 引入作用域并调用了 `rand::thread_rng` 函数：

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
```

`crates.io` 上有很多 Rust 社区成员发布的包，将其引入自己的项目都需要一道相同的步骤：{++在 Cargo.toml 列出它们并通过 use 将其中定义的项引入项目包的作用域中。++}

注意**标准库（std）**对于你的包来说也是外部 `crate`。因为标准库随 Rust 语言一同分发，无需修改 `Cargo.toml` 来引入 `std`，不过需要通过 `use` 将标准库中定义的项引入项目包的作用域中来引用它们，比如使用的 `HashMap`：

```rust
use std::collections::HashMap;  // 以标准库 crate 名 std 开头的绝对路径。
```

### 嵌套路径

当需要引入很多定义于相同包或相同模块的项时，为每一项单独列出一行会占用源码很大的空间。

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--
```

可以使用嵌套路径将相同的项在一行中引入作用域。

```rust
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

在较大的程序中，使用嵌套路径从相同包或模块中引入很多项，可以显著减少所需的独立 use 语句的数量！

可以在路径的任何层级使用嵌套路径，这在组合两个共享子路径的 use 语句时非常有用。

```rust
use std::io;
use std::io::Write;
```

为:

```rust
// 同时引入作用域。
use std::io::{self, Write};
```

### glob运算符

如果希望将一个路径下 所有 公有项引入作用域，可以指定路径后跟 `*`，glob 运算符：

```rust
use std::collections::*;

// 使用 glob 运算符时请多加小心！
// Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。
```

`glob` 运算符经常用于测试模块 `tests` 中，这时会将所有内容引入作用域；

`glob` 运算符有时也用于 `prelude` 模式；查看 [标准库中的文档](https://doc.rust-lang.org/std/prelude/index.html#other-preludes) 了解这个模式的更多细节。

## 模块拆分

**当模块变得更大时，可能想要将它们的定义移动到单独的文件中，从而使代码更容易阅读。**

例如，将 `front_of_house` 模块移动到属于它自己的文件 `src/front_of_house.rs` 中，通过改变 `crate` 根文件，使其包含示例的代码。
在这个例子中，crate 根文件是 `src/lib.rs`，这也同样适用于以 `src/main.rs` 为 `crate` 根文件的二进制 `crate` 项。

```rust
// 文件名: src/lib.rs
// 声明 front_of_house 模块，其内容将位于 src/front_of_house.rs

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

`src/front_of_house.rs` 会获取 `front_of_house` 模块的定义内容

```rust
// 文件名: src/front_of_house.rs
// 在 src/front_of_house.rs 中定义 front_of_house 模块
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```

在 `mod front_of_house` 后使用**分号**，而不是**代码块**，这将告诉 Rust 在另一个与模块同名的文件中加载模块的内容。
继续重构例子，将 `hosting` 模块也提取到其自己的文件中，仅对 `src/front_of_house.rs` 包含 hosting 模块的声明进行修改：

```rust
// 文件名: src/front_of_house.rs
pub mod hosting;
```

接着我们创建一个 `src/front_of_house` 目录和一个包含 `hosting` 模块定义的 `src/front_of_house/hosting.rs` 文件：

```rust
// 文件名: src/front_of_house/hosting.rs
pub fn add_to_waitlist() {}
```

模块树依然保持相同，`eat_at_restaurant` 中的函数调用也无需修改继续保持有效，即便其定义存在于不同的文件中。{++这个技巧可以在模块代码增长时，将它们移动到新文件中。++}

注意，`src/lib.rs` 中的 `pub use crate::front_of_house::hosting` 语句是没有改变的，在文件作为 `crate` 的一部分而编译时，`use` 不会有任何影响。
`mod` 关键字声明了模块，Rust 会在与模块同名的文件中查找模块的代码。

## 总结

Rust 提供了将包分成多个 `crate`，将 `crate` 分成模块，以及通过指定绝对或相对路径从一个模块引用另一个模块中定义的项的方式。
可以通过使用 `use` 语句将路径引入作用域，这样在多次使用时可以使用更短的路径。
{++模块定义的代码默认是私有的，不过可以选择增加 `pub` 关键字使其定义变为公有。++}
