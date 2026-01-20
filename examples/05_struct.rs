// ============================================
// Demo 05: 结构体 (Struct)
// 运行: cargo run --example 05_struct
// ============================================

// ---------- 定义结构体 ----------
#[derive(Debug)] // 允许使用 {:?} 打印
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// ---------- 元组结构体 ----------
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// ---------- 单元结构体 ----------
#[allow(dead_code)] // 允许未使用的结构体
struct AlwaysEqual;

// ---------- 方法与关联函数 ----------
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 关联函数（类似静态方法），用于构造
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    // 方法（第一个参数是 &self）
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 可变方法
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    // ---------- 创建结构体实例 ----------
    let user1 = User {
        email: String::from("test@example.com"),
        username: String::from("rust_learner"),
        active: true,
        sign_in_count: 1,
    };
    println!("用户名: {}", user1.username);
    println!("用户信息: {:?}", user1);

    // ---------- 可变实例 ----------
    let mut user2 = User {
        email: String::from("user2@example.com"),
        username: String::from("mutable_user"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("new_email@example.com");

    // ---------- 结构体更新语法 ----------
    let user3 = User {
        email: String::from("user3@example.com"),
        ..user2 // 其他字段从 user2 取
    };
    println!("user3: {:?}", user3);

    // ---------- 元组结构体 ----------
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("黑色 RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);

    // ---------- 方法调用 ----------
    let rect = Rectangle::new(30, 50);
    println!("矩形: {:?}", rect);
    println!("面积: {}", rect.area());

    let square = Rectangle::square(10);
    println!("正方形面积: {}", square.area());

    println!("rect 能容纳 square 吗? {}", rect.can_hold(&square));

    let mut growing_rect = Rectangle::new(5, 5);
    growing_rect.scale(3);
    println!("放大后: {:?}, 面积: {}", growing_rect, growing_rect.area());
}
