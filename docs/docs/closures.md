# 迭代器与闭包

## 闭包

### 使用闭包创建行为的抽象

版本1:

```rust
// src/main.rs

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("慢慢计算...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "今天, 做 {} 个俯卧撑!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "下一步，做 {} 个仰卧起坐！",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("今天休息一下！ 记得保持水分！")
        } else {
            println!(
                "今天，跑 {} 分钟！",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}
```

### 使用函数重构

将重复的 **simulated_expensive_calculation** 函数调用提取到一个变量中

```rust
// src/main.rs

// snap....
fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("今天, 做 {} 个俯卧撑!", expensive_result);
        println!("下一步，做 {} 个仰卧起坐！", expensive_result);
    } else {
        if random_number == 3 {
            println!("今天休息一下！ 记得保持水分！")
        } else {
            println!("今天，跑 {} 分钟！", expensive_result);
        }
    }
}
```

### 重构 - 使用闭包储存代码

**闭包不要求像 `fn` 函数那样在参数和返回值上注明类型。**
函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。
严格的定义这些接口对于保证所有人都认同函数使用和返回值的类型来说是很重要的。
但是闭包并不用于这样暴露在外的接口：他们储存在变量中并被使用，不用命名他们或暴露给库的用户调用。

闭包通常很短，并只关联于小范围的上下文而非任意情境。
在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型，类似于它是如何能够推断大部分变量的类型一样。

```rust
// src/main.rs

// snap....
fn generate_workout(intensity: u32, random_number: u32) {
    // 闭包重构代码
    let expensive_closure = |num| {
        println!("慢慢计算...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("今天, 做 {} 个俯卧撑!", expensive_closure(intensity));
        println!("下一步，做 {} 个仰卧起坐！", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("今天休息一下！ 记得保持水分！")
        } else {
            println!("今天，跑 {} 分钟！", expensive_closure(intensity));
        }
    }
}
```

### 闭包 - 类型推断和注解

**有了类型注解闭包的语法就更类似函数了。**
如下是一个对其参数加一的函数的定义与拥有相同行为闭包语法的纵向对比。
这里增加了一些空格来对齐相应部分。
这展示了闭包语法如何类似于函数语法，除了使用竖线而不是括号以及几个可选的语法之外：

```rust
fn  add_one_v1   (x: u32) -> u32 { x + 1 }      // 展示了一个函数定义
let add_one_v2 = |x: u32| -> u32 { x + 1 };     // 展示了一个完整标注的闭包定义
let add_one_v3 = |x|             { x + 1 };     // 定义中省略了类型注解
let add_one_v4 = |x|               x + 1  ;     // 去掉了可选的大括号，因为闭包体只有一行
```

调用闭包是 `add_one_v3` 和 `add_one_v4` 能够编译的必要条件，因为类型将从其用法中推断出来。

**闭包定义会为每个参数和返回值推断一个具体类型。** {++如果尝试调用闭包两次，第一次使用 `String` 类型作为参数而第二次使用 `u32`，则会得到一个错误：++}

```rust
// 尝试调用一个被推断为两个不同类型的闭包

fn main() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello")); // 用法为String
    let n = example_closure(5);                     // 用法为u32
}
```

### 使用带有泛型和 Fn trait 的闭包

可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。你可能见过这种模式被称 *memoization* 或 *lazy evaluation* （惰性求值）。

为了让结构体存放闭包，我们需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。为了定义使用闭包的结构体、枚举或函数参数，需要像第十章讨论的那样使用泛型和 **trait bound**。

**Fn** 系列 **trait** 由标准库提供。所有的闭包都实现了 **trait Fn**、**FnMut** 或 **FnOnce** 中的一个。在 “闭包会捕获其环境” 部分我们会讨论这些 **trait** 的区别；在这个例子中可以使用 **Fn trait**。

为了满足 **Fn trait bound** 我们增加了代表闭包所必须的参数和返回值类型的类型。在这个例子中，闭包有一个 `u32` 的参数并返回一个 `u32`，这样所指定的 **trait bound** 就是 `Fn(u32) -> u32`。

示例展示了存放了闭包和一个 **Option** 结果值的 **Cacher** 结构体的定义：

