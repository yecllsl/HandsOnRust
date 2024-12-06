use std::io::stdin;

#[derive(Debug)]//Rust的大多数内置类型，例如String，都原生支持多种派生特性。因此，一旦在结构体上派生了Debug，你就可以在println!中使用{:?}占位符来打印整个结构体。
struct Visitor {
    name: String,
    greeting: String
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet_visitor(&self) {
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
    let mut visitor_list = vec![//vec!使得开发者可以采用和初始化数组类似的语法来初始化向量：
        Visitor::new("Bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("Steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("Fred", "Wow, who invited Fred?"),
    ];

    loop {//不断重复执行，直至遇到break
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
        let name = what_is_your_name();

        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {    // (1)检查用户输入的姓名是否为空，如果是空的，就用break跳出循环。
                    break;  // (2)break语句使程序直接跳到loop语句块的结尾。
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, "New friend"));
                }
            }
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
