#![allow(unused)]

use std::collections::HashMap;
mod leetcode;

fn main() {
    let mut map: HashMap<char, i32> = HashMap::new();
    let string = "aab";
    for ch in string.chars() {
        map.entry(ch).and_modify(|c| *c = *c + 1).or_insert(1);
    }

    println!("{:#?}", map);
}
