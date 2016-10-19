//! 格式化输出
//! 定义在 std.fmt

// 1. format! 将格式化文本写入String
// 2. print! 和format!一样，不过将结果打印到屏幕
// 3. println! 和print!一样，不过会换行打印


pub fn run() {

    //位置
    println!("{0} this is {1}, {1} this is {0}.", "Bob", "Jo");

    //名称
    println!("{subject} {verb} {object}.",
        object = "the lazy dog",
        subject = "The quick brown fox",
        verb = "jumps over"
    );

    //类型
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
}
