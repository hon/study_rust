//! 闭包
//! 闭包是匿名函数，和它在定义时所处的环境的组合体。它能够在外层作用域结束后，继续访问环境里的变量。
//! 闭包的多次调用会实例化多个环境。

mod input_param;

use self::input_param::*;


//语法
fn syntax() {
    let plus_one = |x| x + 1;  //竖线表示管道
    assert_eq!(2, plus_one(1));
}

//与函数定义的区别
// 1. 没有fn关键字
// 2. 不强制对参数和返回值进行类型声明（函数需要），这一点主要是从人体工程学角度考虑的。函数中强制类型声明是
// 为了文档化和类型推导, 闭包通常不需要文档化，因为他们是匿名的。

// 闭包和环境
// 闭包的环境包含它外层的变量绑定，本身的参数内部的局部变量绑定。
//  by default closures capture variables by reference
fn closure_env() -> i32 {
    let num = 5;  // <- 外层变量绑定
    let plus_num = |x| {  // <- 参数
        let y = 10;  // <- 局部变量绑定
        x + num + y
    };

    plus_num(5)

}

fn move_env() {
    let nums = vec![1, 2, 3];

    //nums's ownership is moved into closure
    let takes_nums = || &nums;
    takes_nums();
    println!("{:?}", nums);
}


fn test_mut() {
    let mut num = 5;
    let mut inc = || {
        num += 1;
        num
    };
    println!("{:?}", inc());
}



pub fn run() {

    use std::mem;

    let movable = Box::new(5);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    test_mut();

    move_env();
    syntax();
    input_param();
    println!("{:?}", closure_env());
    println!("Closure end");
}