```rust
// 定义一个 Cacher 结构体来在 calculation 中存放闭包并在 value 中存放 Option 值
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}
```

结构体 **Cacher** 有一个泛型 `T` 的字段 **calculation**。
`T` 的 **trait bound** 指定了 `T` 是一个使用 **Fn** 的闭包。
任何我们希望储存到 **Cacher** 实例的 **calculation** 字段的闭包必须有一个 `u32` 参数（由 **Fn** 之后的括号的内容指定）并必须返回一个 **u32**（由 `->` 之后的内容）。

> **注意**：
>
> 函数也都实现了这三个 **Fn trait**。如果不需要捕获环境中的值，则可以使用实现了 **Fn trait** 的函数而不是闭包。

字段 **value** 是 **Option<u32\>** 类型的。在执行闭包之前，**value** 将是 **None**。如果使用 **Cacher** 的代码请求闭包的结果，这时会执行闭包并将结果储存在 **value** 字段的 **Some** 成员中。接着如果代码再次请求闭包的结果，这时不再执行闭包，而是会返回存放在 **Some** 成员中的结果。

刚才讨论的有关 value 字段逻辑定义于示例：

```rust
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

**Cacher** 结构体的字段是私有的，因为我们希望 **Cacher** 管理这些值而不是任由调用代码潜在的直接改变他们。

当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用 **value** 方法。
这个方法会检查 **self.value** 是否已经有了一个 **Some** 的结果值；
如果有，它返回 **Some** 中的值并不会再次执行闭包。

如果 **self.value** 是 **None**，则会调用 **self.calculation** 中储存的闭包，将结果保存到 **self.value** 以便将来使用，并同时返回结果值。

```rust
// snap...
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("慢慢计算...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("今天, 做 {} 个俯卧撑!", expensive_result.value(intensity));
        println!(
            "下一步，做 {} 个仰卧起坐！",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("今天休息一下！ 记得保持水分！")
        } else {
            println!("今天，跑 {} 分钟！", expensive_result.value(intensity));
        }
    }
}
```

不同于直接将闭包保存进一个变量，我们保存一个新的 **Cacher** 实例来存放闭包。接着，在每一个需要结果的地方，调用 **Cacher** 实例的 **value** 方法。可以调用 **value** 方法任意多次，或者一次也不调用，而慢计算最多只会运行一次。

### Cacher 实现的限制

值缓存是一种更加广泛的实用行为，我们可能希望在代码中的其他闭包中也使用他们。然而，目前 **Cacher** 的实现存在两个小问题，这使得在不同上下文中复用变得很困难。

第一个问题是 **Cacher** 实例假设对于 **value** 方法的任何 **arg** 参数值总是会返回相同的值。也就是说，这个 **Cacher** 的测试会失败：

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
```

这个测试使用返回传递给它的值的闭包创建了一个新的 **Cacher** 实例。使用为 `1` 的 **arg** 和为 `2` 的 **arg** 调用 **Cacher** 实例的 **value** 方法，同时我们期望使用为 `2` 的 **arg** 调用 **value** 会返回 `2`。

使用示例的 **Cacher** 实现运行测试，它会在 **assert_eq!** 失败并显示如下信息：

```shell
$ cargo test
   Compiling cacher v0.1.0 (file:///projects/cacher)
    Finished test [unoptimized + debuginfo] target(s) in 0.72s
     Running unittests (target/debug/deps/cacher-074d7c200c000afa)

running 1 test
test tests::call_with_different_values ... FAILED

failures:

---- tests::call_with_different_values stdout ----
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `2`', src/lib.rs:43:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::call_with_different_values

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

这里的问题是第一次使用 `1` 调用 `c.value`，**Cacher** 实例将 **Some(1)** 保存进 **self.value**。在这之后，无论传递什么值调用 **value**，它总是会返回 **1**。

尝试修改 **Cacher** 存放一个哈希 **map** 而不是单独一个值。哈希 **map** 的 **key** 将是传递进来的 **arg** 值，而 **value** 则是对应 **key** 调用闭包的结果值。相比之前检查 **self.value** 直接是 **Some** 还是 **None** 值，现在 **value** 函数会在哈希 **map** 中寻找 **arg**，如果找到的话就返回其对应的值。如果不存在，**Cacher** 会调用闭包并将结果值保存在哈希 **map** 对应 **arg** 值的位置。

当前 **Cacher** 实现的第二个问题是它的应用被限制为只接受获取一个 **u32** 值并返回一个 **u32** 值的闭包。比如说，我们可能需要能够缓存一个获取字符串 **slice** 并返回 **usize** 值的闭包的结果。请尝试引入更多泛型参数来增加 **Cacher** 功能的灵活性。

### 针对问题2的完善

```rust
use std::collections::HashMap;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    map: HashMap<u32, u32>,  // 使用hash map 存储
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        let value = self.map.get(&arg);

        if let Some(v) = value {
            *v
        } else {
            let v = (self.calculation)(arg);
            self.map.insert(arg, v);
            v
        }
    }
}

