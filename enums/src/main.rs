enum Pet{Dog, Cat, Fish}

impl Pet {
     fn what_am_i(&self)->&'static str{
         match self{
             Pet::Dog => "i am a dog",
             Pet::Cat => "i am a cat",
             Pet::Fish =>"i am a fish",
         }
     }
}

enum IpAddressType {
    V4,
    V6,
}

impl IpAddressType {
    fn what_am_i(&self)-> &str{
        match self {
            IpAddressType::V4 => "v4",
            IpAddressType::V6 => "v6",
        }
    }
}

struct IPAddress{
    addrtype : IpAddressType,
    address: String,
}

fn main() {
    let pet = Pet::Dog;
    println!("what am i ? : {}",pet.what_am_i());
    let pet = Pet::Cat;
    println!("what am i ? : {}",pet.what_am_i());
    let pet = Pet::Fish;
    println!("what am i ? : {}",pet.what_am_i());


    let home = IPAddress{addrtype: IpAddressType::V4, address:String::from("127.0.0.1")};
    let loopback = IPAddress {addrtype: IpAddressType::V6, address: String::from("::1")};
    println!("the home address is {} and its values is {}", home.addrtype.what_am_i(), home.address);
    println!("the loopback address is {} and its values is {}", loopback.addrtype.what_am_i(), loopback.address);



    //enum with data type

    let root = IpAddressWithDataType::V4(String::from("192.168.0.1"));
    println!("the root address is {} and its values is {}", root.what_am_i(), root.get_value());



    //options
    let _some_number = Some(5);
    let _some_string = Some("this is a string");
    let _nothing : Option<i32> =None ;


    let x : Option<i32> = Some(5);
    let y :i32 = 18;
    let sum = add_option_int(x, y);
    println!("the sum is {}", get_value_from_option(sum));


    //default value

    what_pet("Dog");
    what_pet("Bird");


    //if let : equivalent to match
    let dog2 = Some(Pet::Dog    );
    if let Some(Pet::Dog) = dog2 {
        println!("this is a dog");
    }
    else {
        println!("this is not a dog");
    }

    let  mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop(){
        println!("{}", top);
    }

    //other match functionalities
    let x =3 ;
    match x{
        1|2 => println!("one or two"),
        _ => println!("not one or two"),
    }

    match x {
        1..=5 => println!("In range"),
        _ => println!("not in range"),
    }

    let x = Some(5);
    let y = 5;

    match x {
        Some(10)=> println!("she's a 10!"),
        Some(x) if x == y => println!("the values equals to y"),
        _ => println!("none of the above"),
    }

}


//enum with data type (let's take the example of ip address again with string type)
enum IpAddressWithDataType{
    V4(String),
    _V6(String),
}

impl IpAddressWithDataType{
    fn what_am_i(&self)-> &str{
        match self {
            IpAddressWithDataType::V4(_) => "v4",
            IpAddressWithDataType::_V6(_) => "v6",
        }
    }
    fn get_value(&self) -> &str{
        match self {
            IpAddressWithDataType::V4(value) => value.as_str(),
            IpAddressWithDataType::_V6(value) => value.as_str(),
        }
    }
}


//option : either nothing or something of generic type T
// rust doesn't have NULL but None keyword is used instead
//option is already implemented as well as some
/*
    enum option <T> {
        None,
        some(T),
    }
 */

fn add_option_int (x : Option<i32>, y: i32) -> Option<i32>{
    match x {
        None => None,
        Some(value) => Some(value+y),
    }
}

fn get_value_from_option (x: Option<i32>) -> i32{
    match x {
        None=> -1,
        Some(value)=>value,
    }
}

fn what_pet(input :&str){
    match input {
        "Dog" => println!("i am a dog"),
        "Cat" => println!("i am a cat"),
        "Fish" => println!("i am a fish"),
        _ => println!("i have no clue on what i am"),
    }
}