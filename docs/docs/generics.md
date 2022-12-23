# 泛型/Trait/生命周期

每一个编程语言都有高效处理重复概念的工具。在 Rust 中其工具之一就是 **泛型**（generics）。{++泛型是具体类型或其他属性的抽象替代。++}
我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相关联，而不需要在编写和编译代码时知道他们在这里实际上代表什么。

同理为了编写一份可以用于多种具体值的代码，函数并不知道其参数为何值，
这时就可以让函数获取泛型而不是像 `i32` 或 **String** 这样的具体类型。
我们已经使用过第六章的 **Option<T\>**，第八章的 **Vec<T\>** 和 **HashMap<K, V\>**，以及第九章的 **Result<T, E\>** 这些泛型了。

- 首先，我们将回顾一下**提取函数以减少代码重复的机制**。接下来，我们将使用相同的技术，从两个仅参数类型不同的函数中创建一个泛型函数。我们也会讲到**结构体**和**枚举**定义中的泛型。
- 之后，我们讨论 **trait**，这是一个{++定义泛型行为的方法。trait 可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型。++}
- 最后介绍 **生命周期**（lifetimes），它是一类允许我们{++向编译器提供**引用如何相互关联的泛型**++}。Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。

## 提炼函数减少重复

在介绍泛型语法之前，首先来回顾一个不使用泛型的处理重复的技术：**提取一个函数**。
当熟悉了这个技术以后，我们将使用相同的机制来提取一个泛型函数！如同你识别出可以提取到函数中重复代码那样，你也会开始识别出能够使用泛型的重复代码。

考虑一下这个寻找列表中最大值的小程序:

