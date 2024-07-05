use std::sync::{Arc, Mutex};

//thread panicked when holding the lock => mutex is poisoned
fn main() {
    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    let _ = std::thread::spawn(move || {
       let guard = lock2.lock().unwrap();
        panic!(); //mutex is poisoned
    }).join();

    //let mut guard = lock.lock().unwrap();

    let mut guard = match lock.lock(){
        Ok(guard)=> guard,
        Err(poisoned) => poisoned.into_inner(),//call into_inner to recover the mutex and the data inside of it
    };


    //data is no longer poisoned
    *guard +=1;
    println!("the value is {:?}", guard) ;
}
