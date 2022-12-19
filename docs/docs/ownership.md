# 所有权

Rust 的核心功能（之一）是 所有权（ownership）。

所有程序都必须管理其运行时使用计算机内存的方式。

一些语言中具有垃圾回收机制，在程序运行时有规律地寻找不再使用的内存；

在另一些语言中，程序员必须亲自分配和释放内存。

Rust 则选择了第三种方式：{++通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。++}

## 简介

> **栈（Stack）与堆（Heap）**
>
> 在很多语言中，你并不需要经常考虑到栈与堆。不过在像 Rust 这样的系统编程语言中，值是位于栈上还是堆上在更大程度上影响了语言的行为以及为何必须做出这样的抉择。
>
> 栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同。**栈以放入值的顺序存储值并以相反顺序取出值**。这也被称作 {++后进先出++}（last in, first out）。想象一下一叠盘子：当增加更多盘子时，把它们放在盘子堆的顶部，当需要盘子时，也从顶部拿走。不能从中间也不能从底部增加或拿走盘子！增加数据叫做 {++进栈++}（pushing onto the stack），而移出数据叫做 {++出栈++}（popping off the stack）。**栈中的所有数据都必须占用已知且固定的大小**。*在编译时大小未知或大小可能变化的数据，要改为存储在堆上*。 堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。{++内存分配器++}（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 {++指针++}（pointer）。这个过程称作 {++在堆上分配内存++}（allocating on the heap），有时简称为 “**分配**”（allocating）。（将数据推入栈中并不被认为是分配）。因为指向放入堆中数据的指针是已知的并且大小是固定的，你可以将该指针存储在栈上，不过当需要实际数据时，必须访问指针。想象一下去餐馆就座吃饭。当进入时，你说明有几个人，餐馆员工会找到一个够大的空桌子并领你们过去。如果有人来迟了，他们也可以通过询问来找到你们坐在哪。
>
> {++入栈比在堆上分配内存要快++}，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，*在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。*
>
> {++访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问++}。现代处理器在内存中跳转越少就越快（缓存）。继续类比，假设有一个服务员在餐厅里处理多个桌子的点菜。在一个桌子报完所有菜后再移动到下一个桌子是最有效率的。从桌子 A 听一个菜，接着桌子 B 听一个菜，然后再桌子 A，然后再桌子 B 这样的流程会更加缓慢。出于同样原因，处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。
>
> {++当你的代码调用一个函数时，传递给函数的值（包括可能指向堆上数据的指针）和函数的局部变量被压入栈中。当函数结束时，这些值被移出栈。++}
>
> {++跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的++}。一旦理解了所有权，你就不需要经常考虑栈和堆了，不过明白了所有权的主要目的就是为了管理堆数据，能够帮助解释为什么所有权要以这种方式工作。

### 所有权规则

首先，让我们看一下所有权的规则.

{++

1. Rust 中的每一个值都有一个 **所有者**（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

++}

#### 变量作用域

作用域是一个项（item）在程序中有效的范围。假设有这样一个变量：

```rust
{                      // s 在这里无效, 它尚未声明
    let s = "hello";   // 从此处起，s 是有效的

    // 使用 s
    println!("s is {s}")
}                      // 此作用域已结束，s 不再有效
```

两个重要的时间点：

> - 当 s **进入作用域** 时，它就是有效的。
> - 这一直持续到它 **离开作用域** 为止。

#### String 类型

已经见过字符串字面值，即被硬编码进程序里的字符串值。字符串字面值是很方便的，不过它们并不适合使用文本的每一种场景。原因之一就是{++它们是不可变的++}。另一个原因是{++并非所有字符串的值都能在编写代码时就知道++}：例如，要是想获取用户输入并存储该怎么办呢？为此，Rust 有第二个字符串类型，String。这个类型管理被分配到堆上的数据，所以能够存储在编译时未知大小的文本。可以使用 `from` 函数基于字符串字面值来创建 `String`，如下：

