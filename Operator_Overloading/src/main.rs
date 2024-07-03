use std::ops::Add;


fn print<T : std::fmt::Debug> (value :T){
    println!("{:?}", value);
}


#[derive(Debug)]

struct Point <T>{
    x:T,
    y:T,
}

impl <T> Add for Point<T>
    //this where restrict T to types can be added to themselfs yielding another T value
    where
    T: Add<Output=T>{
        type Output = Self;
        fn add(self, rhs: Self)->Self{
            Point{
                x:self.x + rhs.x,
                y:self.y + rhs.y,
            }
        }
    }

fn main() {
    let coord1= Point{x: 5.0, y:6.0};
    let coord2= Point{x: 2.0, y:12.0};

    let sum = coord1 + coord2 ;

    println!("the sum is {}, {}", sum.x, sum.y);
    println!("the summ is {:?}", sum);  //#[derive(Debug)]
}
