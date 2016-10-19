//! 注释相关学习
//! 参考地址：
//! 1. http://rustbyexample.com/hello/comment.html
//! 2. https://doc.rust-lang.org/book/comments.html

//! 1. 常规注释，会被编译器忽略
//! - // 行注释
//! - /**/块注释
//! 2. 文档注释，支持markdown语法, 会被解析成html文档
//! - /// 注释程序里的某个功能
//! - //! 对模块，包，函数等整体进行注释，通常放在文件顶部进行注释

//! 生成文档:  https://doc.rust-lang.org/book/documentation.html
//! rustdoc

//! 类似语法 宏


pub fn run() {
    println!("{:?}", "comment");
}
