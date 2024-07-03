//hash map and Binary tree map

use std::collections::{hash_map, HashMap};
use std::ptr::hash;

fn main() {
    let mut hmap = HashMap::new();

    hmap.insert("first", 27);
    hmap.insert("second", 11);
    hmap.insert("third", 2002);
    //insert retursn the old value if it exists
    println!("the hashmap : {:?}", hmap);

    println!("the third value is {}", hmap["third"]);

    println!("contain key ? {}", hmap.contains_key("fourth"));

    println!("get value of key index {:?}", hmap.get("first"));

    let removed = hmap.remove("first");
    println!("the removed value is {:?} and the new map is {:?}", removed, hmap);

    let removed = hmap.remove_entry("third");
    println!("the removed value is {:?} and the new map is {:?}", removed, hmap);

    hmap.clear();
    println!("is map empty ? :{}", hmap.is_empty());

}
