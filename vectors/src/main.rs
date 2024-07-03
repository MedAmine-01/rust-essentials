use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut nums : Vec<i32> = vec![];

    nums.push(1);
    nums.push(2);
    nums.push(3);

    let pop=nums.pop();//option<T> , return none or some(T)
    println!("popped values: {:?}", pop);

    let x = nums[1]; //copy
    //&nums[0] refrence
    let y = &nums[0];
    println!("x: {}", x);
    println!("y: {}", y);

    let one = nums.first(); //.last .first_mut .last_mut (borrow mutuable reference)
    println!("one {:?}", one);

    println!("length: {}", nums.len());
    println!("is empty? :{}", nums.is_empty());

    nums.insert(0, 19);
    nums.insert(3, 27);
    nums.insert(2, 11);

    println!("the vector after insert: {:?}", nums);

    nums.remove(3);

    println!("the vector after remove: {:?}", nums);

    nums.sort();

    println!("the vector after sort: {:?}", nums);

    nums.reverse();

    println!("the vector after reverse: {:?}", nums);

    //thread random number generator
    nums.shuffle(&mut thread_rng());

    println!("the vector after shuffle: {:?}", nums);
}