```rust
let s = String::from("hello");
// 两个冒号 :: 是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。
```

**可以** 修改此类字符串 ：

```rust
 let mut s = String::from("hello");

s.push_str(", world!"); // push_str() 在字符串后追加字面值

println!("{}", s); // 将打印 `hello, world!`

// 那么这里有什么区别呢？
// 为什么 String 可变而字面值却不行呢？
// 区别在于两个类型对内存的处理上。
```

#### 内存与分配

就字符串字面值来说，我们在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中。这使得字符串字面值快速且高效。
不过这些特性都只得益于字符串字面值的不可变性。
不幸的是，我们不能为了每一个在编译时大小未知的文本而将一块内存放入二进制文件中，并且它的大小还可能随着程序运行而改变。

对于 String 类型，为了支持一个可变，可增长的文本片段，**需要在堆上分配一块在编译时未知大小的内存来存放内容**。这意味着：

- 必须在运行时向内存分配器（memory allocator）请求内存。
- 需要一个当我们处理完 String 时将内存返回给分配器的方法。

第一部分由我们完成：当调用 **String::from** 时，它的实现 (implementation) 请求其所需的内存。这在编程语言中是非常通用的。

然而，第二部分实现起来就各有区别了。在有 {++垃圾回收++}（garbage collector，**GC**）的语言中， **GC** 记录并清除不再使用的内存，而我们并不需要关心它。在大部分没有 **GC** 的语言中，识别出不再使用的内存并调用代码显式释放就是我们的责任了，跟请求内存的时候一样。从历史的角度上说正确处理内存回收曾经是一个困难的编程问题。如果忘记回收了会浪费内存。如果过早回收了，将会出现无效变量。如果重复回收，这也是个 bug。我们需要{++精确的为一个 allocate 配对一个 free++}。

Rust 采取了一个不同的策略：{==内存在拥有它的变量离开作用域后就被自动释放==}。下面是作用域例子的一个使用 **String** 而不是字符串字面值的版本：

```rust
{
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
    println!("s is {s}")
}                               // 此作用域已结束，
                                // s 不再有效
```

当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 [drop](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop)，Rust 在结尾的 `}` 处自动调用 `drop`。

##### 数据交互：移动

多个变量可以采取不同的方式与同一数据进行交互。

```rust
fn main() {
    let x = 5;
    let y = x;  // 将变量 x 的整数值赋给 y
}
```

> “将 5 绑定到 x；接着生成一个值 x 的拷贝并绑定到 y”。
>
> 现在有了两个变量，x 和 y，都等于 5。这也正是事实上发生了的，因为整数是有**已知固定大小**的简单值，所以这**两个 5 被放入了栈中**。

**String** 版本：

```rust
let s1 = String::from("hello");
let s2 = s1;

    // 当我们将 s1 赋值给 s2，String 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。
    // Rust并没有复制指针指向的堆上数据。

    // 如果 Rust 也拷贝了堆上的数据，那么操作 s2 = s1 在堆上数据比较大的时候会对运行时性能造成非常大的影响。
```

过当变量离开作用域后，Rust 自动调用 drop 函数并清理变量的堆内存。不过两个数据指针指向了同一位置。
这就有了一个问题：当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。
这是一个叫做 {++二次释放++}（double free）的错误，也是之前提到过的内存安全性 bug 之一。**两次释放**（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。

{++为了确保内存安全，在 `let s2 = s1` 之后，Rust 认为 s1 不再有效，因此 Rust 不需要在 s1 离开作用域后清理任何东西。++}

