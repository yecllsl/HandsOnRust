use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {    // (1) 定义Visitor结构体的实现块使用impl关键字和结构体的名字可以为结构体实现(implement)函数。
    fn new(name: &str, greeting: &str) -> Self {    // (2) 实现一个名为new的构造函数，用于创建Visitor实例这是一个关联函数(associated function)。这个关联函数的参数列表中没有self，因此开发者不能通过name.new()这样的写法来访问new函数。但是，这个关联函数在结构体的命名空间中是可用的，开发者可以通过Visitor::new()的方式来调用它。
        // (3) 使用提供的name和greeting参数初始化Visitor实例的字段缺失的分号表明这是一个“隐式返回”的语法。它创建了结构体的一个新实例，此处同样使用Self代替了Visitor。结构体中的每一个字段都必须以field_name:value的形式给出。
        Self {  //首字母大写的Self指代结构体类型本身，小写的self指代结构体的实例。
            /* 这个函数接收的参数类型是&str，但是存储的数据却是String类型。
            to_lowercase和to_string这两个函数实现了其中的转换。通过接收&str类型
            的参数，这个构造函数可以在不进行转换的情况下，直接将字符串字面量作为
            自己的参数。这样，在调用构造函数时，开发者就不用自己书写
            类似String::from("bert")这样的代码了。 */
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }

// 定义一个方法来欢迎访客
// 该方法属于某个结构体或类（根据上下文），不需要返回值
fn greet_visitor(&self) { // (4)这个函数是一个成员函数(member function)，或者称为方法。它接收self作为参数，当通过一个结构体的实例来调用这个函数时（例如my_visitor.greet_visitor()）​，当前这个结构体实例就会自动被作为self参数传递到这个函数中。
    // 输出问候语
    println!("{}", self.greeting);
}
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name.trim().to_lowercase()
}

fn main() {
    /* 这仍然是一个数组，但是它现在包含的元素是通过刚才编写的构造函数创建出的Visitor结构体，而不是字符串。
    现在你仍然可以通过索引序号来访问独立的元素。此时，visitor_list[0].name包含“bert”​。
    英文句号(.)表示“成员访问”​，visitor_list[0]代表一个Visitor实例。英文句号授予了开发者访问字段和函数
    的权限。 */
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve","Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();

    let known_visitor = visitor_list// (5)把迭代器函数调用链的最终结果赋值给known_visitor变量。
        .iter()// (6)使用iter()创建一个包含visitor_list中所有数据的迭代器。
        .find(|visitor| visitor.name == name);// (7)find()会运行一个闭包(closure)。如果闭包的返回值是true，那么find()会返回匹配的结果。最后的分号用来表示语句的结束。
    match known_visitor {// (8)在此处列出希望匹配的变量。
        Some(visitor) => visitor.greet_visitor(),// (9)Some(visitor)可以检查Option类型中是否有数据，如果有，则将其中包含的内容放到一个叫作visitor的局部变量中，使其对当前匹配分支中的代码可见。胖箭头(“=>”)表示了匹配成功以后需要执行的代码——在这个例子中是给访客打招呼。不同的匹配分支之间通过逗号分隔。
        None => println!("You are not on the visitor list. Please leave.")// (10)None表示Option类型中没有数据的情况——find函数没有在名单中找到访客的名字，因此门卫需要礼貌地请访客离开。
    }
}