// 测试可以通过
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}
```

### 闭包会捕获其环境

闭包还有另一个函数所没有的功能：**他们可以捕获其环境并访问其被定义的作用域的变量。**

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
```

这里，即便 `x` 并不是 `equal_to_x` 的一个参数，`equal_to_x` 闭包也被允许使用变量 `x`，因为它与 `equal_to_x` 定义于相同的作用域。

**当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。**
这会使用内存并产生额外的开销，在更一般的场景中，当我们不需要闭包来捕获环境时，我们不希望产生这些开销。
因为函数从未允许捕获环境，定义和使用函数也就从不会有这些额外开销。

闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：**获取所有权**，**可变借用**和**不可变借用**。
这三种捕获值的方式被编码为如下三个 **Fn trait**：

* **FnOnce** 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 **环境**，**environment**。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 **Once** 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
* **FnMut** 获取可变的借用值所以可以改变其环境
* **Fn** 从其环境获取不可变的借用值

当创建一个闭包时，Rust 根据其如何使用环境中变量来推断我们希望如何引用环境。
由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 **FnOnce** 。
那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 **FnMut** ，而不需要对被捕获的变量进行可变访问的闭包则也实现了 **Fn** 。
在上面示例中，`equal_to_x` 闭包不可变的借用了 `x`（所以 `equal_to_x` 具有 **Fn trait**），因为闭包体只需要读取 `x` 的值。

{++如果希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 **move** 关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。++}

> **注意**：
>
> 即使其捕获的值已经被移动了，**move** 闭包仍需要实现 **Fn** 或 **FnMut**。这是因为闭包所实现的 **trait** 是由闭包所捕获了什么值而不是如何捕获所决定的。而 **move** 关键字仅代表了后者。

