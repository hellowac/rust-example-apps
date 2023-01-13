#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main1() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let b = Box::new(5);
    println!("b = {}", b);

    dbg!(list);
}

fn main2() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn main3() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main4() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main5() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

fn main6() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main7() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn main8() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); // std::mem::drop 函数, 其已经prelude
    println!("CustomSmartPointer dropped before the end of main.");
}

// fn main9() {
//     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
//     let b = Cons(3, Box::new(a));
//     let c = Cons(4, Box::new(a)); // 报错, 因为 a 不能被拥有所有权2次
// }

enum List1 {
    Cons1(i32, Rc<List1>),
    Nil1,
}

use crate::List1::{Cons1, Nil1};
use std::rc::Rc;

fn main10() {
    let a = Rc::new(Cons1(5, Rc::new(Cons1(10, Rc::new(Nil1)))));
    let b = Cons1(3, Rc::clone(&a));
    let c = Cons1(4, Rc::clone(&a)); // 不会报错， 因为 Rc 结构可以共享所有权。
}

fn main() {
    let a = Rc::new(Cons1(5, Rc::new(Cons1(10, Rc::new(Nil1)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons1(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons1(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
