
struct Rectangle{
    height : u32,
    width : u32,
}

impl Rectangle {
    fn area(&self)->u32{
        self.height * self.width
    }
    fn get_width(&self)->u32{
        self.width
    }
    fn get_height(&self)->u32{
        self.height
    }
    //variable has to be mutuable
    fn set_width(&mut self, new_width :u32){
        self.width = new_width;
    }
    fn set_height(&mut self, new_height :u32){
        self.height = new_height;
    }
}

fn main() {
    let mut rc = Rectangle{height: 25, width : 20};
    let area = rc.area();
    println!("the height, width and area of the rectangle is {}, {} and {}",rc.get_height(),rc.get_width() ,area);
    rc.set_width(15);
    rc.set_height(30);
    println!("the height, width and area of the rectangle is {}, {} and {}",rc.get_height(),rc.get_width() ,rc.area());

}
