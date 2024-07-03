use std::collections::{binary_heap, BinaryHeap};

fn main() {
    let mut bheap = BinaryHeap::new();

    bheap.push(11);
    bheap.push(27);
    bheap.push(1);
    bheap.push(2002);
    bheap.push(5);

    //always the biggest in in front
    println!("the binary heap : {:?}", bheap);

    //always pops the bigger
    for i in 1..=5{
        println!("heap pop {:?}", bheap.pop());
    }
    // we can use peek() returns values without popping it
 }
