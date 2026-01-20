fn main() {
    let ip = String::from("192.168.1.1");
    
    // Vec<&str> - 引用，指向原字符串
    let parts: Vec<&str> = ip.split(".").collect();
    
    // 验证：parts[0] 的地址在 ip 内部
    println!("ip 的地址: {:p}", ip.as_ptr());
    println!("parts[0] 的地址: {:p}", parts[0].as_ptr());
    // 输出会显示 parts[0] 的地址就是 ip 开头的地址！
}