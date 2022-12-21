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
    // practise2();
    practise3();
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

// 使用哈希 map 和 vector，创建一个文本接口来允许用户向公司的部门中增加员工的名字。例如，“Add Sally to Engineering” 或 “Add Amir to Sales”。接着让用户获取一个部门的所有员工的列表，或者公司每个部门的所有员工按照字典序排列的列表。

fn practise3() {
    let mut dep_map = HashMap::new();

    let mut continue_add = true;

    // 循环添加人员到部门
    while continue_add {
        println!("请输入要添加的人员和部门(add who to where):");
        let add_text = get_input_text(); // Add Sally to Engineering

        let mut name: Option<String> = None;
        let mut department: Option<String> = None;

        for (idx, word) in add_text.split_whitespace().enumerate() {
            let text: String = match word.parse() {
                Ok(name) => {
                    println!("解析的单词为: {name}");
                    name
                }
                Err(_) => {
                    println!("解析输入失败");
                    break;
                }
            };

            match idx {
                1 => name = Some(String::from(text)),       // 索引为1为名称；
                3 => department = Some(String::from(text)), // 索引为3为部门；
                _ => (),
            }
        }

        // 增加部门到公司
        if let Some(name) = name {
            if let Some(department) = department {
                let persons = dep_map.entry(department).or_insert(Vec::from([]));
                persons.push(name);
            }
        }

        println!("是否继续增加(y/n):");

        continue_add = get_answer();
    }

    println!("请输入要获取的员工列表的部门(where):");

    let dep_name = get_input_text();

    let mut persons = dep_map.get_mut(&dep_name); // 获取可以改变的vec集合

    if let Some(persons) = persons {
        println!("{} 部门的员工有:", &dep_name);

        persons.sort(); // 根据字母表排序

        for person in persons {
            println!("\t{}", person);
        }
    } else {
        println!("{}部门没有员工存在!", &dep_name)
    }

    dbg!(dep_map);
}

fn get_answer() -> bool {
    let mut word = String::new();

    loop {
        match io::stdin().read_line(&mut word) {
            Ok(_) => {
                let chars_count = word.trim().chars().count();

                dbg!(chars_count);
                dbg!(word.trim().chars());

                if chars_count == 1 && word.trim().chars().eq("y".chars()) {
                    return true;
                }

                return false;
            }
            Err(_) => {
                println!("请输入y或者n!");
                continue;
            }
        }
    }
}

fn get_input_text() -> String {
    let mut word = String::new();

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
