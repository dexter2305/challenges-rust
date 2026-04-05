#![allow(unused)]

fn main() {
    let mut v: Vec<u32> = Vec::with_capacity(20);
    v[2] = 10;

    println!("{:?}", v);
}
