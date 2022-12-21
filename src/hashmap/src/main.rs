use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    dbg!(scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    dbg!(scores2);

    let mut field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(&field_name[..], &field_value[..]);

    dbg!(map);

    field_name.remove(2);

    println!("field_name is {field_name}");
    println!("field_value is {field_value}");

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    dbg!(score);

    if let Some(value) = score {
        println!("value is {value}")
    } else {
        println!("not value")
    }

    match score {
        Some(value) => {
            println!("value is {value}")
        }
        // None => {
        //     println!("not value")
        // }
        _ => println!("not value"),
    }

    for (k, v) in &scores {
        println!("{}:{}", k, v);
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);

        *count += 1;
    }

    println!("{:?}", map);

    // 下面为练习题

    // practise1();
    practise2();
}

// 练习题1 给定一系列数字，使用 vector 并返回这个列表的中位数（排列数组后位于中间的值）和众数（mode，出现次数最多的值；这里哈希 map 会很有帮助）。
fn practise1() {
    let mut numbers = vec![];

    for _ in 0..100 {
        let rand_number = thread_rng().gen_range(0..100);
        numbers.push(rand_number);
    }

    numbers.sort(); // 从小到大排序
    numbers.reverse(); // 逆序

    println!("生成的随机数列是: {:?}", &numbers);

    let mid_idx = numbers.len() / 2;

    println!("中位数是: {}", numbers[mid_idx]);

    let max_word = max_count_word(&numbers);

    if let Some(value) = max_word {
        println!("众数是: {}", value);
    } else {
        println!("未发现众数!");
    }
}

fn max_count_word(vec: &Vec<i32>) -> Option<&i32> {
    let mut word_count = HashMap::new();

    for word in vec {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_count_key = None;

    for (k, v) in word_count {
        if v > max_count {
            max_count = v;
            max_count_key = Some(k);
        }
    }

    max_count_key
}

// 练习题2 将字符串转换为 Pig Latin，也就是每一个单词的第一个辅音字母被移动到单词的结尾并增加 “ay”，所以 “first” 会变成 “irst-fay”。元音字母开头的单词则在结尾增加 “hay”（“apple” 会变成 “apple-hay”）。牢记 UTF-8 编码！
fn practise2() {
    let vowel_chars = ['a', 'e', 'i', 'o', 'u'];

    // 从控制台获取单词
    let word = get_word_from_line();

    println!("获取的单词是: {}", &word);

    let mut fixed_word = String::from("");

    let mut not_vowel_char: Option<char> = None;
    let mut other_chars = String::from("");

    for (idx, c) in word.chars().enumerate() {
        // 是否为第一个字符并且不是元音字母开头
        if idx == 0 && !vowel_chars.contains(&c) {
            not_vowel_char = Some(c);
            continue;
        } else {
            other_chars.push(c);
        };
    }

    // 非元音字幕开头的情况
    if let Some(not_vowel_c) = not_vowel_char {
        fixed_word = format!("{}-{}ay", other_chars, not_vowel_c);

    // 元音字母开头的情况
    } else {
        fixed_word = format!("{}-hay", other_chars);
    }

    println!("修改后的字符为: {}", fixed_word);
}

fn get_word_from_line() -> String {
    let mut word = String::new();

    println!("请输入一个单词:");

    loop {
        match io::stdin().read_line(&mut word) {
            Ok(_) => break,
            Err(_) => continue,
        }
    }

    word = match word.trim().parse() {
        Ok(w) => w,
        Err(_) => String::from(""),
    };

    word
}