在 s2 被创建之后尝试使用 s1 会发生什么；这段代码不能运行：

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
// 会得到一个错误: value borrowed here after move
// 因为 Rust 禁止使用无效的引用。
```

如果在其他语言中听说过术语 **浅拷贝**（shallow copy）和 **深拷贝**（deep copy），那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。不过因为 Rust 同时使第一个变量无效了，这个操作被称为 {++移动++}（move），而不是浅拷贝。上面的例子可以解读为 s1 被 {++移动++} 到了 s2 中。

这样就解决了二次释放的问题！因为只有 s2 是有效的，当其离开作用域，它就释放了自己的内存。

> 这里还隐含了一个设计选择：**Rust 永远也不会自动创建数据的 “深拷贝”**。因此，任何 {++自动++} 的复制可以被认为对运行时性能影响较小。

##### 数据交互：克隆

如果 **确实** 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

这段代码能正常运行，这里堆上的数据 **确实** 被复制了。

当出现 `clone` 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源。

##### 拷贝

这里还有一个没有提到的小窍门。这些代码使用了整型并且是有效的!

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

但这段代码似乎与刚刚学到的内容相矛盾：没有调用 clone，不过 x 依然有效且没有被移动到 y 中。

原因是像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。这意味着没有理由在创建变量 y 后使 x 无效。换句话说，**这里没有深浅拷贝的区别**，所以这里调用 `clone` 并不会与通常的浅拷贝有什么不同，我们可以不用管它。

Rust 有一个叫做 `Copy trait` 的特殊注解，可以用在类似整型这样的存储在栈上的类型上（[第十章](https://kaisery.github.io/trpl-zh-cn/ch10-00-generics.html)将会详细讲解 trait）。如果一个类型实现了 `Copy trait`，那么一个旧的变量在将其赋值给其他变量后仍然可用。

{++Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait++}。如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。要学习如何为你的类型添加 Copy 注解以实现该 trait，请阅读[附录 C 中的 “可派生的 trait”](https://kaisery.github.io/trpl-zh-cn/appendix-03-derivable-traits.html)。

那么哪些类型实现了 `Copy` trait 呢？可以查看给定类型的文档来确认，不过作为一个通用的规则，任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 `Copy` 。如下是一些 `Copy` 的类型：

- 所有整数类型，比如 u32。
- 布尔类型，bool，它的值是 true 和 false。
- 所有浮点数类型，比如 f64。
- 字符类型，char。
- 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。

### 所有权与函数

**将值传递给函数与给变量赋值的原理相似。向函数传递值可能会移动或者复制，就像赋值语句一样。**

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
```

当尝试在调用 takes_ownership 后使用 s 时，Rust 会抛出一个编译时错误。这些静态检查使我们免于犯错。

试试在 main 函数中添加使用 s 和 x 的代码来看看哪里能使用他们，以及所有权规则会在哪里阻止我们这么做。

### 返回值与作用域

返回值也可以转移所有权。

```rust
ffn main() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 转移给 s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中,
                                       // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域.

    some_string // 返回 some_string
                // 并移出给调用的函数
                //
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    //

    a_string // 返回 a_string 并移出给调用的函数
}
```

变量的所有权总是遵循相同的模式：{++将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。++}

虽然这样是可以的，但是在每一个函数中都获取所有权并接着返回所有权有些啰嗦。如果我们想要函数使用一个值但不获取所有权该怎么办呢？如果我们还要接着使用它的话，每次都传进去再返回来就有点烦人了，除此之外，我们也可能想返回函数体中产生的一些数据。

