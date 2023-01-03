use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    map: HashMap<u32, u32>,
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

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v1, 1);
    assert_eq!(v2, 2);
}

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

fn generate_workout_v3(intensity: u32, random_number: u32) {
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

fn generate_workout_v2(intensity: u32, random_number: u32) {
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

fn generate_workout_v1(intensity: u32, random_number: u32) {
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
