/*in terms of memory we want give our memory back when needed and we never want to reference
memory once we have given it back


in rust each variable has its owner and when the owner goes out of scope the value will be
dropped (aka freed)
 */

fn main() {

    let _var = 1;//pushed into the stack
    let mut s = "hello".to_string();//created on the heap
    s.push_str(" world");//s is allocated on heap it can grow

    //moving ownership of value from one variable to another ; most types implement a move
    let x = vec!["amine".to_string()];
    let y = x;
    //println!("{:?}",x); error
    let z = y;
    //println!("{:?}",y); error
    println!("{:?}",z);

    //instead of moving a value we can clone it :
    //a clone can be an expensive operation if the data is big
    let x = vec!["amine".to_string()];
    let y = x.clone();
    let z = y.clone();
    println!("the value of x, y and z are {:?}, {:?} and {:?}",x, y, z);

    //some other data types implement a copy (those who are stored in stack : integer , boolean, float, char)
    //A tuple can also implement a copy if all the values in it implement a copy
    let x = 1;
    let y = x;
    println!("the value of x and y are {} and {}", x, y);


    //how move works
    let s:String =  String::from("takes");
    take_ownership(s);//give ownership to function
    //println!("{}",s); error ownership is taken by function

    let val=1;
    make_copy(val);
    println!("after copy {}",val);

    let s1 = give_ownership();
    println!("{}", s1);
    let s2 = take_and_give_ownership(s1);
    println!("{}", s2);



    //a reference allow us to reference to a value without taking a ownership of it
    //two types of references shared and mutable
    //shared lets you read only and you can have multiple shared references to a value
    //mutable references allow you to manupilate the value but you can have only one mut ref to a certain value

    let mut s = String::from("hello ");
    change_string(&mut s);
    println!("{}", s);
}
// here var is dropped , s is dropped

fn change_string(param : &mut String){
    param.push_str("world");
}

fn take_ownership(s:String){
    let ch = s;
    println!("{}",ch);
}

fn make_copy(val:i32){
    let v = val;
    println!("the copy {}", v);
}

fn give_ownership()-> String{
    "given".to_string()
}

fn take_and_give_ownership (stri :String)->String{
    stri
}
