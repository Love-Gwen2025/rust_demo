macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("函数 {:?} 被调用", stringify!($func_name));
        }
    };
}

// 使用宏创建多个函数（在 main 外部，模块级别）
create_function!(foo); // 创建了一个名为 foo 的函数
create_function!(bar); // 创建了一个名为 bar 的函数
create_function!(hello); // 创建了一个名为 hello 的函数

fn main() {
    // 调用宏创建的函数
    foo();
    bar();
    hello();
}
