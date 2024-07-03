use std::collections::{hash_set, HashSet};


fn main() {
    let mut hset = HashSet::new();


    hset.insert(11);
    println!("the size of h set is {}", hset.len());
    hset.insert(2002);
    println!("the size of h set is {}", hset.len());
    hset.insert(27);
    println!("the size of h set is {}", hset.len());
    hset.insert(2002);
    println!("the size of h set is {}", hset.len());
    hset.insert(1);
    println!("the size of h set is {}", hset.len());
    //hset.remove(&   2002);
    //println!("the size of h set is {}", hset.len());

    for x in hset.iter(){
        println!("Iter: {}", x);
    }

    let mut hset2 = HashSet::new();
    hset2.insert(2002);
    hset2.insert(11);
    hset2.insert(6);
    hset2.insert(12);

    println!("the intersection values");
    for x in hset.intersection(&hset2){
        println!("Iter: {}", x);
    }

    //another way to do intersection
    let intersection = &hset & &hset2;
    for x in intersection{
        println!("short hand way : {}", x);
    }

    println!("the union values");
    for x in hset.union(&hset2){
        println!("Iter: {}", x);
    }

    //another way to do union
    let union = &hset | &hset2;
    for x in union {
        println!("Iter: {}", x);
    }

    println!("the difference values");
    for x in hset.difference(&hset2){
        println!("Iter: {}", x);
    }

    //another way to do intersection
    let intersection = &hset - &hset2;
    for x in intersection{
        println!("short hand way : {}", x);
    }


}
