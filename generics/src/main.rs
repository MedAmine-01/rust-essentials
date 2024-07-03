struct Point<T, U>{
    x:T,
    y:U,
}

//traits

trait  Overview {
    //default implementation
    fn overview(&self)-> String{
        String::from("this is a Rust Course")
    }
}

struct Course {
    headline: String,
    author : String,
}

impl Drop for Course{
    fn drop(&mut self){
        println!("we're dropping {}", self.author);
    }
}

/*
    trait Clone : sized{
        fn clone(&self)->self;
        fn clone_from(&mut self, source : &self){
            *self = source.clone()
        }
     }
 */

//copy clone from into tryFrom tryInto

struct AnotherCourse {
    _headline: String,
    _author : String,
}

impl Overview for Course{
    fn overview(&self) -> String {
        format!("{}, {}", self.headline, self.author)
    }
}

impl Overview for AnotherCourse{}

fn main() {
    let coord1 = Point{x: 5.0, y: 6.5};
    let coord2 = Point{x: 'x', y: "6.65"};
    println!("the first coords are {} and {}", coord1.x, coord1.y);
    println!("the second coords are {} and {}", coord2.x, coord2.y);


    let course1 = Course{headline : String::from("headline"), author : String::from("amine")};
    let course2 = AnotherCourse{_headline : String::from("another headline"), _author : String::from("another amine")};

    println!("{}", course1.overview());
    println!("{}", course2.overview());

    call_overview(&course1);
    call_overview(&course2);

}//course1 went out of scope here so drop is called


/*
fn call_overview(item: &impl Overview){
    println!("overview {}", item.overview());
}
*/

    fn call_overview <T: Overview> (item: &T){
    println!("overview {}", item.overview());
}

//fn call_overview (item1: &impl Overview + anotherOverview)
//fn call_overview<T:Overview + anotherOverview> (item1 : &T)