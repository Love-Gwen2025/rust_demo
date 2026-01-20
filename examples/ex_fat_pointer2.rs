fn main() {
    // ============================================
    // 字符串字面量 vs String 的内存位置
    // ============================================

    // 1. 字符串字面量 - 在静态区（编译时就确定）
    let s1: &str = "hello";
    println!("s1 (&str 字面量):");
    println!("  地址: {:p}", s1.as_ptr());
    println!("  长度: {}", s1.len());
    println!("  生命周期: 'static（程序运行期间一直存在）");

    // 2. String - 在堆上（运行时分配）
    let s2: String = String::from("hello");
    println!("\ns2 (String 堆分配):");
    println!("  地址: {:p}", s2.as_ptr());
    println!("  长度: {}", s2.len());
    println!("  生命周期: 跟随变量作用域");

    // 3. 观察地址差异
    println!("\n========== 地址分析 ==========");
    println!("s1 地址: {:p}  ← 静态区（地址较小，固定）", s1.as_ptr());
    println!("s2 地址: {:p}  ← 堆区（地址较大，动态）", s2.as_ptr());

    // 4. 证明字面量是静态的
    let s3: &'static str = "hello"; // 显式标注 'static 生命周期
    println!("\ns3 的类型是 &'static str，证明字面量是静态的！");

    // 5. 图解
    println!("\n========== 内存布局图解 ==========");
    println!(
        r#"
    栈:
    ┌─────────────────────┐
    │ s1: &str            │
    │   ptr ──────────────┼────→ 静态区: "hello" (只读)
    │   len: 5            │
    ├─────────────────────┤
    │ s2: String          │
    │   ptr ──────────────┼────→ 堆: "hello" (可修改)
    │   len: 5            │
    │   cap: 5            │
    └─────────────────────┘
    "#
    );
}
