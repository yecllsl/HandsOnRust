// 导入标准库中的stdin模块，用于从标准输入读取数据
use std::io::stdin;

// 主函数，程序的入口点
fn main() {
    // 打印提示信息，询问用户姓名
    println!("Hello, what's your name?");
    
    // 创建一个可变的空字符串变量，用于存储用户输入的姓名
    let mut your_name = String::new();
    
    // 从标准输入读取一行数据
    stdin()
        // 将读取的数据存储到your_name变量中
        .read_line(&mut your_name)
        // 如果读取失败，输出错误信息并终止程序
        .expect("Failed to read line");
    
    // 使用格式化输出，将用户输入的姓名插入到问候语中并打印
    println!("Hello, {}", your_name)
}