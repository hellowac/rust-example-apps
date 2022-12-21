# 常见集合

Rust 标准库中包含一系列被称为 **集合**（collections）的非常有用的数据结构。
大部分其他数据类型都代表一个特定的值，不过集合可以包含多个值。
{++不同于内建的数组和元组类型，这些集合指向的数据是储存在堆上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小。++}
每种集合都有着不同功能和成本，而根据当前情况选择合适的集合，这是一项应当逐渐掌握的技能。

- **vector** 允许我们一个挨着一个地储存一系列数量可变的值
- **字符串**（string）是字符的集合。之前见过 **String** 类型。
- **哈希 map**（hash map）允许我们将值与一个特定的键（key）相关联。这是一个叫做 **map** 的更通用的数据结构的特定实现。

对于标准库提供的其他类型的集合，请查看[文档](https://www.rustwiki.org.cn/zh-CN/std/collections/index.html)。

## Vector 列表

`Vec<T>`，也被称为 **vector**。 **vector** 允许在一个单独的数据结构中储存多于一个的值，它在内存中彼此相邻地排列所有的值。
**vector** 只能储存相同类型的值。它们在拥有**一系列项**的场景下非常实用，例如文件中的文本行或是购物车中商品的价格。

### 新建 vector

```rust
// 创建一个新的空 vector，可以调用 Vec::new 函数
let v: Vec<i32> = Vec::new();
// 告诉 Rust v 这个 Vec<T> 将存放 i32 类型的元素。
// 注意增加了一个类型注解, 因为没有向这个 vector 中插入任何值，Rust 并不知道我们想要储存什么类型的元素。
// vector 是用泛型实现的，第十章会涉及到如何对你自己的类型使用它们。
```

现在，所需要知道的就是 `Vec<T>` 是一个由标准库提供的类型，它可以存放任何类型，而当 `Vec` 存放某个特定类型时，那个类型位于尖括号中。

通常，会用初始值来创建一个 `Vec<T>` 而 Rust 会推断出储存值的类型，所以很少会需要这些类型注解。

为了方便 Rust 提供了 `vec!` 宏，这个宏会根据我们提供的值来创建一个新的 `vector`。

```rust
// 新建一个拥有值 1、2 和 3 的 Vec<i32>
let v = vec![1, 2, 3];
```

因为提供了 `i32` 类型的初始值，Rust 可以推断出 `v` 的类型是 `Vec<i32>`，因此类型注解就不是必须的。

### 更新 vector

使用 `push` 方法可以向其增加元素：

```rust
// 如果想要能够改变它的值，必须使用 mut 关键字使其可变
let mut v = Vec::new();

v.push(5);  // 增加元素
v.push(6);  // 增加元素
v.push(7);
v.push(8);

// 放入其中的所有值都是 i32 类型的，而且 Rust 也根据数据做出如此判断，所以不需要 Vec<i32> 注解。
```

### 丢弃 vector

类似于任何其他的 `struct`，`vector` 在其离开作用域时会被释放

```rust
{
    let v = vec![1, 2, 3, 4];

    // 处理变量 v
} // <- 这里 v 离开作用域并被丢弃
```

{++当 `vector` 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。++}
这可能看起来非常直观，不过一旦开始使用 `vector` 元素的引用，情况就变得有些复杂了。

### 读取 vector

有两种方法引用 `vector` 中储存的值。 **索引语法**或者 `get` 方法

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引值 2 来获取第三个元素，索引是从 0 开始的。
    // 使用 & 和 [] 返回一个引用
    let third: &i32 = &v[2];
    println!("第三个数是 {}", third);

    // 使用 get 方法以索引作为参数来返回一个 Option<&T>。
    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("这儿没有元素"),
    }
}
```

{++Rust 提供了两种引用元素的方法的原因是当**尝试使用现有元素范围之外的索引值时可以选择让程序如何运行。**++}

```rust
// 尝试在当有一个 5 个元素的 vector 接着访问索引 100 位置的元素会发生什么
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];       // 当引用一个不存在的元素时 Rust 会造成 panic
let does_not_exist = v.get(100);    // 当 get 方法被传递了一个数组外的索引时，它不会 panic 而是返回 None。
// 当偶尔出现超过 vector 范围的访问属于正常情况的时候可以考虑使用它。
// 接着你的代码可以有处理 Some(&element) 或 None 的逻辑，
```

**一旦程序获取了一个有效的引用，借用检查器将会执行所有权和借用规则来确保 `vector` 内容的这个引用和任何其他引用保持有效。
回忆一下不能在相同作用域中同时存在可变和不可变引用的规则。**

这个规则适用于下面的示例：

```rust
// 在拥有 vector 中项的引用的同时向其增加一个元素

