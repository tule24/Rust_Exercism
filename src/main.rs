use std::borrow::Borrow;


fn check<T: Borrow<str>>(s: T) {
    println!("{:?}", s.borrow());
}
fn main() {
    let s = "abc";
    check(s);
    let s = String::from("zyc");
    check(s);
}

