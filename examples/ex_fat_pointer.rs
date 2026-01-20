fn main() {
    // ============================================
    // 第一部分：验证胖指针的大小
    // ============================================
    println!("========== 胖指针大小验证 ==========\n");

    // &str 是胖指针，占 16 字节 (ptr + len)
    println!("&str 大小: {} 字节", std::mem::size_of::<&str>());

    // 普通指针只有 8 字节
    println!("*const u8 大小: {} 字节", std::mem::size_of::<*const u8>());

    // ============================================
    // 第二部分：字符串切分时的底层变化
    // ============================================
    println!("\n========== 字符串切分底层变化 ==========\n");

    let original = "Hello, World!";

    // 获取原始字符串的指针地址
    let original_ptr = original.as_ptr();
    println!("原始字符串: \"{}\"", original);
    println!("原始指针地址: {:p}", original_ptr);
    println!("原始长度: {}", original.len());

    println!("\n--- 切分操作 (split by ',') ---\n");

    // 切分字符串
    let parts: Vec<&str> = original.split(',').collect();

    for (i, part) in parts.iter().enumerate() {
        let part_ptr = part.as_ptr();
        // 计算相对于原始字符串的偏移量
        let offset = part_ptr as usize - original_ptr as usize;

        println!("切片 {}: \"{}\"", i, part);
        println!("  - 指针地址: {:p}", part_ptr);
        println!("  - 相对偏移: {} 字节", offset);
        println!("  - 切片长度: {}", part.len());
        println!();
    }

    // ============================================
    // 第三部分：手动切片操作
    // ============================================
    println!("========== 手动切片操作 ==========\n");

    let s = "Hello";
    let slice = &s[1..4]; // "ell"

    println!(
        "原字符串: \"{}\" (地址: {:p}, 长度: {})",
        s,
        s.as_ptr(),
        s.len()
    );
    println!(
        "切片 [1..4]: \"{}\" (地址: {:p}, 长度: {})",
        slice,
        slice.as_ptr(),
        slice.len()
    );
    println!(
        "指针偏移: {} 字节",
        slice.as_ptr() as usize - s.as_ptr() as usize
    );

    // ============================================
    // 第四部分：可视化内存布局
    // ============================================
    println!("\n========== 内存布局可视化 ==========\n");
    println!("原始数据（只有一份，在内存中）:");
    println!(
        "地址:   {:p}  {:p}  {:p}  {:p}  {:p}",
        s.as_ptr(),
        unsafe { s.as_ptr().add(1) },
        unsafe { s.as_ptr().add(2) },
        unsafe { s.as_ptr().add(3) },
        unsafe { s.as_ptr().add(4) }
    );
    println!("内容:   [H]     [e]     [l]     [l]     [o]");
    println!("索引:    0       1       2       3       4");
    println!();
    println!(
        "s = \"Hello\"  →  胖指针 {{ ptr: {:p}, len: 5 }}",
        s.as_ptr()
    );
    println!(
        "slice = \"ell\" →  胖指针 {{ ptr: {:p}, len: 3 }}",
        slice.as_ptr()
    );
    println!();
    println!("✓ 两个胖指针指向【同一块内存】的不同位置");
    println!("✓ 切片操作 = 创建新的胖指针，【不复制数据】");
}
