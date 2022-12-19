fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("s is {r1}");
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    println!("s is {s}");

    let r2 = &mut s;

    let mut s = "123hello";

    let b = &s;

    dbg!(b);

    println!("s is {b}")
}
