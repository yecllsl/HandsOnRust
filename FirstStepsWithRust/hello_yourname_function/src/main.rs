use std::io::stdin;

/// 从标准输入读取用户名称并返回
///
/// # 示例
/// ```
/// let name = what_is_your_name();
/// println!("Hello, {}!", name);
/// ```
///
/// # 错误处理
/// 如果读取输入失败，会触发 panic 并显示 "Failed to read line" 错误信息
///
/// # 返回值
/// 返回包含用户输入名称的 String 对象，包含换行符
fn what_is_your_name() -> String {          // (1) 函数定义：返回String类型的函数
    let mut your_name = String::new();      // (2) 创建一个可变的空字符串，用于存储用户输入
    
    // 从标准输入读取一行数据
    stdin()
        // 将读取的数据存储到your_name变量中
        .read_line(&mut your_name)
        // 如果读取失败，输出错误信息并终止程序
        .expect("Failed to read line");
    
    your_name                               // (3) 返回用户输入的姓名（包含换行符）
}

// 主函数，程序的入口点
fn main() {
    // 打印提示信息，询问用户姓名
    println!("Hello, what's your name?");
    
    // 调用what_is_your_name函数获取用户输入的姓名
    let name = what_is_your_name(); // (4) 将函数返回值存储在name变量中
    
    // 使用格式化输出，将获取到的姓名插入到问候语中并打印
    println!("Hello, {}", name);
}