let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("第一个元素是: {}", first);

// 当我们获取了 vector 的第一个元素的不可变引用
// 并尝试在 vector 末尾增加一个元素的时候，如果尝试在函数的后面引用这个元素是行不通的
```

编译会给出这个错误

```rust
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 | 
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here

For more information about this error, try `rustc --explain E0502`.
error: could not compile `collections` due to previous error
```

{==

> **为什么第一个元素的引用会关心 vector 结尾的变化？**
>
> 不能这么做的原因是由于 `vector` 的工作方式：
> 在 `vector` 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。
> 这时，**第一个元素的引用就指向了被释放的内存。**借用规则阻止程序陷入这种状况。
>
> 关于 `Vec<T>` 类型的更多实现细节，请查看 “[The Rust onomicon](https://doc.rust-lang.org/nomicon/vec/vec.html)”

==}

### 遍历 vector

如果想要依次访问 `vector` 中的每一个元素，可以遍历其所有的元素而无需通过索引一次一个的访问。

```rust
// 通过 for 循环遍历 vector 的元素并打印

let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

```rust
// 也可以遍历可变 vector 的每一个元素的可变引用以便能改变他们
let mut v = vec![100, 32, 57];
for i in &mut v {
    // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
    *i += 50;
}
```

### 枚举来储存多种类型

`vector` 只能储存相同类型的值。这是很不方便的；**绝对会有需要储存一系列不同类型的值的用例**。
幸运的是，枚举的成员都被定义为相同的枚举类型，所以当需要在 `vector` 中储存不同类型值时，我们可以定义并使用一个枚举！

例如，假如我们想要从电子表格的一行中获取值，而这一行的有些列包含数字，有些包含浮点值，还有些是字符串。
我们可以定义一个枚举，其成员会存放这些不同类型的值，同时所有这些枚举成员都会被当作相同类型，那个枚举的类型。
接着可以创建一个储存枚举值的 vector，这样最终就能够储存不同类型的值了。

```rust
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
```

{==

**Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。**
第二个好处是可以准确的知道这个 `vector` 中允许什么类型。
如果 Rust 允许 `vector` 存放任意类型，那么当对 `vector` 元素执行操作时一个或多个类型的值就有可能会造成错误。
使用枚举外加 `match` 意味着 Rust 能在编译时就保证总是会处理所有可能的情况.

==}

如果在编写程序时不能确切无遗地知道运行时会储存进 `vector` 的所有类型，枚举技术就行不通了。相反，你可以使用 trait 对象，第十七章会讲到它。