我们可以使用元组来返回多个值，示例：

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
```

但是这未免有些形式主义，而且这种场景应该很常见。幸运的是，Rust 对此提供了一个不用获取所有权就可以使用值的功能，叫做 **引用**（references）。

## 引用与借用

以提供一个 String 值的引用（reference）。**引用**（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。 与指针不同，{++引用确保指向某个特定类型的有效值++}。

```rust
fn main() {
    let s1 = String::from("hello");

    // 传递 &s1 给 calculate_length
    // 这些 & 符号就是 引用，它们允许你使用值但不获取其所有权。
    // &s1 语法让我们创建一个 指向 值 s1 的引用，但是并不拥有它。
    // 因为并不拥有这个值，所以当引用停止使用时，它所指向的值也不会被丢弃。
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// 函数签名使用 & 来表明参数 s 的类型是一个引用。
fn calculate_length(s: &String) -> usize {  // s是String的引用
    s.len()
}   // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
    // 所以什么也不会发生 (Rust 不会调用drop)
```

**当函数使用引用而不是实际值作为参数，无需返回值来交还所有权，因为就不曾拥有所有权。**

将创建一个引用的行为称为 **借用**（borrowing）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。

{++正如变量默认是不可变的，引用也一样。（默认）不允许修改**引用**的值。++}

### 可变引用

允许我们修改一个借用的值，这就是 {++可变引用++}（mutable reference）

```rust
fn main() {
    let mut s = String::from("hello");  // 必须将 s 改为 mut， 声明可变

    change(&mut s);     // 传入可变引用
}

// 声明可变引用
fn change(some_string: &mut String) {
    some_string.push_str(", world");  // 因为可变，所以可以更改。
}
```

可变引用有一个很大的限制：{++如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用++}。这些尝试创建两个 s 的可变引用的代码会失败：

```rust
let mut s = String::from("hello");

let r1 = &mut s;    // 有一个了
let r2 = &mut s;    // 第二个将会报错。

println!("{}, {}", r1, r2);
```

报错如下:

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here
6 | 
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here

For more information about this error, try `rustc --explain E0499`.
error: could not compile `ownership` due to previous error
```

这一限制以一种非常小心谨慎的方式允许可变性，{++防止同一时间对同一数据存在多个可变引用++}。新 Rustacean 们经常难以适应这一点，因为大部分语言中变量任何时候都是可变的。这个限制的好处是 Rust 可以在编译时就避免数据竞争。**数据竞争**（data race）类似于竞态条件，它可由这三个行为造成：

- **两个或更多指针同时访问同一数据。**
- **至少有一个指针被用来写入数据。**
- **没有同步数据访问的机制。**

数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！

一如既往，可以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 **同时** 拥有：

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}
```

Rust 在同时使用可变与不可变引用时也采用的类似的规则。这些代码会导致一个错误：

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
let r3 = &mut s; // 大问题

println!("{}, {}, and {}", r1, r2, r3);  // 使用不可变引用时，同时又使用了可变引用。
```

错误如下：

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

哇哦！我们 {++**也** 不能在拥有不可变引用的同时拥有可变引用。++}

注意{++一个引用的作用域从声明的地方开始一直持续到最后一次使用为止++}。例如，因为最后一次使用**不可变引用**（println!)，发生在声明可变引用之前，所以如下代码是可以编译的：

```rust
let mut s = String::from("hello");

let r1 = &s; // 没问题
let r2 = &s; // 没问题
println!("{} and {}", r1, r2);
// 此位置之后 r1 和 r2 不再使用

let r3 = &mut s; // 没问题
println!("{}", r3);
```

不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。它们的作用域**没有重叠**，所以代码是可以编译的。
编译器在作用域结束之前判断不再使用的引用的能力被称为 {++非词法作用域生命周期++}（Non-Lexical Lifetimes，简称 NLL）。你可以在 [The Edition Guide](https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html) 中阅读更多关于它的信息。

### 悬垂引用（Dangling References）

所谓**悬垂指针**是其指向的内存可能已经被分配给其它持有者。相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：{++当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。++}

当尝试创建一个悬垂引用时，Rust 会通过一个编译时错误来避免：

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle 返回一个字符串的引用

    let s = String::from("hello"); // s 是一个新字符串

    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！
```

错误:

```shell
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> src/main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                ~~~~~~~~

