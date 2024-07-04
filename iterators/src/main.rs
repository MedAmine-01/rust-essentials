#[derive(Debug)]
struct Item{
    name:String,
}

struct Range{
    start: u32,
    end: u32,
}

impl Iterator for Range{
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start>=self.end{
            return None;
        }
        let result = Some(self.start);
        self.start +=1 ;
        return result;
    }
}

fn check_inventory(items : Vec<Item>, product: String) -> Vec<Item>{
    items.into_iter().filter(|i| i.name==product).collect()
}

fn main() {
    let vec = vec![1, 2, 3, 4];
    for value in vec.iter(){
        println!("{}", value);
    }

    let vec2 = vec![1, 2, 3, 4];
    let mut iter = (&vec2).into_iter();
    while let Some(v)= iter.next(){
        println!("{:?}", v);
    }

    let mut vec_item = Vec::new();
    vec_item.push(Item{name : String::from("Shoes")});
    vec_item.push(Item{name : String::from("Coat")});
    vec_item.push(Item{name : String::from("T-shirt")});
    vec_item.push(Item{name : String::from("Socks")});

    let checked = check_inventory(vec_item, String::from("Coat"));

    println!("{:?}", checked);

    let mut range = Range{start:0, end:30};

    for r in range {
        println!("the value of the range iterator is {}", r);
    }

    let mut range = Range{start:0, end:30};
    let vect_iter= range.filter(|x| x%2==0);
    let vect :Vec<u32> = vect_iter.collect();
    println!("{:?}", vect);
  /*  for r in vect_iter{
        println!("the even numbers are {}", r);
    }


*/
    println!("TESSSSSSSSSSt");

    let odd_vect : Vec<u32> = (1..=9).filter(|x| x%2 ==1).collect();
    println!("{:?}", odd_vect);
    let multiplied_odd_vect:Vec<u32> = odd_vect.into_iter().map(|x| 10*x).collect();
    println!("{:?}", multiplied_odd_vect);



}




/*
pub trait Iterator{
    type Item;
    fn next(&mut self)->Option<Self::Item>;
}
*/