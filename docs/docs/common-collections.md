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

## HashMap 键值对