For more information about this error, try `rustc --explain E0106`.
error: could not compile `ownership` due to previous error
```

因为 s 是在 dangle 函数内创建的，当 dangle 的代码执行完毕后，s 将被释放。不过我们尝试返回它的引用。这意味着这个引用会指向一个无效的 String，这可不对！Rust 不会允许这么做。

这里的解决方法是直接返回 String：

```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s   // 直接返回s
        // 所有权被移动出去，所以没有值被释放。
}
```

### 引用的规则

- **在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。**
- **引用必须总是有效的。**

## Slice类型

slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。{++slice 是一类引用，所以它没有所有权。++}

小练习： 编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词, 如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

```rust
fn first_word(s: &String) -> ?
```

`first_word` 函数有一个参数 `&String`。因为我们不需要所有权，所以这没有问题。不过应该返回什么呢？我们并没有一个真正获取 **部分** 字符串的办法。不过，我们可以返回单词结尾的索引，结尾由一个空格表示。

```rust
fn first_word(s: &String) -> usize {
    // 用 as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();

    // 使用 iter 方法在字节数组上创建一个迭代器：
    // enumerate 包装了 iter 的结果，将这些元素作为元组的一部分来返回。
    // enumerate 返回的元组中，第一个元素是索引，第二个元素是集合中元素的引用。
    // 因为 enumerate 方法返回一个元组，我们可以使用模式来解构
    // 元组中的 i 是索引而元组中的 &item 是单个字节。
    // 因为我们从 .iter().enumerate() 中获取了集合元素的引用，所以模式中使用了 &。
    for (i, &item) in bytes.iter().enumerate() {

        // 通过字节的字面值语法来寻找代表空格的字节。
        // 如果找到了一个空格，返回它的位置。
        if item == b' ' {
            return i;
        }
    }

    // 否则，使用 s.len() 返回字符串的长度：
    s.len()
}
```

现在有了一个找到字符串中第一个单词结尾索引的方法，不过这有一个问题。
我们返回了一个独立的 `usize`，不过它只在 `&String` 的上下文中才是一个有意义的数字。
换句话说，因为它是一个与 `String` 相分离的值，无法保证将来它仍然有效。

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}
```

这个程序编译时没有任何错误，而且在调用 `s.clear()` 之后使用 word 也不会出错。因为 word 与 s 状态完全没有联系，所以 word 仍然包含值 5。
可以尝试用值 5 来提取变量 s 的第一个单词，不过这是有 bug 的，因为在我们将 5 保存到 word 之后 s 的内容已经改变。

我们不得不时刻担心 word 的索引与 s 中的数据不再同步，这很啰嗦且易出错！如果编写这么一个 second_word 函数的话，管理索引这件事将更加容易出问题。它的签名看起来像这样：

```rust
fn second_word(s: &String) -> (usize, usize) {
```

现在我们要跟踪一个**开始索引** 和 一个**结尾索引**，同时有了更多从数据的某个特定状态计算而来的值，但都完全没有与这个状态相关联。现在有三个飘忽不定的不相关变量需要保持同步。

### 字符串Slice

**字符串 slice**（string slice）是 String 中一部分值的引用，它看起来像这样：

```rust
let s = String::from("hello world");

let hello = &s[0..5];       // hello 是一个部分 String 的引用，由一个额外的 [0..5] 部分指定。
let world = &s[6..11];      // 
```

**可以使用一个由中括号中的 \[starting_index..ending_index\] 指定的 range 创建一个 slice，其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。**

在其内部，{++slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 ending_index 减去 starting_index 的值。++} 所以对于 `let world = &s[6..11];` 的情况，world 将是一个包含指向 s 索引 6 的指针和长度值 5 的 slice。

对于 Rust 的 `..` range 语法，如果想要从索引 0 开始，可以{++不写两个点号之前的值++}。如下两个语句是相同的：

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
```

依此类推，如果 slice 包含 String 的最后一个字节，也可以{++舍弃尾部++}的数字。这意味着如下也是相同的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
let slice = &s[3..];
```

也可以{++同时舍弃++}这两个值来获取整个字符串的 slice。所以如下亦是相同的：

```rust
let s = String::from("hello");

let len = s.len();

let slice = &s[0..len];
let slice = &s[..];
```