修改了示例中的代码（作为演示），在闭包定义中增加 **move** 关键字并使用 **vector** 代替整型，因为整型可以被拷贝而不是移动；注意这些代码还不能编译：

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    println!("不能在这儿使用 x: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

这个例子并不能编译，会产生以下错误：

```shell
$ cargo run
   Compiling equal-to-x v0.1.0 (file:///projects/equal-to-x)
error[E0382]: borrow of moved value: `x`
 --> src/main.rs:6:40
  |
2 |     let x = vec![1, 2, 3];
  |         - move occurs because `x` has type `Vec<i32>`, which does not implement the `Copy` trait
3 | 
4 |     let equal_to_x = move |z| z == x;
  |                      --------      - variable moved due to use in closure
  |                      |
  |                      value moved into closure here
5 | 
6 |     println!("can't use x here: {:?}", x);
  |                                        ^ value borrowed here after move

For more information about this error, try `rustc --explain E0382`.
error: could not compile `equal-to-x` due to previous error
```

`x` 被移动进了闭包，因为闭包使用 **move** 关键字定义。接着闭包获取了 `x` 的所有权，同时 **main** 就不再允许在 **println!** 语句中使用 `x` 了。去掉 **println!** 即可修复问题。

大部分需要指定一个 **Fn** 系列 **trait bound** 的时候，可以从 **Fn** 开始，而编译器会根据闭包体中的情况告诉你是否需要 **FnMut** 或 **FnOnce**。

## 迭代器

**迭代器**（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。

在 Rust 中，迭代器是 **惰性的**（lazy），这意味着在调用方法使用迭代器之前它都不会有效果。例如，示例代码通过调用定义于 **Vec** 上的 **iter** 方法在一个 **vector v1** 上创建了一个迭代器。这段代码本身没有任何用处：

```rust
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();
```

一旦创建迭代器之后，可以选择用多种方式利用它。

迭代器被储存在 **v1_iter** 变量中，而这时没有进行迭代。一旦 **for** 循环开始使用 **v1_iter**，接着迭代器中的每一个元素被用于循环的一次迭代，这会打印出其每一个值：

```rust
fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
```

在标准库中没有提供迭代器的语言中，我们可能会使用一个从 0 开始的索引变量，使用这个变量索引 vector 中的值，并循环增加其值直到达到 vector 的元素数量。

迭代器为我们处理了所有这些逻辑，这减少了重复代码并消除了潜在的混乱。另外，迭代器的实现方式提供了对多种不同的序列使用相同逻辑的灵活性，而不仅仅是像 vector 这样可索引的数据结构.让我们看看迭代器是如何做到这些的。

### Iterator trait 和 next 方法

迭代器都实现了一个叫做 **Iterator** 的定义于标准库的 trait。这个 trait 的定义看起来像这样：

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // 此处省略了方法的默认实现
}
```

注意这里有一个我们还未讲到的新语法：**type Item** 和 **Self::Item**，他们定义了 trait 的 **关联类型**（associated type）。
第十九章会深入讲解关联类型，不过现在只需知道这段代码表明实现 **Iterator trait** 要求同时定义一个 **Item** 类型，这个 **Item** 类型被用作 **next** 方法的返回值类型。
换句话说，**Item** 类型将是迭代器返回元素的类型。

`next` 是 **Iterator** 实现者被要求定义的唯一方法。**next** 一次返回迭代器中的一个项，封装在 **Some** 中，当迭代器结束时，它返回 **None**。

可以直接调用迭代器的 `next` 方法；示例有一个测试展示了重复调用由 **vector** 创建的迭代器的 `next` 方法所得到的值：

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

注意 **v1_iter** 需要是可变的：在迭代器上调用 **next** 方法改变了迭代器中用来记录序列位置的状态。换句话说，代码 **消费**（consume）了，或使用了迭代器。每一个 **next** 调用都会从迭代器中消费一个项。使用 **for** 循环时无需使 **v1_iter** 可变因为 **for** 循环会获取 **v1_iter** 的所有权并在后台使 **v1_iter** 可变。

另外需要注意到从 **next** 调用中得到的值是 **vector** 的不可变引用。{++**iter** 方法生成一个不可变引用的迭代器。如果我们需要一个获取 **v1** 所有权并返回拥有所有权的迭代器，则可以调用 **into_iter** 而不是 **iter**。类似的，如果我们希望迭代可变引用，则可以调用 **iter_mut** 而不是 **iter**。++}

### 消费迭代器的方法

{++**Iterator trait** 有一系列不同的由标准库提供默认实现的方法；你可以在 **Iterator trait** 的[标准库 API](https://www.rustwiki.org.cn/zh-CN/std/iter/trait.Iterator.html) 文档中找到所有这些方法。一些方法在其定义中调用了 **next** 方法，这也就是为什么在实现 **Iterator trait** 时要求实现 **next** 方法的原因。++}

{++这些调用 **next** 方法的方法被称为 **消费适配器**（consuming adaptors），因为调用他们会消耗迭代器。一个消费适配器的例子是 **sum** 方法。这个方法获取迭代器的所有权并反复调用 **next** 来遍历迭代器，因而会消费迭代器。当其遍历每一个项时，它将每一个项加总到一个总和并在迭代完成时返回总和。++}示例有一个展示 **sum** 方法使用的测试：

```rust
// 调用 sum 方法获取迭代器所有项的总和

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

调用 `sum` 之后不再允许使用 **v1_iter** 因为调用 `sum` 时它会获取迭代器的所有权。

### 产生其他迭代器的方法

**Iterator trait** 中定义了另一类方法，被称为 **迭代器适配器**（iterator adaptors），{++他们允许我们将当前迭代器变为不同类型的迭代器。可以链式调用多个迭代器适配器。不过因为所有的迭代器都是惰性的，必须调用一个消费适配器方法以便获取迭代器适配器调用的结果。++}

