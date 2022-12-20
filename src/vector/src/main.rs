fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个数是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {}", third),
        None => println!("这儿没有元素"),
    }

    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    for i in v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
        Char(Option<i32>),
        Bool(bool),
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Char(Some(1)),
        SpreadsheetCell::Char(None),
        SpreadsheetCell::Bool(true),
        SpreadsheetCell::Bool(false),
    ];

    row.push(SpreadsheetCell::Text(String::from("我是最后一个元素")));

    dbg!(&row);
    println!("row is {:?}", &row);

    for i in &row {
        match i {
            SpreadsheetCell::Int(value) => {
                println!("i is {}", value)
            }
            SpreadsheetCell::Float(value) => {
                println!("i is {value}")
            }
            SpreadsheetCell::Text(value) => {
                println!("i is {value}")
            }
            SpreadsheetCell::Char(value) => {
                if let Some(value2) = value {
                    println!("i is {}", value2)
                } else {
                    println!("没有值")
                }
            }
            SpreadsheetCell::Bool(value) => {
                println!("i is {value}")
            }
        }
    }
}
