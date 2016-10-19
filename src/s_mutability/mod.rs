//! 可变性 rust语言的变量绑定默认是“不可变”的。决定可变与否是由rust语言的理念决定的：内存安全，所有权系统和
//! 借贷检查。

fn base() {
    // let x = 5;
    // x = 6;  //error,

    //可以通过mut关键字进行可变绑定
    let mut x = 5;
    x = 6;  //此处只是将x绑定到了另外一个值，而没有修改x绑定的资源
    println!("{:?}", x);
}


//注意！ mut 是模式的一部分
fn part_of_pattern() {
    let (x, y) = (10, 5);
    println!("{:?}, {}", x, y);
}



// 隐士可变

pub fn run() {
    base();
    part_of_pattern();
    // ref_base();
    // println!("可变性");
}