示例展示了一个调用迭代器适配器方法 **map** 的例子，该 **map** 方法使用闭包来调用每个元素以生成新的迭代器。
这里的闭包创建了一个新的迭代器，对其中 **vector** 中的每个元素都被加 `1`。不过这些代码会产生一个警告：

```rust
// 调用迭代器适配器 map 来创建一个新迭代器

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    v1.iter().map(|x| x + 1);
}
```

得到的警告是：

```shell
$ cargo run
   Compiling iterators v0.1.0 (file:///projects/iterators)
warning: unused `Map` that must be used
 --> src/main.rs:4:5
  |
4 |     v1.iter().map(|x| x + 1);
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_must_use)]` on by default
  = note: iterators are lazy and do nothing unless consumed

warning: `iterators` (bin "iterators") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.47s
     Running `target/debug/iterators`
```

示例中的代码实际上并没有做任何事；所指定的闭包从未被调用过。警告提醒了我们为什么：**迭代器适配器是惰性的，而这里我们需要消费迭代器。**

为了修复这个警告并消费迭代器获取有用的结果，我们将使用第十二章示例结合 **env::args** 使用的 **collect** 方法。这个方法消费迭代器并将结果收集到一个数据结构中。

在示例中，我们将遍历由 **map** 调用生成的迭代器的结果收集到一个 **vector** 中，它将会含有原始 **vector** 中每个元素加 `1` 的结果：

```rust
// 调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector

let v1: Vec<i32> = vec![1, 2, 3];

let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();  // 消费生成的迭代器

assert_eq!(v2, vec![2, 3, 4]);
```

因为 **map** 获取一个闭包，可以指定任何希望在遍历的每个元素上执行的操作。这是一个展示如何使用闭包来自定义行为同时又复用 **Iterator trait** 提供的迭代行为的绝佳例子。

### 使用闭包获取环境

现在我们介绍了迭代器，让我们展示一个通过使用 `filter` 迭代器适配器和捕获环境的闭包的常规用例。迭代器的 `filter` 方法获取一个使用迭代器的每一个项并返回布尔值的闭包。如果闭包返回 `true`，其值将会包含在 `filter` 提供的新迭代器中。如果闭包返回 `false`，其值不会包含在结果迭代器中。

示例展示了使用 `filter` 和一个捕获环境中变量 `shoe_size` 的闭包，这样闭包就可以遍历一个 **Shoe** 结构体集合以便只返回指定大小的鞋子：

```rust
// 使用 filter 方法和一个捕获 shoe_size 的闭包

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

`shoes_in_my_size` 函数获取一个鞋子 `vector` 的所有权和一个鞋子大小作为参数。它返回一个只包含指定大小鞋子的 `vector`。

`shoes_in_my_size` 函数体中调用了 `into_iter` 来创建一个获取 `vector` 所有权的迭代器。接着调用 `filter` 将这个迭代器适配成一个只含有那些闭包返回 `true` 的元素的新迭代器。

闭包从环境中捕获了 `shoe_size` 变量并使用其值与每一只鞋的大小作比较，只保留指定大小的鞋子。最终，调用 `collect` 将迭代器适配器返回的值收集进一个 `vector` 并返回。

这个测试展示当调用 `shoes_in_my_size` 时，我们只会得到与指定值相同大小的鞋子。

### 实现 Iterator trait 来创建自定义迭代器

已经展示了可以通过在 **vector** 上调用 `iter`、`into_iter` 或 `iter_mut` 来创建一个迭代器。也可以用标准库中其他的集合类型创建迭代器，比如**哈希 map**。另外，可以实现 **Iterator trait** 来创建任何我们希望的迭代器。正如之前提到的，定义中唯一要求提供的方法就是 `next` 方法。一旦定义了它，就可以使用所有其他由 **Iterator trait** 提供的拥有默认实现的方法来创建自定义迭代器了！

作为展示，让我们创建一个只会从 `1` 数到 `5` 的迭代器。首先，创建一个结构体来存放一些值，接着实现 **Iterator trait** 将这个结构体放入迭代器中并在此实现中使用其值。

