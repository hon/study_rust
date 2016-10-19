//http://rustbyexample.com/fn/closures/input_parameters.html
fn apply<F>(f: F) where
    F: FnOnce(){
    f();
}
pub fn input_param() {
    let greeting = "hello";
    // let farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);
    };
    apply(diary);
}
