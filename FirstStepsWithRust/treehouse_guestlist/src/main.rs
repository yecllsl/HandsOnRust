use std::io::stdin;

/// 询问并返回用户输入的名字
///
/// 该函数会提示用户输入名字，并返回一个经过处理的字符串
/// 具体处理步骤包括：
/// 1. 创建一个空字符串以存储用户输入的名字
/// 2. 从标准输入读取用户输入的名字
/// 3. 移除输入名字前后的空白字符，并转换为小写
///
/// 返回值:
/// 返回一个经过处理的字符串，表示用户输入的名字
fn what_is_your_name() -> String {
    // 创建一个空字符串以存储用户输入的名字
    let mut your_name = String::new();

    // 从标准输入读取用户输入的名字
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");

    // 移除输入名字前后的空白字符，并转换为小写
    your_name.trim().to_lowercase()
}

// 定义主函数
fn main() {
    // 定义访客列表
    let visitor_list = ["bert", "steve", "fred"];

    // 打印欢迎信息
    println!("Hello, what's your name?");
    // 调用函数获取用户输入的名字
    let name = what_is_your_name();

    // 初始化允许进入的标志为false
    let mut allow_them_in = false;
    // 遍历访客列表
    for visitor in &visitor_list {
        // 如果列表中的访客名字与输入的名字相同，允许进入
        if visitor == &name {
            allow_them_in = true;
        }
    }

    // 根据允许进入的标志打印相应的信息
    if allow_them_in {
        println!("Welcome to the Treehouse, {}", name);
    } else {
        println!("Sorry, you aren't on the list.");
    }

}