> **注意**
>
> 字符串 **slice** range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。
> 出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；第八章的 “[使用字符串存储 UTF-8 编码的文本](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html#%E4%BD%BF%E7%94%A8%E5%AD%97%E7%AC%A6%E4%B8%B2%E5%AD%98%E5%82%A8-utf-8-%E7%BC%96%E7%A0%81%E7%9A%84%E6%96%87%E6%9C%AC)” 部分会更加全面的讨论 UTF-8 处理问题。

重写 **first_word** 来返回一个 `slice`。**“字符串 slice”** 的类型声明写作 `&str`：

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

现在当调用 `first_word` 时，会返回与底层数据关联的单个值。这个值由一个 **slice 开始位置的引用和 slice 中元素的数量组成**。

`second_word` 函数也可以改为返回一个 `slice`：

```rust
fn second_word(s: &String) -> &str {
```

现在有了一个不易混淆且直观的 API 了，{++因为编译器会确保指向 String 的引用持续有效。++}

使用 slice 版本的 `first_word` 会抛出一个编译时错误：

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 错误!

    println!("the first word is: {}", word);
}
```

编译错误：

```rust
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:18:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 | 
18 |     s.clear(); // error!
   |     ^^^^^^^^^ mutable borrow occurs here
19 | 
20 |     println!("the first word is: {}", word);
   |                                       ---- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `ownership` due to previous error
```

回忆一下**借用规则**，当拥有某值的不可变引用时，就不能再获取一个可变引用。因为 clear 需要清空 String，它尝试获取一个可变引用。
在调用 clear 之后的 `println!` 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。
{++Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。++}
Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！

### 字符串字面值就是 slice

还记得讲到过字符串字面值被储存在二进制文件中吗？现在知道 slice 了，我们就可以正确地理解字符串字面值了：

```rust
let s = "Hello, world!";
```

这里 s 的类型是 `&str`：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；**&str 是一个不可变引用**。

### 字符串 slice 作为参数

在知道了能够获取字面值和 String 的 slice 后，我们对 first_word 做了改进，这是它的签名：

```rust
fn first_word(s: &String) -> &str {
```

而更有经验的 Rustacean 会编写出这样的函数签名，因为它使得可以对 `&String` 值和 `&str` 值使用相同的函数：

```rust
fn first_word(s: &str) -> &str {
```

如果有一个字符串 slice，可以直接传递它。如果有一个 **String**，则可以传递整个 **String** 的 `slice` 或对 **String** 的引用。
这种灵活性利用了 ***deref coercions*** 的优势，这个特性我们将在“[函数和方法的隐式 Deref 强制转换](https://kaisery.github.io/trpl-zh-cn/ch15-02-deref.html#%E5%87%BD%E6%95%B0%E5%92%8C%E6%96%B9%E6%B3%95%E7%9A%84%E9%9A%90%E5%BC%8F-deref-%E5%BC%BA%E5%88%B6%E8%BD%AC%E6%8D%A2)”章节中介绍。
定义一个获取字符串 slice 而不是 String 引用的函数使得我们的 API **更加通用**并且不会丢失任何功能：

```rust
fn main() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 是 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}
```

### 其他类型的 slice

字符串 slice，正如你想象的那样，是针对字符串的。不过也有更通用的 slice 类型。考虑一下这个数组：

```rust
let a = [1, 2, 3, 4, 5];
```

就跟想要获取字符串的一部分那样，也会想要引用数组的一部分。可以这样做：

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];  // 这个 slice 的类型是 &[i32]。

assert_eq!(slice, &[2, 3]);
```

它跟字符串 slice 的工作方式一样，通过{++存储第一个集合元素的引用和一个集合总长度++}。你可以对其他所有集合使用这类 slice。第八章讲到 vector 时会详细讨论这些集合。

## 总结

**所有权**、**借用**和 **slice** 这些概念让 Rust 程序在编译时确保内存安全。
Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。