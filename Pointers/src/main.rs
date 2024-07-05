use std::cell::RefCell;
use std::rc;
use std::rc::Rc;

struct Flagger{
    //you can use refcell without the Rc
    boolean : Rc<RefCell<bool>>,
}

fn main() {
    let t = (13, "Eggs");//created on the stack
    let b = Box::new(t);//created on the Heap, but b was stored on the stack and point to the address of the heap
    println!("{:?} {:?}", b, t);


    let  x = 5;
    let  y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);


    let  x = 5;
    let  y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("{:?}", y);

    //in case of graph multiple nodes can own the address of one node so we need shared ownership
    //we use rc : reference counting

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();

    //reference counter=3 and s1, s2 and s3 are allocated on the stack and refer to the same string on the heap
    //we can use string method directly on s1, s2, s3
    println!("{}", Rc::strong_count(&s1));
    println!("{} {} {}", s1, s2, s3);

    //Refcell make imut variable mut in runtime
    let rfc = Flagger{boolean : Rc::new(RefCell::new(false))};


    //borrow
    /*
    let reference = rfc.boolean.borrow();
    println!("{}", reference);
    */

    //mut
    let mut reference = rfc.boolean.borrow_mut();
    *reference = true;
    println!("{}", reference);
    //we can't use both unless we use Rc to allow multiple borow or else runtime error
}
