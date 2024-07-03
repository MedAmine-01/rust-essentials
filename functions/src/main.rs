fn main() {
    println!("the gcd of 20 and 5 is {}",gcd(20, 5));
    println!("true flag {}",multiple_return_values(true));

    //conditions if else if else
    let one =15 ;
    if one == 1{
        println!("Equal");
    }
    else if one < 1{
        println!("lesser");
    }
    else {
        println!("greater");
    }

    //infinte loop
    /*
    loop{
        print!("loop")
    }

     */

    let mut num = 0;
    'counter: loop{
        println!("num {}",num);
        let mut decrease=6;
        loop{
            println!("Decreasing: {}", decrease);
            if num == 4 {
                break 'counter;
            }
            if decrease ==3{
                break;
            }
            decrease-=1;
        }
        num +=1;
    }

    //while

    num = 0 ;
    while num<5{
        num+=1;
        println!("Num {}", num);
    }
    let vec:Vec<i8> = (0..10).collect();
    for element in vec{
        print!("{} ", element);
    }
    println!();
    for number in (1..4).rev(){
        print!("{} ",number);
    }
    println!();
}



//snake_casing_is_default_casing_for_functions
fn gcd(mut a:i32, mut b:i32) -> i32{
    while a!=0{
        if a<b{
            let c=a;
            a=b;
            b=c;
        }
        a = a % b;
    }
    //this is how to return a value no semicol.
    b
}

//multiple return values depends on logic
fn multiple_return_values(flag : bool)-> bool{
    if flag==true{
        true
    }
    else{
        false
    }
}