示例有一个 **Counter** 结构体定义和一个创建 **Counter** 实例的关联函数 **new**：

```rust
// 定义 Counter 结构体和一个创建 count 初值为 0 的 Counter 实例的 new 函数

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
```

接下来将为 **Counter** 类型实现 **Iterator trait**，通过定义 `next` 方法来指定使用迭代器时的行为，如下所示：

```rust
// 在 Counter 结构体上实现 Iterator trait

impl Iterator for Counter {
    type Item = u32;  // 这里将迭代器的关联类型 Item 设置为 u32，意味着迭代器会返回 u32 值集合。

    fn next(&mut self) -> Option<Self::Item> {
        // 希望迭代器对其内部状态加一，这也就是为何将 count 初始化为 0：
        // 我们希望迭代器首先返回 1。
        // 如果 count 值小于 6，next 会返回封装在 Some 中的当前值，
        // 不过如果 count 大于或等于 6，迭代器会返回 None。
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 使用 Counter 迭代器的 next 方法

一旦实现了 **Iterator trait**，我们就有了一个迭代器！下面示例展示了一个测试用来演示使用 **Counter** 结构体的迭代器功能，通过直接调用 `next` 方法，正如从 **vector** 创建的迭代器那样：

```rust
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

### 使用自定义迭代器中其他 Iterator trait 方法

通过定义 `next` 方法实现 **Iterator trait**，我们现在就可以使用任何标准库定义的拥有默认实现的 `Iterator trait` 方法了，因为他们都使用了 **next** 方法的功能。

例如，出于某种原因我们希望获取 **Counter** 实例产生的值，将这些值与另一个 **Counter** 实例在省略了第一个值之后产生的值配对，将每一对值相乘，只保留那些可以被三整除的结果，然后将所有保留的结果相加，这可以如下示例中的测试这样做：

```rust
// 使用自定义的 Counter 迭代器的多种方法

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
```

注意 `zip` 只产生四对值；理论上第五对值 `(5, None)` 从未被产生，因为 `zip` 在任一输入迭代器返回 `None` 时也返回 `None`。

所有这些方法调用都是可能的，因为我们指定了 `next` 方法如何工作，而标准库则提供了其它调用 `next` 的方法的默认实现。

#### 自定义的一个实现

跳过指定的 **value** 的实现。

```rust
struct Counter {
    count: u32,
    skip_value: Option<u32>,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0,
            skip_value: None,
        }
    }

    fn skip_value(&mut self, skip: u32) {
        self.skip_value = Some(skip);
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;

            if let Some(skip_value) = self.skip_value {
                if self.count == skip_value {
                    self.count += 1;

                    if self.count > 5 {
                        return None;
                    }
                }
            }

            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

#[test]
fn skip_special_value() {
    let mut counter = Counter::new();
    counter.skip_value(5);

    let sum: u32 = counter.sum();
    assert_eq!(1 + 2 + 3 + 4, sum);
}
```

这段实现会跳过指定的**value**。

## 改进I/O项目

有了这些关于迭代器的新知识，我们可以使用迭代器来改进第十二章中 I/O 项目的实现来使得代码更简洁明了。让我们看看迭代器如何能够改进 **Config::new** 函数和 **search** 函数的实现。

### 使用迭代器并去掉 clone

在示例中，我们增加了一些代码获取一个 **String slice** 并创建一个 **Config** 结构体的实例，他们索引 **slice** 中的值并克隆这些值以便 **Config** 结构体可以拥有这些值。在示例中重现了第十二章结尾示例中 **Config::new** 函数的实现：

