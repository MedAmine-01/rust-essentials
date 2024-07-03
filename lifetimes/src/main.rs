
struct MyString<'a> {
    text:  &'a str,
}

fn main() {
    /*
        we have to explicitly annouce the lifetime so that the instance of MyString
        does not have a dangling reference to str1 (outlives the refrence it holds)
     */

    let str1 = String::from("hello world");
    let x = MyString{text: str1.as_str()};
    println!("the string is {}", x.text);


    // a static lifetime means that the variable have the lifetime of the entire program
    // it is saved in the program binary
    let s : &'static str = "i have a static lifetime";

}



//lifetime annotation indicates that x must live as long as the generic annotation `a
//this indicates the lifetime of the returned values must match the lifetime of parameter variable
fn _example<'a>(x: &'a str) -> &'a str  {
    x
}//in case of multiple parameter we can have `a `b `c ...