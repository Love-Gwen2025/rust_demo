// ============================================
// Demo 03: 所有权 (Ownership) - Rust 核心概念！
// 运行: cargo run --example 03_ownership
// ============================================

fn main() {
    // ========== 所有权规则 ==========
    // 1. 每个值都有一个所有者
    // 2. 同一时刻只能有一个所有者
    // 3. 所有者离开作用域时，值被丢弃

    // ---------- 移动 (Move) ----------
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("{}", s1);  // 错误！s1 已失效
    println!("s2 = {}", s2);

    // ---------- 克隆 (Clone) ----------
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝
    println!("s3 = {}, s4 = {}", s3, s4); // 两个都有效

    // ---------- Copy trait ----------
    // 基本类型（整数、浮点、布尔、字符）实现了 Copy trait
    let n1 = 42;
    let n2 = n1; // 这是复制，不是移动
    println!("n1 = {}, n2 = {}", n1, n2); // 两个都有效

    // ---------- 函数与所有权 ----------
    let s = String::from("ownership");
    takes_ownership(s); // s 的所有权移动到函数
    // println!("{}", s);  // 错误！s 已失效

    let x = 5;
    makes_copy(x); // x 被复制
    println!("x 仍然有效: {}", x); // x 仍然有效

    // ---------- 返回值与所有权 ----------
    let s5 = gives_ownership(); // 函数返回值移动给 s5
    println!("获得所有权: {}", s5);

    let s6 = String::from("接力");
    let s7 = takes_and_gives_back(s6); // s6 移入，返回值移给 s7
    // println!("{}", s6);  // 错误！
    println!("接力后: {}", s7);
}

fn takes_ownership(some_string: String) {
    println!("接管: {}", some_string);
} // some_string 在这里被 drop

fn makes_copy(some_integer: i32) {
    println!("复制: {}", some_integer);
}

fn gives_ownership() -> String {
    String::from("我是新创建的")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回，所有权移出
}
