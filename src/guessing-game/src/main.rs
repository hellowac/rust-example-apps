use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数游戏！！");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("请输入一个值:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("读数失败!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数为: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了！"),
            Ordering::Greater => println!("太大了！"),
            Ordering::Equal => {
                println!("猜对了！");
                break;
            }
        }
    }
}
