use std::thread;
use std::time::Duration; //to allow main thread to sleep

use std::sync::mpsc;


fn main() {

    let handle = std::thread::spawn(move || {
        println!("hello world from thread");
    });
    //thread::sleep(Duration::from_secs(1));//sleep the main thread so it doesn't exit before the other thread is spawed

    //better solution than sleep
    handle.join().unwrap();//blocking until thread waiting to be joined terminates
    println!("hello from main");

    let v = vec![1, 2, 3];

    //move allow to force closure to take owenrship of values it uses
    let handle = std::thread::spawn(move || {
        println!("{:?}", v);
    });
    handle.join().unwrap();


    //multiple threads
    let v = vec![1, 2, 3];
    let mut thread_handles = Vec::new();

    for e in v{
        thread_handles.push(thread::spawn(move || {println!("Thread {}", e)}))
    }
    println!("main thread");

    for handle in thread_handles {
        handle.join().unwrap();
    }



    //Channels : 2 ends tx and rx.
    let (transmitter, receiver) = mpsc::channel();

    let txmessage = String::from("a message");
    std::thread::spawn(move || {
        transmitter.send(txmessage).unwrap();
    });

    let rxmessage = receiver.recv().unwrap();
    println!("{}", rxmessage);

    //send() takes ownership of value of txmessage then recv() takes the ownership of that value and return it to rxmessage


    //we can have multiple producers

    let (transmitter, receiver) = mpsc::channel();
    let tx = transmitter.clone();
    std::thread::spawn(move || {
        let vec = vec![String::from("Transmitting"), String::from("from"), String::from(("original"))];
        for val in vec {
            transmitter.send(val).unwrap();
        }
    });
    std::thread::spawn(move || {
        let vec = vec![String::from("Transmitting"), String::from("from"), String::from(("clone"))];
        for val in vec {
            tx.send(val).unwrap();
        }
    });
    //the messages are sent and put into Q until the receiver is ready to handle it
    for rec in receiver{
        println!("{}", rec);
    }


    //to avoid wasting memory with long Qs we can use sync
    //Sync channel : we limit the size of the Q . if the Q is full the transmitter will block and wait for space in Q
    let (transmitter, receiver) = mpsc::sync_channel(1000);


    //rc is not safe to use across threades but Arc is
}
