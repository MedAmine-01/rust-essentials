use std::sync::{Arc, Mutex};

fn main() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..8 {
        //each thread clones a reference to the mutex
        let count = Arc::clone(&counter);
        //create a thread
        let handle = std::thread::spawn(move || {
           //each thread takes a key if available
            let mut num = count.lock().unwrap();
            //let mut num2 = count.lock().unwrap(); //dead lock
            *num +=1;
        });//num/MutexGuard goes out of scope -> lock is given up
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("{}", counter.lock().unwrap());
}
