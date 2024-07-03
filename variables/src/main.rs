fn main() {
    let mut x=5;
    println!("this is x {}", x);
    //variable has to be mutable to change it
    x=6;
    println!("this is x {}", x);
    const  EULER: f32 = 2.71;
    const PI: f32 = 3.14;
    println!("this is euler number minus pi{}",EULER-PI);

    //there are 4 Scalar Data types : Integers, float, booleans, chars

    //Integers can be 8, 16, 32, 64, 128 bit.
    let a: i8 = -10;
    println!("this is 8 bit number {}",a);
    let b: u8 = 10;
    println!("this is 8 unsigned bit number {}",b);

    //Integer literals
    let decimal = 2_55;
    let hex = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;
    let byte =b'A';
    println!("this is dec: {}, this is hex: {} this is octal: {}, this is binary: {}",decimal, hex, octal, binary);
    println!("this is a byte {}",byte);

    //Booleans
    let t:bool= true;
    let f = false;
    println!("this is true: {}, and this is false {}", t, f);

    //character
    let c:char = 'e';
    println!("this is a char {}",c);

    //operators + - * / %
    let k =10;
    let j = 4;
    let remainder = k%j;
    println!("the remainder of {} divided by {} is {}", k, j, remainder);

    //tuples
    let tup = (27, "amine", true);
    println!("the tuple is {}, {}, {}", tup.0, tup.1, tup.2);
    let(x, y, z)=tup;
    println!("x={}, y={}, z={}",x, y, z);

    //arrays
    let mut array:[i32;3] = [27, 11, 2002];
    println!("these are the array elements {}, {}, {}", array[0], array[1],array[2]);
    array[0]=01;
    println!("these are the array elements {}, {}, {}", array[0], array[1],array[2]);
    //you cannot access array[3] rust compiler will throw an error either at compile time or panick at runtime (array[j] j>=3)

    //vectors are dynamic sized we use vec! macro to define a vector
    let mut vect1 = vec![1, 2, 3];
    vect1.push(27);
    //:? formats vect1 to printable
    println!("this is a vect1or after push {:?}",vect1);
    vect1.pop();
    println!("this is a vect1or after pop{:?}",vect1);
    //we can use vector constructor instead of vec! marcro
    let mut vect2 = Vec::new();
    vect2.push("first");
    vect2.push("second");
    vect2.push("third");
    vect2.reverse();
    println!("this is second vect1 {:?}",vect2);
    //the capacity is expandable
    let vect3 = Vec::<i32>::with_capacity(12);
    println!("this is third vector capacity {}",vect3.capacity());
    //define vector with itterator
    let vect4: Vec<i32>= (0..5).collect();
    println!("this is a vector with iterator definition {:?}", vect4);

    //slices : they cannot be directly stored in variable but they point directly to a part of data in memory and it's non owning refrence
    let  svect4:&[i32]=&vect4[2..5];
    println!("this is a slice of vector4 {:?}", svect4);
    //a refrence is a non owning pointer to a single value and a slice is non owning pointer of a range consecutive values
    //a slice can be used with functions that operates on either array or vector

    //string are similar to vector of bytes the difference is string is garentee its UTF format
    //string allocated on heap is global and non null terminated
    let name = String::from("Amine");
    let famname = "Belhaj".to_string();
    let new_name = name.replace("A","MedA");
    println!("old name is {:?}, family name is {:?} and new name is {:?}", name, famname, new_name);
    //string slice doesn't allocate memory in the heap but we can flip flop between string slice and string
    let str1 = "Hello world";//&str
    //convert string slice to string and vice versa
    let str2=str1.to_string();
    let str3= &str2;
    println!("string and string slice str1: {}, str2: {}, str3: {}",str1, str2, str3);
    //we can compare strings == !=
    println!("str1 == str3 ? :{}",str1==str2);

    //string literals
    let rust = "\x52\x75\x73\x74";
    println!("string literals: {}",rust);
}
