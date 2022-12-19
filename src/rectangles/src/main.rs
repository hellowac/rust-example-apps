struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("矩形rect1可以包含rect2: {}", rect1.can_hold(&rect2));
    println!("矩形rect2可以包含rect3: {}", rect2.can_hold(&rect3));

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z = match y {
        None => 0,
        Some(t) => t,
    };

    let sum = x + z;

    println!("sum is {sum}");

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let dice_roll = 9;
    match dice_roll {
        // 对于前两个分支，匹配模式是字面值 3 和 7
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 最后一个分支则涵盖了所有其他可能的值
        other => move_player(other),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("num_spaces is {num_spaces}")
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
