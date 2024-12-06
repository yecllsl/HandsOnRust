use std::io::stdin;

#[derive(Debug)]    // (1)枚举体和结构体一样，都可以通过派生宏来获得某些能力。此处在派生宏中指定Debug能力，这将使得Rust的字符串格式化功能可以把枚举体的值以名称的形式显示出来。
enum VisitorAction {  // (2)通过enum关键字来声明一个新的枚举体，声明的语法和其他类型的声明一模一样。
  Accept,         // (3)Accept是一个简单枚举项，它没有与之关联的数据。可以用let visitor_action=VisitorAction::Accept;这样的语法为VisitorAction类型的变量赋值。
  AcceptWithNote { note: String },    // (4)AcceptWithNote枚举项包含附加数据：一个名为note的字符串变量。对于该枚举项，你可以用let visitor_action=VisitorAction::AcceptWithNote{ note: "my note".to_string() };这样的语法为其赋值。
  Refuse,
  Probation,
} // (5)和结构体的声明一样，枚举体声明不需要分号来标记结束。


#[derive(Debug)]
struct Visitor {
  name: String,
  action: VisitorAction,    // (6)定义了一个名为action的字段，它的类型是刚刚声明的VisitorAction枚举类型。结构体中的字段可以是任意类型的，包括枚举体和其他结构体。
  age: i8                // (7)定义了一个名为age的字段，它的类型是i8。这是一个8-bit的有符号整数，意味着它可以表示−128和127之间的数字。Rust还提供了i32和i64类型，可用于表示更大范围的整数，但是似乎鲜有年龄超过127岁的人会来树屋。
}

impl Visitor {
  fn new(name: &str, action: VisitorAction, age: i8) -> Self { // (8)这个函数遵循构造函数的模式，可接收用于描述结构体内容的参数，并且返回Self类型。
    Self {
      name: name.to_lowercase(), // (9)先列出字段的名称，然后是一个冒号，位于最后面的是字段的值。to_lowercase()把静态字符串(&str)转换为String，同时将其全部转为小写。
      action, // (10)当开发者不需要对数据进行额外调整，且函数中存在和结构体字段同名的变量时，字段名和冒号可以省略——Rust会使用和结构体字段同名的变量的值来初始化对应的结构体字段。
      age
    }
  }
  fn greet_visitor(&self) {
    match &self.action {
      VisitorAction::Accept => println!("Welcome to the tree 
          house, {}", self.name), // (11)如果action的值等于VisitorAction::Accept，那么门卫会向用户打招呼。
      VisitorAction::AcceptWithNote { note } => { // (12)如果正在匹配的可选项包含额外数据，就把字段名写在花括号中。这样的语法称为解构(destructuring)。这样，在当前匹配分支的作用域中就可以通过字段名来访问数据了。注意，这里并没有采用单一表达式的写法，而是用了一个具有作用域的代码块。在match语句中，开发者可以使用这两种写法中的任何一种。
          println!("Welcome to the treehouse, {}", self.name);
          println!("{}", note); // (13)note变量是从上一行的模式匹配里抽取出来的，在这里当作一个局部变量来使用。
          if self.age < 21 { // (14)整数类型的数学运算和其他编程语言类似：如果年龄小于21岁，则打印一条不提供酒精的警告。
              println!("Do not serve alcohol to {}", self.name);
          }
      }
      VisitorAction::Probation => println!("{} is now a 
          probationary member", self.name),
      VisitorAction::Refuse => println!("Do not allow {} in!", self.name),
      }
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
  let mut visitor_list = vec![
    Visitor::new("Bert", VisitorAction::Accept, 45),
    Visitor::new("Steve", VisitorAction::AcceptWithNote{ 
        note: String::from("Lactose-free milk is in the fridge") 
    }, 15),
    Visitor::new("Fred", VisitorAction::Refuse, 30),
  ];
  loop {
    println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
    let name = what_is_your_name();
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);
    match known_visitor {
      Some(visitor) => visitor.greet_visitor(),
      None => {
        if name.is_empty() {
          break;
        } else {
          println!("{} is not on the visitor list.", name);
          visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
        }
      }
    }
  }
  println!("The final list of visitors:");
  println!("{:#?}", visitor_list);
}
