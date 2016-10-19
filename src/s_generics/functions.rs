struct A;           //concrete type A
struct S(A);        //concrete type S
struct SGEN<T>(T);  //generic type SGEN

fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGEN<A>) {}
fn gen_spec_i32(_s: SGEN<i32>) {}
fn generic<T>(_s: SGEN<T>) {}

pub fn gen_fn() {
    reg_fn(S(A));  //传入S结构体，成员为A
    gen_spec_t(SGEN(A));
    gen_spec_i32(SGEN(5));
    generic::<char>(SGEN('c'));  //强制泛型类型，如果SEGN中的成员不是char, 会报类型匹配错误
    generic(SGEN("test"));   //没有强制类型
    println!("{:?}", "generic functions");
}
