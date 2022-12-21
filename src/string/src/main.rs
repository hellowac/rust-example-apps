fn main() {
    let data = "initial contents";

    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();

    let mut s1 = "foo".to_string();

    let s2 = "bar";

    s1.push_str(s2);
    println!("s2 is {s2}");

    let mut s3 = String::from("lo");

    s3.push('l');
    println!("s3 is {s3}");

    let a = String::from("Hello ");
    let b = String::from("World");
    let c = a + &b;

    println!("b is {b}");
    println!("c is {c}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);

    println!("s1 is {s1}");
    println!("s2 is {s2}");
    println!("s3 is {s3}");
    println!("s4 is {s4}");

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("s is {s}");

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for c in "नमस्ते".bytes() {
        println!("{c}");
    }
}
