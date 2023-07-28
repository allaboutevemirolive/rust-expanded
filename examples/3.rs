// https://users.rust-lang.org/t/what-is-the-meaning-of-a-a-in-rust-lifetime-parameters/53570

#![allow(unused)]
fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    // let pf = f::<'static> as fn();
    let pg = g::<'static> as fn();
    //print!("{}", pf == pg);
}