```rust
fn main() {
    // 码获取一个整型列表，存放在变量 number_list 中。
    let number_list = vec![34, 50, 25, 100, 65];

    // 将列表的第一项放入了变量 largest 中。
    let mut largest = number_list[0];

    // 接着遍历了列表中的所有数字，如果当前值大于 largest 中储存的值，将 largest 替换为这个值。
    // 如果当前值小于或者等于目前为止的最大值，largest 保持不变。
    // 当列表中所有值都被考虑到之后，largest 将会是最大值，在这里也就是 100。
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

如果需要在两个不同的列表中寻找最大值，我们可以重复示例中的代码，这样程序中就会存在两段相同逻辑的代码，

```rust
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}
```

{++虽然代码能够执行，但是重复的代码是冗余且容易出错的，并且意味着当更新逻辑时需要**修改多处地方的代码**。++}

为了消除重复，可以创建一层**抽象**，在这个例子中将表现为一个获取任意整型列表作为参数并对其进行处理的函数。
这将增加代码的简洁性并让我们将表达和推导寻找列表中最大值的这个概念与使用这个概念的特定位置相互独立。

在示例中的程序中将寻找最大值的代码提取到了一个叫做 `largest` 的函数中。这不同于上面示例中的代码只能在一个特定的列表中找到最大的数字，这个程序可以在两个不同的列表中找到最大的数字。

```rust
// largest 函数有一个参数 list，它代表会传递给函数的任何具体的 i32值的 slice。
// 函数定义中的 list 代表任何 &[i32]。
// 当调用 largest 函数时，其代码实际上运行于我们传递的特定值上。
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // 目前不需要担心 for 循环的语法。这里不是引用了一个 i32 的引用，这里只是模式匹配并表明循环的值应该是 &i32。
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
```

总的来说，在示例中涉及的机制经历了如下几步：

1. **找出重复代码。**
2. **将重复代码提取到了一个函数中，并在函数签名中指定了代码中的输入和返回值。**
3. **将重复代码的两个实例，改为调用函数。**

在不同的场景使用不同的方式，我们也可以利用相同的步骤和泛型来减少重复代码。与函数体可以在抽象`list`而不是特定值上操作的方式相同，泛型允许代码对抽象类型进行操作。

如果我们有两个函数，一个寻找一个 i32 值的 slice 中的最大项而另一个寻找 char 值的 slice 中的最大项该怎么办？该如何消除重复呢？看下去！

## 泛型

**可以使用泛型为像函数签名或结构体这样的项创建定义，这样它们就可以用于多种不同的具体数据类型。**

### 函数中使用泛型

当使用泛型定义函数时，本来在函数签名中指定参数和返回值的类型的地方，会改用**泛型**来表示。
采用这种技术，使得代码适应性更强，从而为函数的调用者提供更多的功能，同时也避免了代码的重复。

回到 `largest` 函数，示例中展示了两个函数，它们的功能都是寻找 `slice` 中最大值。

```rust
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}
```

因为两者函数体的代码是一样的，可以定义一个函数，再引进泛型参数来消除这种重复。

为了参数化新函数中的这些类型，也需要为类型参数取个名字，道理和给函数的形参起名一样。任何标识符都可以作为类型参数的名字。
这里选用 `T`，因为传统上来说，Rust 的参数名字都比较短，通常就只有一个字母，同时，**Rust 类型名的命名规范是骆驼命名法**（CamelCase）。`T` 作为 **“type”** 的缩写是大部分 Rust 程序员的首选。

如果要在函数体中使用参数，就必须在函数签名中声明它的名字，好让编译器知道这个名字指代的是什么。
同理，当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。为了定义泛型版本的 `largest` 函数，
类型参数声明位于函数名称与参数列表中间的尖括号 **<\>** 中，像这样：

```rust
fn largest<T>(list: &[T]) -> T {
```

可以这样理解这个定义：{++函数 `largest` 有泛型类型 **T**。它有个参数 `list`，其类型是元素为 **T** 的 `slice`。`largest` 函数的返回值类型也是 **T**。++}

下面示例 10-5 中的 `largest` 函数在它的签名中使用了泛型，统一了两个实现。
该示例也展示了如何调用 `largest` 函数，把 `i32` 值的 `slice` 或 `char` 值的 **slice** 传给它。
请注意这些代码还不能编译，不过稍后在本章会解决这个问题。

```rust
// 一个使用泛型参数的 largest 函数定义，尚不能编译
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

如果现在就编译这个代码，会出现如下错误：

```rus
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- T
  |            |
  |            T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` due to previous error
```

注释中提到了 `std::cmp::PartialOrd`，这是一个 `trait`。
下一部分会讲到 `trait`。不过简单来说，这个错误表明 `largest` 的函数体不能适用于 `T` 的所有可能的类型。因为在函数体需要比较 `T` 类型的值，不过它只能用于我们知道如何排序的类型。
为了开启比较功能，标准库中定义的 `std::cmp::PartialOrd` **trait** 可以实现类型的比较功能（查看[附录 C](https://kaisery.github.io/trpl-zh-cn/appendix-03-derivable-traits.html) 获取该 trait 的更多信息）。

标准库中定义的 `std::cmp::PartialOrd` **trait** 可以实现类型的比较功能。在 “[trait 作为参数](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html#trait-%E4%BD%9C%E4%B8%BA%E5%8F%82%E6%95%B0)” 部分会讲解如何指定泛型实现特定的 `trait`，不过让我们先探索其他使用泛型参数的方法。

### 结构体中使用泛型

同样也可以用 `<>`语法来定义结构体，它包含一个或多个泛型参数类型字段。下面示例展示了如何定义和使用一个可以存放任何类型的 `x` 和 `y` 坐标值的结构体 **Point**：

```rust
// Point 结构体存放了两个 T 类型的值 x 和 y
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

**其语法类似于函数定义中使用泛型。首先，必须在结构体名称后面的尖括号中声明泛型参数的名称。接着在结构体定义中可以指定具体数据类型的位置使用泛型类型。**

注意 **Point<T\>** 的定义中只使用了一个泛型类型，这个定义表明结构体 **Point<T\>** 对于一些类型 **T** 是泛型的，而且字段 `x` 和 `y` 都是 相同类型的，无论它具体是何类型。
如果尝试创建一个有不同类型值的 **Point<T\>** 的实例，就不能编译：

```rust
// 字段 x 和 y 的类型必须相同，因为他们都有相同的泛型类型 T
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let wont_work = Point { x: 5, y: 4.0 };
}
```

在这个例子中，当把整型值 `5` 赋值给 `x` 时，就告诉了编译器这个 **Point<T\>** 实例中的泛型 `T` 是整型的。接着指定 `y` 为 `4.0`，它被定义为与 `x` 相同类型，就会得到一个像这样的类型不匹配错误：

```rust
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0308]: mismatched types
 --> src/main.rs:7:38
  |
7 |     let wont_work = Point { x: 5, y: 4.0 };
  |                                      ^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `chapter10` due to previous error
```

如果想要定义一个 `x` 和 `y` 可以有不同类型且仍然是泛型的 **Point** 结构体，
我们可以使用多个泛型类型参数。在示例中，我们修改 **Point** 的定义为拥有两个泛型类型 `T` 和 `U`。
其中字段 `x` 是 `T` 类型的，而字段 `y` 是 `U` 类型的：

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

现在所有这些 **Point** 实例都合法了！
你可以在定义中使用任意多的泛型类型参数，**不过太多的话，代码将难以阅读和理解**。
{++当你的代码中需要许多泛型类型时，它可能表明你的代码需要重构，分解成更小的结构。++}

### 枚举中使用泛型

和结构体类似，**枚举也可以在成员中存放泛型数据类型**。第六章我们曾用过标准库提供的 **Option<T\>** 枚举，这里再回顾一下：

```rust
enum Option<T> {
    Some(T),
    None,
}
```

现在这个定义应该更容易理解了。如你所见 **Option<T\>** 是一个拥有泛型 **T** 的枚举，它有两个成员：**Some**，它存放了一个类型 **T** 的值，和不存在任何值的**None**。
通过 **Option<T\>** 枚举可以表达有一个可能的值的抽象概念，同时因为 **Option<T\>** 是泛型的，无论这个可能的值是什么类型都可以使用这个抽象。

枚举也可以拥有多个泛型类型。第九章使用过的 **Result** 枚举定义就是一个这样的例子：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Result 枚举有两个泛型类型，T 和 E。Result 有两个成员：Ok，它存放一个类型 T 的值，而 Err 则存放一个类型 E 的值。这个定义使得 Result 枚举能很方便的表达任何可能成功（返回 T 类型的值）也可能失败（返回 E 类型的值）的操作。实际上，这就是我们在示例 9-3 用来打开文件的方式：当成功打开文件的时候，T 对应的是 std::fs::File 类型；而当打开文件出现问题时，E 的值则是 std::io::Error 类型。

{++当你意识到代码中定义了多个结构体或枚举，它们不一样的地方只是其中的值的类型的时候，不妨通过泛型类型来避免重复。++}

### 方法中使用泛型

在为结构体和枚举实现方法时（像第五章那样），一样也可以用泛型。示例中定义的结构体 **Point<T\>**，和在其上实现的名为 `x` 的方法。

```rust
// 在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用

struct Point<T> {
    x: T,
    y: T,
}

// // 这里在 Point<T> 上定义了一个叫做 x 的方法来返回字段 x 中数据的引用：
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

注意必须在 `impl` 后面声明 **T**，这样就可以在 **Point<T\>** 上实现的方法中使用它了。
在 `impl` 之后声明泛型 **T** ，这样 **Rust** 就知道 **Point** 的尖括号中的类型是泛型而不是具体类型。
因为再次声明了泛型，我们可以为泛型参数选择一个与结构体定义中声明的泛型参数所不同的名称，不过依照惯例使用了相同的名称。
{++`impl` 中编写的方法声明了泛型类型可以定位为任何类型的实例，不管最终替换泛型类型的是何具体类型。 m++}

另一个选择是定义方法适用于某些有限制（constraint）的**泛型类型**。
例如，可以选择为 **Point<f32\>** 实例实现方法，而不是为泛型 **Point** 实例。
示例展示了一个没有在 `impl` 之后（的尖括号）声明泛型的例子，这里使用了一个具体类型，`f32`：

```rust
// 这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，
// 而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。
// 这个方法计算点实例与坐标 (0.0, 0.0) 之间的距离，并使用了只能用于浮点型的数学运算符。
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

结构体定义中的泛型类型参数并不总是与结构体方法签名中使用的泛型是同一类型。
示例中为 **Point** 结构体使用了泛型类型 **X1** 和 **Y1**，为 `mixup` 方法签名使用了 **X2** 和 **Y2** 来使得示例更加清楚。
这个方法用 `self` 的 **Point** 类型的 `x` 值（类型 **X1**）和参数的 **Point** 类型的 `y` 值（类型 **Y2**）来创建一个新 **Point** 类型的实例：

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// 方法使用了与结构体定义中不同类型的泛型
impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

在 `main` 函数中，定义了一个有 `i32` 类型的 x（其值为 5）和 f64 的 y（其值为 10.4）的 **Point**。
p2 则是一个有着字符串 slice 类型的 x（其值为 "Hello"）和 char 类型的 y（其值为c）的 **Point**。
在 p1 上以 p2 作为参数调用 `mixup` 会返回一个 p3，它会有一个 i32 类型的 x，因为 x 来自 p1，并拥有一个 char 类型的 y，因为 y 来自 p2。println! 会打印出 p3.x = 5, p3.y = c。

这个例子的目的是展示一些泛型通过 `impl` 声明而另一些通过方法定义声明的情况。
这里泛型参数 **X1** 和 **Y1** 声明于 `impl` 之后，因为他们与结构体定义相对应。
而泛型参数 **X2** 和 **Y2** 声明于 `fn mixup` 之后，因为他们只是相对于方法本身的。

## Trait

在阅读本部分内容的同时，你可能会好奇使用泛型类型参数是否会有运行时消耗。好消息是：{++Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。++}

Rust 通过在编译时进行泛型代码的 **单态化**（monomorphization）来保证效率。{++单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。++}

**编译器所做的工作正好与上面示例中我们创建泛型函数的步骤相反。编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码。**

让我们看看一个使用标准库中 **Option** 枚举的例子：

```rust
let integer = Some(5);
let float = Some(5.0);
```

当 Rust 编译这些代码的时候，它会进行单态化。
编译器会读取传递给 **Option<T\>** 的值并发现有两种 **Option<T\>**：一个对应 `i32` 另一个对应 `f64`。
为此，它会将泛型定义 **Option<T\>** 展开为 **Option_i32** 和 **Option_f64**，接着将泛型定义替换为这两个具体的定义。

编译器生成的单态化版本的代码看起来像这样，并包含将泛型 **Option<T\>** 替换为编译器创建的具体定义后的用例代码：

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

我们可以使用泛型来编写不重复的代码，**而 Rust 将会为每一个实例编译其特定类型的代码**。这意味着在使用泛型时没有运行时开销；
{++当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样。这个单态化过程正是 Rust 泛型在运行时极其高效的原因。++}

### 定义Trait

### 为类型实现Trait

### 默认实现

### trait作为参数

#### Trait Bound语法

#### 指定多个trait bound(+)

#### where简化trait bound

#### 返回实现了 trait 的类型

### 使用 trait bounds 来修复 largest 函数

### 使用 trait bound 有条件地实现方法

## 生命周期

### 生命周期避免了悬垂引用

#### 借用检查器

### 函数中的泛型生命周期

### 生命周期注解语法

### 函数签名中的生命周期注解

### 深入理解生命周期

### 结构体定义中的生命周期注解

### 生命周期省略（Lifetime Elision）

### 方法定义中的生命周期注解

### 静态生命周期

## 总结
