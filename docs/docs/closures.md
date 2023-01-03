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
