// ============================================
// Demo 06: 枚举与模式匹配 (Enum & Pattern Matching)
// 运行: cargo run --example 06_enum
// ============================================

// ---------- 定义枚举 ----------
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8), // 可以携带数据
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // 无数据
    Move { x: i32, y: i32 },    // 匿名结构体
    Write(String),              // 单个值
    ChangeColor(i32, i32, i32), // 多个值
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
            Message::Write(text) => println!("写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("颜色: RGB({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // ---------- 使用枚举 ----------
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("home: {:?}", home);
    println!("loopback: {:?}", loopback);

    // ---------- match 表达式 ----------
    let msg1 = Message::Write(String::from("hello"));
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Quit; // 使用 Quit 变体
    msg1.process();
    msg2.process();
    msg3.process();

    // ---------- Option<T> - 处理空值 ----------
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    // 必须处理 None 的情况
    match some_number {
        Some(n) => println!("数字是: {}", n),
        None => println!("没有数字"),
    }

    // 处理 None 的情况
    match no_number {
        Some(n) => println!("no_number 的值: {}", n),
        None => println!("no_number 是 None"),
    }

    // if let - 只关心一种情况时的简写
    if let Some(n) = some_number {
        println!("if let: 数字是 {}", n);
    }

    // ---------- 模式匹配的其他用法 ----------
    let x = 5;
    match x {
        1 => println!("一"),
        2 | 3 => println!("二或三"), // 多个模式
        4..=9 => println!("四到九"), // 范围
        _ => println!("其他"),       // 通配符
    }

    // 匹配守卫
    let num = Some(4);
    match num {
        Some(n) if n < 5 => println!("小于5的数: {}", n),
        Some(n) => println!("数字: {}", n),
        None => println!("无"),
    }

    // @ 绑定
    let msg = Message::ChangeColor(255, 128, 0);
    match msg {
        Message::ChangeColor(r @ 200..=255, g, b) => {
            println!("高红色值 r={}: RGB({}, {}, {})", r, r, g, b);
        }
        Message::ChangeColor(r, g, b) => {
            println!("普通颜色: RGB({}, {}, {})", r, g, b);
        }
        _ => {}
    }
}
