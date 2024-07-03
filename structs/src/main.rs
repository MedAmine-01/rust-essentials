/*
there is 3 types of structs
    - a name field : gives a name to each component
    - a tuple like struct : identifies components by the order they appear
    - a unit like struct : has no components
 */

use std::ops::Range;

// name field struct
//camel case
struct User{
    active : bool,
    username : String,
    sign_in_count : u32,
}

//tuple like Struct
struct Coordinates(i32, i32, i32);


//unit like Struct
struct UnitStruct;


fn main() {
    let user1 = User{active: true, username:String::from("Amine"), sign_in_count: 2711};
    println!("first user data: username: {}, active: {}, sing in count: {}", user1.username, user1.active, user1.sign_in_count);
    let user2 = build_user("Eya".to_string());
    println!("second user data: username: {}, active: {}, sing in count: {}", user2.username, user2.active, user2.sign_in_count);

    //tuple like Struct
    let coord1 = Coordinates(27, 11, 2002);
    println!("this is a tuple like Struct {}-{}-{}", coord1.0, coord1.1, coord1.2);

    //unit like Struct  example (1..5) / .. is shorthand for struct value Range{start:1, end:5}
    

}


//compiler is smart enough to know that username param value will be stored in username in User
fn build_user (username: String) ->User{
    User{
        username,
        active:false,
        sign_in_count:0,
    }
}