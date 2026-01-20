// ============================================
// Demo 04: 引用与借用 (References & Borrowing)
// 运行: cargo run --example 04_references
// ============================================

fn main() {
    // ---------- 不可变引用 ----------
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 传递引用，不转移所有权
    println!("'{}' 的长度是 {}", s1, len); // s1 仍然有效！

    // 可以有多个不可变引用
    let r1 = &s1;
    let r2 = &s1;
    println!("多个不可变引用: r1={}, r2={}", r1, r2);

    // ---------- 可变引用 ----------
    let mut s2 = String::from("hello");
    change(&mut s2); // 传递可变引用
    println!("修改后: {}", s2);

    // 同一时间只能有一个可变引用
    let r3 = &mut s2;
    // let r4 = &mut s2;  // 错误！不能同时有两个可变引用
    r3.push_str("!!!");
    println!("追加后: {}", r3);

    // ---------- 引用的规则 ----------
    // 1. 在任意给定时间，要么只能有一个可变引用
    //    要么只能有多个不可变引用
    // 2. 引用必须总是有效的（不能悬垂）

    // ---------- 引用的作用域 ----------
    let mut s3 = String::from("scope");
    let r5 = &s3;
    let r6 = &s3;
    println!("不可变引用: {} and {}", r5, r6);
    // r5 和 r6 的作用域到这里就结束了（NLL - Non-Lexical Lifetimes）

    let r7 = &mut s3; // 这里可以创建可变引用了
    r7.push_str(" demo");
    println!("可变引用: {}", r7);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s 离开作用域，但因为它不拥有所有权，所以不会 drop

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
