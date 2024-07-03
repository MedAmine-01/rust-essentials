use std::fs::File;
use std::io;
use io::ErrorKind;

fn main() {
   // panic!("panicked Here");

    /*
    let vec = vec![1];
    vec[10];
    */


    let file = File::open("error.txt"); // returns Result
    let file = match file {
        Ok(file)=>file,
        Err(error)=> match error.kind() {
            ErrorKind::NotFound => match File::create("error.txt") {
                Ok(file_created)=>file_created,
                Err(err)=> panic!("cannot create file"),
            },
            _ => panic!("it is some other error kind"),
        },
    };

    let file = File::open("error2.txt").expect("Error opening the file!!!!");




}



/*

implemented in io

enum Result<T, E>{
    OK(T),
    Err(E),
}

 */