fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    //let r3= 0x3cbf57faf4 as *const i32;//eq segfault

    println!("r2 is {:?}", r1);
    println!("r1 is {:?}", r2);
    //we can print the pointer value however we can dereference it
    //to do so we have to use unsafe block

    unsafe {
        println!("r2 is {:?}", *r1);
        println!("r1 is {:?}", *r2);
       //println!("r3 is {:?}", *r3);//error
    }

}
