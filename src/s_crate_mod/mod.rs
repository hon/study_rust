//! 包和模块

/// crate 是rust中的包，类似node里面的package
/// crate 包名命名规范为小写的蛇形，这样在引入的时候就不会报错。如果
/// 导入包 use crate cratename
/// 安装包：cargo install cratename, 但是这只是安装二进制包，如果要安装开发包，需要修改Cargo.toml文件
/// 包地址 http://crate.io


/// module 是程序员的逻辑模块，将相关的逻辑分成文档，进行归类，最后组成组合成程序或者包
/// 导入模块
/// use modulename

/// crate root 指的是src/lib.rs
/// module root 指的是src/module/mod.rs(相当于nodej里的index.js)


/// 经常会出现模块找不到的情况可能是：
/// 1. 名字拼写错误
/// 2. 没有模块文件
/// 3. lib.rs里没有导出

/// 路径
/// use 的情况下, 即使用use 导入模块的时候
/// 默认是crate root, 如：use english 导入lib.rs 中可访问的包 english, 类似于: '/'
/// use self::english 表示相对本身的包路径，类似于 '.'
/// use super::english 表示本身父级包路径，类似于 '../'

/// use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy

/// 非use的情况下， 即调用函数的时候
/// 默认是相对当前模块, foo::bar() 调用foo模块下的bar函数
/// 访问上一级 super::foo::bar()
/// 调用根模块中函数 ::foo::bar()

// 以下代码假设在main.rs中运行
// fn root_mod_fn() {
//     print!("{:?}", "root_mod_fn");
// }
//
// mod mod1 {
//     pub mod mod2 {
//         pub fn mod2_fn(){
//             println!("{:?}", "mod2_fn");
//         }
//         pub mod mod3 {
//             pub fn mod3_fn() {
//                 super::mod2_fn();
//                 ::root_mod_fn();
//             }
//         }
//     }
// }



pub fn run() {
    println!("{:?}", "Crate and module.");
}