现在我们了解了一些使用 vector 的最常见的方式，请一定去看看标准库中 `Vec` 定义的很多其他实用方法的 [API 文档](https://www.rustwiki.org.cn/zh-CN/std/vec/struct.Vec.html)。
例如，除了 `push` 之外还有一个 `pop` 方法，它会移除并返回 `vector` 的最后一个元素。

## 字符串和UTF-8

**字符串**是新晋 Rustacean 们通常会被困住的领域，这是由于三方面理由的结合：**Rust 倾向于确保暴露出可能的错误，字符串是比很多程序员所想象的要更为复杂的数据结构，以及 UTF-8**。所有这些要素结合起来对于来自其他语言背景的程序员就可能显得很困难了。

**在集合章节中讨论字符串的原因是，字符串就是作为字节的集合外加一些方法实现的，当这些字节被解释为文本时，这些方法提供了实用的功能。**

### 什么是字符串？

**字符串**。Rust 的核心语言中只有一种字符串类型：**字符串slice** `str`，它通常以被**借用**的形式出现，`&str`。第四章讲到了 **字符串 slices**：{++它们是一些对储存在别处的 UTF-8 编码字符串数据的引用++}。
举例来说，由于字符串**字面值**被储存在程序的二进制输出中，因此字符串字面值也是**字符串slices**。

称作 **String** 的类型是由标准库提供的，而没有写进核心语言部分，它是**可增长的**、**可变的**、**有所有权的**、**UTF-8 编码的字符串类型**。
当 Rustacean 们谈到 Rust 的 “**字符串**”时，它们通常指的是 **String** 或**字符串slice &str 类型**，而不特指其中某一个。
虽然本部分内容大多是关于 **String** 的，不过这两个类型在 Rust 标准库中都被广泛使用，**String** 和**字符串 slices** 都是 UTF-8 编码的。

### 新建字符串

```rust
// 以 new 函数创建字符串开始

// 新建一个空的 String
let mut s = String::new();
```

通常字符串会有初始数据，因为希望一开始就有这个字符串。为此，可以使用 `to_string` 方法，它能用于任何实现了 `Display` trait 的类型，字符串字面值也实现了它。

```rust
let data = "initial contents";

let s = data.to_string();

// 该方法也可直接用于字符串字面值：
// 会创建包含 initial contents 的字符串。
let s = "initial contents".to_string();

// 也可以使用 String::from 函数来从字符串字面值创建 String
let s = String::from("initial contents");
```

因为字符串应用广泛，这里有很多不同的用于字符串的通用 API 可供选择。其中一些可能看起来多余，不过都有其用武之地！在这个例子中，`String::from` 和 `.to_string` 最终做了完全相同的工作，所以如何选择就是**代码风格与可读性**的问题了。

```rust
// 字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
let hello = String::from("السلام عليكم");
let hello = String::from("Dobrý den");
let hello = String::from("Hello");
let hello = String::from("שָׁלוֹם");
let hello = String::from("नमस्ते");
let hello = String::from("こんにちは");
let hello = String::from("안녕하세요");
let hello = String::from("你好");
let hello = String::from("Olá");
let hello = String::from("Здравствуйте");
let hello = String::from("Hola");
// 所有这些都是有效的 String 值
```

### 更新字符串

**String** 的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变 **Vec** 的内容一样。
另外，可以方便的使用 `+` 运算符或 `format!` 宏来拼接 **String** 值。

#### push_str 和 push

```rust
// 通过 push_str 方法来附加字符串 slice，从而使 String 变长
let mut s = String::from("foo");
s.push_str("bar");
```

```rust
// s 将会包含 foobar。push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权。
// 将字符串 slice 的内容附加到 String 后使用它
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);   // 如果 push_str 方法获取了 s2 的所有权，就不能在最后一行打印出其值了
println!("s2 is {}", s2);


// push 方法被定义为获取一个单独的字符作为参数，并附加到 String 中
// 使用 push 将一个字符加入 String 值中
let mut s = String::from("lo"); // 添加mut关键字使其可变。
s.push('l');
```

#### + 运算符或 format! 宏

通常会希望将两个已知的字符串合并在一起。一种办法是像这样使用 `+` 运算符

```rust
// 使用 + 运算符将两个 String 值合并到一个新的 String 值中
fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    // 使用了 &，意味着我们使用第二个字符串的 引用 与第一个字符串相加。
    // 因为 add 函数的 s 参数：只能将 &str 和 String 相加，不能将两个 String 值相加。

    // println!("a is {s1}"); // 取消注释，会编译不通过，s1已经移动了，不可使用
    println!("c is {s3}");  // s3 则没问题
}
```

`s1` 在相加后不再有效的原因，和使用 `s2` 的引用的原因，与使用 `+` 运算符时调用的函数签名有关。`+` 运算符使用了 `add` 函数，这个函数签名看起来像这样：

```rust
fn add(self, s: &str) -> String {

// 这并不是标准库中实际的签名；标准库中的 add 使用泛型定义。
```

这里我们看到的 `add` 的签名使用具体类型代替了泛型，这也正是当使用 **String** 值调用这个方法会发生的。第十章会讨论泛型。

{==

> 正如 **add** 的第二个参数所指定的，**&s2** 的类型是 **&String** 而不是 **&str**。那么为什么示例还能编译呢？
>
> 之所以能够在 `add` 调用中使用 `&s2` 是因为 `&String` 可以被 **强转**（coerced）成 `&str`。当`add`函数被调用时，Rust 使用了一个被称为 `Deref` **强制转换**（deref coercion）的技术，
> 可以将其理解为它把 `&s2` 变成了 `&s2[..]`。
> [第十五章](https://kaisery.github.io/trpl-zh-cn/ch15-00-smart-pointers.html)会更深入的讨论 `Deref` 强制转换。
> 因为 `add` 没有获取参数的所有权，所以 `s2` 在这个操作后仍然是有效的 **String**。
>
> 其次，可以发现签名中 `add` 获取了 `self` 的所有权，因为 `self` **没有** 使用 `&`。这意味着示例中的 `s1` 的所有权将被移动到 `add` 调用中，之后就不再有效。
> 所以虽然 `let s3 = s1 + &s2;` 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 `s1` 的所有权，附加上从 `s2` 中拷贝的内容，**并返回结果的所有权**。
> 换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：**这个实现比拷贝要更高效**。

==}

```rust
// 级联多个字符串，+ 的行为就显得笨重了
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;
// 这时 s 的内容会是 “tic-tac-toe”

// 对于更为复杂的字符串链接，可以使用 format! 宏
let s = format!("{}-{}-{}", s1, s2, s3);
```

`format!` 与 `println!` 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的 `String`。这个版本就好理解的多，宏 `format!` 生成的代码使用引用所以不会获取任何参数的所有权。

### 索引字符串

**Rust 的字符串不支持索引。** 如下使用索引访问字符串会报编译错误：

```rust
let s1 = String::from("hello");
let h = s1[0];      // 尝试对字符串使用索引语法
```

会导致如下错误：

```rust
$ cargo run
   Compiling collections v0.1.0 (file:///projects/collections)
error[E0277]: the type `String` cannot be indexed by `{integer}`
 --> src/main.rs:3:13
  |
3 |     let h = s1[0];
  |             ^^^^^ `String` cannot be indexed by `{integer}`
  |
  = help: the trait `Index<{integer}>` is not implemented for `String`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `collections` due to previous error
```

为什么不支持呢？那么就要回答**Rust是如何在内存中储存字符串的**。看下面👇🏻

#### 内部表现

**String 是一个 `Vec<u8>` 的封装。**

```rust
let hello = String::from("Hola");
// 在这里，len 的值是 4 ，
// 这意味着储存字符串 “Hola” 的 Vec 的长度是四个字节：
// 这里每一个字母的 UTF-8 编码都占用一个字节。
```

这个呢?

```rust
// 字符串中的首字母是西里尔字母的 Ze 而不是阿拉伯数字 3 
let hello = String::from("Здравствуйте");

// 当问及这个字符是多长的时候有人可能会说是 12。
// 然而，Rust 的回答是 24。
```

这是使用 UTF-8 编码 `“Здравствуйте”` 所需要的字节数，这是因为每个 **Unicode** 标量值需要**两个字节**存储。
因此一个字符串字节值的索引并不总是对应一个有效的 **Unicode** 标量值。

作为演示，考虑如下无效的 Rust 代码：

```rust
let hello = "Здравствуйте";
let answer = &hello[0];
// 已经知道 answer 不是第一个字符 З。
```

当使用 **UTF-8** 编码时，`З` 的第一个字节 `208`，第二个是 `151`，所以 `answer` 实际上应该是 `208`，不过 `208` 自身并不是一个有效的字母。
返回 `208` 可不是一个请求字符串第一个字母的人所希望看到的，不过它是 Rust 在字节索引 `0` 位置所能提供的唯一数据。

用户通常不会想要一个字节值被返回，即便这个字符串只有拉丁字母： 即便 `&"hello"[0]` 是返回字节值的有效代码，它也应当返回 `104` 而不是 `h`。

**为了避免返回意外的值并造成不能立刻发现的 bug，Rust 根本不会编译这些代码，并在开发过程中及早杜绝了误会的发生。**

#### 字节、标量值和字形簇

从 Rust 的角度来讲，事实上有三种相关方式可以理解字符串：**字节**、**标量值**和**字形簇**（最接近人们眼中 **字母** 的概念）。

比如这个用梵文书写的印度语单词 `“नमस्ते”`，最终它储存在 **vector** 中的 `u8` 值看起来像这样：

```rust
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

这里有 18 个字节，也就是计算机最终会储存的数据。如果从 **Unicode** 标量值的角度理解它们，也就像 **Rust** 的 **char** 类型那样，这些字节看起来像这样：

```rust
['न', 'म', 'स', '्', 'त', 'े']
// 这里有六个 char，不过第四个和第六个都不是字母，它们是发音符号本身并没有任何意义。
// 最后，如果以字形簇的角度理解，就会得到人们所说的构成这个单词的四个字母：
["न", "म", "स्", "ते"]
```

Rust 提供了多种不同的方式来解释计算机储存的原始字符串数据，这样程序就可以选择它需要的表现方式，而无所谓是何种人类语言。

{==

最后一个 Rust 不允许使用索引获取 String 字符的原因是，**索引操作预期总是需要常数时间 (O(1))**。但是对于 `String` 不可能保证这样的性能，**因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。**

==}

### 字符串 slice

索引字符串通常是一个坏点子，{++因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice。++}

为了更明确索引并表明你需要一个字符串 `slice`，相比使用 `[]` 和**单个值**的索引，可以使用 `[]` 和一个 `range` 来创建含特定字节的字符串 `slice`：

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];

// s 会是一个 &str，它包含字符串的头四个字节。
// 早些时候，提到了这些字母都是两个字节长的，
// 所以这意味着 s 将会是 “Зд”。
```

> 如果获取 `&hello[0..1]` 会发生什么呢？
>
> 答案是：Rust 在运行时会 `panic`，就跟访问 `vector` 中的无效索引时一样：
>
> ```rust
> $ cargo run
>    Compiling collections v0.1.0 (file:///projects/collections)
>     Finished dev [unoptimized + debuginfo] target(s) in 0.43s
>      Running `target/debug/collections`
> thread 'main' panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`', src/main.rs:4:14
> note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
> ```
>
> **应该小心谨慎的使用这个操作，因为这么做可能会使你的程序崩溃。**

### 遍历字符串

{++操作字符串每一部分的最好的方法是明确表示需要**字符**还是**字节**++}。对于单独的 **Unicode** 标量值使用 `chars` 方法。

对 **“नमस्ते”** 调用 `chars` 方法会将其分开并返回六个 `char` 类型的值，接着就可以遍历其结果来访问每一个元素了：

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

会打印出如下内容：

```rust
न
म
स
्
त
े
```

另外 `bytes` 方法返回每一个**原始字节**，这可能会适合你的使用场景：

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

会打印出组成 String 的 18 个字节：

```rust
224
164
// --snip--
165
135
```

{++请记住有效的 Unicode 标量值可能会由不止一个字节组成。++}

从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能。[crates.io](https://crates.io/) 上有些提供这样功能的 `crate`。

### 字符串并不简单

{==

总而言之，字符串还是很复杂的。**不同的语言选择了不同的向程序员展示其复杂性的方式**。
Rust 选择了以准确的方式处理 **String** 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更多的思考如何预先处理 `UTF-8` 数据。
这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，不过也使你在开发生命周期后期免于处理涉及非 `ASCII` 字符的错误。

==}

## HashMap 键值对

集合类型 **哈希 map**（hash map）。`HashMap<K, V>`类型储存了一个键类型 `K` 对应一个值类型 `V` 的映射。
它通过一个 `哈希函数`（hashing function）来实现映射，决定如何将键和值放入内存中。
很多编程语言支持这种数据结构，不过通常有不同的名字：`哈希`、`map`、`对象`、`哈希表`或者`关联数组`，

哈希 `map` 可以用于需要任何类型作为键来寻找数据的情况，而不是像 `vector` 那样通过索引。

### 新建哈希 map

```rust
// 用 new 创建一个空的 HashMap，并使用 insert 增加元素。
use std::collections::HashMap;
// 必须首先 use 标准库中集合部分的 HashMap
// HashMap 是最不常用的，所以并没有被 prelude 自动引用。
// 标准库中对 HashMap 的支持也相对较少，例如，并没有内建的构建宏。

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

像 `vector` 一样，哈希 `map` 将它们的数据储存在堆上，这个 `HashMap` 的键类型是 `String` 而值类型是 `i32`。
类似于 **vector**，**哈希 map** 是同质的：{++所有的键必须是相同类型，值也必须都是相同类型。++}

另一个构建哈希 map 的方法是在一个元组的 **vector** 上使用**迭代器**（iterator）和 **collect** 方法，其中每个元组包含一个键值对。

会在第十三章的 “[使用迭代器处理一系列元素](https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html)” 部分 **介绍迭代器及其关联方法**。
**collect** 方法可以将数据收集进一系列的集合类型，包括 **HashMap**。

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// 这里 HashMap<_, _> 类型注解是必要的，因为可能 collect 为很多不同的数据结构，而除非显式指定否则 Rust 无从得知你需要的类型。
// 但是对于键和值的类型参数来说，可以使用下划线占位，而 Rust 能够根据 vector 中数据的类型推断出 HashMap 所包含的类型。
// 键（key）类型是 String，
// 值（value）类型是 i32，
let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

### 哈希 map 和所有权

对于像 `i32` 这样的实现了 **Copy** trait 的类型，其值可以拷贝进**哈希 map**。对于像 **String** 这样拥有所有权的值，其值将被**移动**而**哈希 map** 会成为这些值的所有者，

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// 这里 field_name 和 field_value 不再有效，
// 尝试使用它们看看会出现什么编译错误！

// 当 insert 调用将 field_name 和 field_value 移动到哈希 map 中后，将不能使用这两个绑定。
```

{++如果将值的引用插入哈希 map，这些值本身将不会被移动进哈希 map。但是这些引用指向的值必须至少在哈希 map 有效时也是有效的。++}
第十章 “[生命周期与引用有效性](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html#%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F%E4%B8%8E%E5%BC%95%E7%94%A8%E6%9C%89%E6%95%88%E6%80%A7)” 部分将会更多的讨论这个问题。

### 访问哈希 map

可以通过 `get` 方法并提供对应的键来从哈希 map 中获取值

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);  // score 是与蓝队分数相关的值，应为 Some(10)。
// 因为 get 返回 Option<V>，所以结果被装进 Some；
// 如果某个键在哈希 map 中没有对应的值，get 会返回 None。
// 这时就要用某种第六章提到的方法之一来处理 Option。

// if let 方式👇🏻
if let Some(value) = score {
    println!("value is {value}")
} else {
    println!("not value")
}

// match 方式👇🏻
match score {
    Some(value) => {
        println!("value is {value}")
    }
    // None => {
    //     println!("not value")
    // }
    _ => println!("not value"),
}
```

可以使用与 vector 类似的方式来遍历哈希 map 中的每一个键值对，也就是 for 循环：

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 这会以任意顺序打印出每一个键值对：
// Blue:10
// Yellow:50
```

### 更新哈希 map

{==

**尽管键值对的数量是可以增长的，不过任何时候，每个键只能关联一个值。**
当我们想要改变哈希 map 中的数据时，必须决定如何处理一个键已经有值了的情况。

- 可以选择完全无视旧值并用新值代替旧值。
- 可以选择保留旧值而忽略新值，并只在键 **没有** 对应值时增加新值。
- 或者可以结合新旧两值。

==}

#### 覆盖一个值

**如果插入了一个键值对，接着用相同的键插入一个不同的值，与这个键相关联的旧值将被替换。**

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
// 这会打印出 {"Blue": 25}。原始的值 10 则被覆盖了。
```

#### 只在键没有对应值时插入

**经常会检查某个特定的键是否有值，如果没有就插入一个值。**
为此**哈希 map** 有一个特有的 API，叫做 `entry`，它获取我们想要检查的键作为参数。
`entry` 函数的返回值是一个枚举，`Entry`，它代表了可能存在也可能不存在的值。

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
// 会打印出 {"Yellow": 50, "Blue": 10}。
```

{==

**Entry** 的 `or_insert` 方法在键对应的值存在时就返回这个值的可变引用，如果不存在则将参数作为新值插入并返回新值的可变引用。

这比编写自己的逻辑要简明的多，另外也与借用检查器结合得更好。

==}

#### 根据旧值更新一个值

**另一个常见的哈希 map 的应用场景是找到一个键对应的值并根据旧的值更新它。**

示例中的代码计数一些文本中每一个单词分别出现了多少次。

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

// split_whitespace 方法会迭代 text 的值由空格分隔的子 slice.
for word in text.split_whitespace() {
    // or_insert 方法返回这个键的值的一个可变引用（&mut V）。
    let count = map.entry(word).or_insert(0);

    // 这里将这个可变引用储存在 count 变量中，所以为了赋值必须首先使用星号（*）解引用 count。
    *count += 1;

    // 这个可变引用在 for 循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
}

println!("{:?}", map);
// 会打印出 {"world": 2, "hello": 1, "wonderful": 1}。
```

### 哈希函数

HashMap 默认使用一种叫做 `SipHash` 的哈希函数，它可以抵御涉及**哈希表**（hash table）1 的拒绝服务（Denial of Service, DoS）攻击。
然而这并不是可用的最快的算法，不过为了更高的安全性值得付出一些性能的代价。
如果性能监测显示此哈希函数非常慢，以致于你无法接受，可以指定一个不同的 `hasher` 来切换为其它函数。
`hasher` 是一个实现了 **BuildHasher trait** 的类型。
第十章会讨论 `trait` 和如何实现它们。并不需要从头开始实现你自己的 `hasher`；
[crates.io](https://crates.io/) 有其他人分享的实现了许多常用哈希算法的 `hasher` 的库。

> 参考: <https://en.wikipedia.org/wiki/SipHash>

## 总结

`vector`、**字符串**和**哈希 map** 会在你的程序需要**储存**、**访问**和**修改数据**时帮助你。这里有一些你应该能够解决的练习问题：

- 给定一系列数字，使用 `vector` 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 `map` 会很有帮助）。
- 将字符串转换为 `Pig Latin`，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
- 使用`哈希 map` 和 `vector`，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

标准库 API 文档中描述的这些类型的方法将有助于你进行这些练习！

## 练习-我的答案

### 返回列表的中位数

```rust
fn main() {
    let mut numbers = vec![];

    // 生成100个随机数
    for _ in 0..100 {
        let rand_number = thread_rng().gen_range(0..100); // 随机数从0-100中取得。
        numbers.push(rand_number);
    }

    numbers.sort(); // 从小到大排序
    numbers.reverse(); // 逆序

    println!("生成的随机数列是: {:?}", &numbers);

    let mid_idx = numbers.len() / 2; // 找到中位数坐标

    println!("中位数是: {}", numbers[mid_idx]);  // 打印中位数

    let max_word = max_count_word(&numbers);   // 获取出现最多次数的随机数

    // 使用if let 判断并打印出现最多次数的随机数
    if let Some(value) = max_word {
        println!("众数是: {}", value);
    } else {
        println!("未发现众数!");
    }
}

fn max_count_word(vec: &Vec<i32>) -> Option<&i32> {
    let mut word_count = HashMap::new();

    // 统计每个随机数出现的次数
    for word in vec {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_count_key = None;

    // 找到出现最多次的随机数;
    for (k, v) in word_count {
        if v > max_count {
            max_count = v;
            max_count_key = Some(k);
        }
    }

    // 返回出现最多次的随机数
    max_count_key
}
```

### 字符串转换

```rust
fn main() {
    let vowel_chars = ['a', 'e', 'i', 'o', 'u'];

    // 从控制台获取单词
    let word = get_word_from_line();

    println!("获取的单词是: {}", &word);

    let mut fixed_word = String::from("");

    // 非元音开头单词的第一个字母
    let mut not_vowel_char: Option<char> = None;
    let mut other_chars = String::from("");

    for (idx, c) in word.chars().enumerate() {
        // 获取第一个字符并且不是元音字母开头的单词的第一个字母
        if idx == 0 && !vowel_chars.contains(&c) {
            not_vowel_char = Some(c);
            continue;
        } else {
            other_chars.push(c);
        };
    }

    // 非元音字母开头的情况
    if let Some(not_vowel_c) = not_vowel_char {
        fixed_word = format!("{}-{}ay", other_chars, not_vowel_c); // 使用format!宏格式化字符

    // 元音字母开头的情况
    } else {
        fixed_word = format!("{}-hay", other_chars);   // 使用format!宏格式化字符
    }

    println!("修改后的字符为: {}", fixed_word);
}

fn get_word_from_line() -> String {
    let mut word = String::new();

    println!("请输入一个单词:");

    loop {
        match io::stdin().read_line(&mut word) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }

    word = match word.trim().parse() {
        Ok(w) => w,
        Err(_) => String::from(""),
    };

    word
}
```

### 增加员工名字
