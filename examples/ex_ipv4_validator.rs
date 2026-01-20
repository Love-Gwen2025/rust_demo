// ============================================
// Demo: IPv4 地址合法性验证
// 运行: cargo run --example ipv4_validator
// ============================================

use std::net::Ipv4Addr;
use std::str::FromStr;

fn main() {
    // 测试用例
    let test_cases = vec![
        "192.168.1.1",     // ✓ 合法
        "255.255.255.255", // ✓ 合法
        "0.0.0.0",         // ✓ 合法
        "192.168.01.1",    // ✗ 非法 - 前导零
        "256.1.1.1",       // ✗ 非法 - 超出范围
        "192.168.1",       // ✗ 非法 - 不足4段
        "192.168.1.1.1",   // ✗ 非法 - 超过4段
        "192.168.1.-1",    // ✗ 非法 - 负数
        "192.168.1.a",     // ✗ 非法 - 非数字
        "192.168.1.1 ",    // ✗ 非法 - 尾部空格
        " 192.168.1.1",    // ✗ 非法 - 前导空格
        "",                // ✗ 非法 - 空字符串
        "192.168.1.001",   // ✗ 非法 - 前导零
    ];

    println!("========== 方法一：使用标准库（推荐）==========\n");
    for ip in &test_cases {
        let result = validate_ipv4_stdlib(ip);
        println!(
            "{:20} => {}",
            format!("\"{}\"", ip),
            if result { "✓ 合法" } else { "✗ 非法" }
        );
    }

    println!("\n========== 方法二：手动解析（面试常考）==========\n");
    for ip in &test_cases {
        let result = validate_ipv4_manual(ip);
        println!(
            "{:20} => {}",
            format!("\"{}\"", ip),
            if result { "✓ 合法" } else { "✗ 非法" }
        );
    }
}

// ============================================
// 方法一：使用标准库（最简单、最推荐）
// ============================================
fn validate_ipv4_stdlib(ip: &str) -> bool {
    // 标准库会自动处理大部分情况
    // 但注意：标准库允许前导零（如 "192.168.01.1"），
    // 如果题目要求不允许前导零，需要额外检查
    Ipv4Addr::from_str(ip).is_ok()
}

// ============================================
// 方法二：手动解析（面试常考，展示 Rust 能力）
// ============================================
fn validate_ipv4_manual(ip: &str) -> bool {
    //要求没有前导零
    //1.按.分割
    //2.检查段数量=4
    //3.检查每段不能为空，长度>1时不能以零开头，检查前导0,检查是否为真数字
    //4.检查每段的数字范围
    let parts: Vec<&str> = ip.split(".").collect();
    if parts.len() != 4 {
        return false;
    }
    for part in parts {
        if part.is_empty() {
            return false;
        }
        if part.len() > 1 && part.starts_with("0") {
            return false;
        }

        if !part.chars().all(|c| c.is_ascii_digit()) {
            return false;
        }
        match part.parse::<u16>() {
            Ok(num) if num <= 255 => continue, // Ok 大写，用 => 不是 ->
            _ => return false,
        }
    }

    true
}

// ============================================
// 方法三：一行搞定（函数式风格）
// ============================================
#[allow(dead_code)]
fn validate_ipv4_oneliner(ip: &str) -> bool {
    ip.split('.')
        .map(|s| {
            // 检查：非空、无前导零、全数字、范围合法
            !s.is_empty()
                && !(s.len() > 1 && s.starts_with('0'))
                && s.chars().all(|c| c.is_ascii_digit())
                && s.parse::<u16>().map_or(false, |n| n <= 255)
        })
        .filter(|&valid| valid)
        .count()
        == 4
}

// ============================================
// 方法四：使用迭代器 + try_fold（更 Rusty）
// ============================================
#[allow(dead_code)]
fn validate_ipv4_rusty(ip: &str) -> bool {
    let parts: Vec<&str> = ip.split('.').collect();

    if parts.len() != 4 {
        return false;
    }

    parts
        .iter()
        .try_fold((), |_, part| {
            // 1. 非空
            // 2. 无前导零（除了单独的 "0"）
            // 3. 全数字
            // 4. 0-255 范围
            let valid = !part.is_empty()
                && !(part.len() > 1 && part.starts_with('0'))
                && part.chars().all(|c| c.is_ascii_digit())
                && part.parse::<u8>().is_ok(); // u8 自动限制 0-255

            if valid { Some(()) } else { None }
        })
        .is_some()
}