```rust
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

起初这里需要 **clone** 的原因是参数 **args** 中有一个 **String** 元素的 **slice**，而 **new** 函数并不拥有 **args**。为了能够返回 **Config** 实例的所有权，我们需要克隆 **Config** 中字段 **query** 和 **filename** 的值，这样 **Config** 实例就能拥有这些值。

在学习了迭代器之后，我们可以将 **new** 函数改为获取一个有所有权的迭代器作为参数而不是借用 **slice**。我们将使用迭代器功能之前检查 **slice** 长度和索引特定位置的代码。这会明确 **Config::new** 的工作因为迭代器会负责访问这些值。

一旦 **Config::new** 获取了迭代器的所有权并不再使用借用的索引操作，就可以将迭代器中的 **String** 值移动到 **Config** 中，而不是调用 **clone** 分配新的空间。

### 直接使用 env::args 返回的迭代器

打开 I/O 项目的 **src/main.rs** 文件，它看起来应该像这样：

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

修改第十二章结尾示例中的 **main** 函数的开头为示例中的代码。在更新 **Config::new** 之前这些代码还不能编译：

```rust
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // --snip--
}
```

**env::args** 函数返回一个迭代器！不同于将迭代器的值收集到一个 **vector** 中接着传递一个 **slice** 给 **Config::new**，现在我们直接将 **env::args** 返回的迭代器的所有权传递给 **Config::new**。

接下来需要更新 **Config::new** 的定义。在 I/O 项目的 **src/lib.rs** 中，将 **Config::new** 的签名改为如示例所示。这仍然不能编译因为我们还需更新函数体：

```rust
// 以迭代器作为参数更新 Config::new 的签名

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // --snip--
```

**env::args** 函数的标准库文档显示，它返回的迭代器的类型为 **std::env::Args**。我们已经更新了 **Config :: new** 函数的签名，因此参数 **args** 的类型为 **std::env::Args** 而不是 **&[String]**。因为我们拥有 **args** 的所有权，并且将通过对其进行迭代来改变 **args** ，所以我们可以将 **mut** 关键字添加到 **args** 参数的规范中以使其可变。

### 使用 Iterator trait 代替索引

接下来，我们将修改 **Config::new** 的内容。标准库文档还提到 **std::env::Args** 实现了 **Iterator trait**，因此我们知道可以对其调用 **next** 方法！示例更新了中的代码，以使用 **next** 方法：

```rust
impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

请记住 **env::args** 返回值的第一个值是程序的名称。我们希望忽略它并获取下一个值，所以首先调用 **next** 并不对返回值做任何操作。之后对希望放入 **Config** 中字段 **query** 调用 **next**。如果 **next** 返回 **Some**，使用 **match** 来提取其值。如果它返回 **None**，则意味着没有提供足够的参数并通过 **Err** 值提早返回。对 **filename** 值进行同样的操作。

### 使用迭代器适配器来使代码更简明

I/O 项目中其他可以利用迭代器的地方是 **search** 函数，示例中重现了第十二章结尾示例中此函数的定义：

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

可以通过使用迭代器适配器方法来编写更简明的代码。这也避免了一个可变的中间 **results vector** 的使用。{++函数式编程风格倾向于最小化可变状态的数量来使代码更简洁。去掉可变状态可能会使得将来进行并行搜索的增强变得更容易，因为我们不必管理 **results vector** 的并发访问。++}示例展示了该变化：

```rust
// 在 search 函数实现中使用迭代器适配器

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
```

回忆 **search** 函数的目的是返回所有 **contents** 中包含 **query** 的行。类似于示例中的 **filter** 例子，可以使用 **filter** 适配器只保留 **line.contains(query)** 返回 **true** 的那些行。接着使用 **collect** 将匹配行收集到另一个 **vector** 中。这样就容易多了！尝试对 **search_case_insensitive** 函数做出同样的使用迭代器方法的修改吧。

接下来的逻辑问题就是在代码中应该选择哪种风格：是使用示例中的原始实现还是使用迭代器的版本？大部分 Rust 程序员倾向于使用迭代器风格。开始这有点难以理解，不过一旦你对不同迭代器的工作方式有了感觉之后，迭代器可能会更容易理解。相比摆弄不同的循环并创建新 **vector**，（迭代器）代码则更关注循环的目的。这抽象掉那些老生常谈的代码，这样就更容易看清代码所特有的概念，比如迭代器中每个元素必须面对的过滤条件。

不过这两种实现真的完全等同吗？直觉上的假设是更底层的循环会更快一些。让我们聊聊性能吧。

#### 忽略大小写函数使用迭代器版本

原版本:

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    // 使用迭代器版本

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}
```

## 性能比较-循环和迭代器
