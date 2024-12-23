use bracket_lib::prelude::*;

struct State {}

// 实现GameState trait，以定制游戏循环中的每一帧更新
impl GameState for State {
    // GameState这个trait要求开发者实现一个名为tick()的函数，且该函数的签名要和这里给出的一样。
    //&mut self的写法将允许tick函数访问并修改State类型的实例。ctx参数提供了一个窗口，
    //用于和当前正在运行的bracket-terminal交互——可以通过它来获取鼠标的位置以及键盘输入，
    //也可以给窗口发送绘图命令。你可以把tick()函数看作连接游戏引擎和游戏程序本身的“桥梁”​。
    fn tick(&mut self, ctx: &mut BTerm) {
        // ctx（​“context”的简写）提供了一些函数用来和游戏显示窗口进行交互。cls()的作用是清空
        //显示窗口。在大多数帧的绘制流程中都要先清空屏幕，这样可以避免上一帧的遗留数据被渲染出来。
        ctx.cls();
        // print()提供了在游戏窗口中打印文本的接口。它和前几章中用过的println!类似，但是只能
        //接收字符串，不能接收格式模板。不过，你还是可以通过这种方法来格式化
        //字符串：ctx.print(format!("{}",my_string))。
        ctx.print(1, 1, "Hello, Bracket Terminal!");
    }
}


fn main() -> BError {
    let context = BTermBuilder::simple80x50() // (5)以请求构造一个80×50大小的终端窗口作为起点。
        .with_title("Flappy Dragon") // (6)请求将窗口的标题设置为“Flappy Dragon”​。
        .build()?; // (7)调用build()函数来完成初始化，并且将返回的上下文对象存储到context变量中。之所以能够使用?操作符是因为main()函数的返回值已经被改成了BError类型。

    main_loop(context, State{})
}
