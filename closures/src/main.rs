#[derive(Debug)]
struct City{
    city: String,
    population: u64,
}

fn sort_by_population(city : &mut Vec<City>){
    //the key is extracted from the City each time we need to compare the values
    city.sort_by_key(population_helper);
}

fn population_helper(city :&City) -> u64{
    city.population

}

fn sort_by_population_closure(city : &mut Vec<City>){
    //the key is extracted from the City each time we need to compare the values
    city.sort_by_key(|p| p.population);
}

fn main() {
    let  ariana = City{city: String::from("Ariana"), population: 114486};
    let  tunis = City{city: String::from("Tunis"), population: 638845};
    let  kelibia = City{city: String::from("Kelibia"), population: 58524};
    let  sfax = City{city: String::from("Sfax"), population: 330440};

    let mut tunisia = vec![ariana, tunis, kelibia, sfax];
    //sort_by_population(&mut tunisia);
    sort_by_population_closure(&mut tunisia);
    println!("{:?}", tunisia);


    //Closure Annotation
    let add = |x: i32|-> i32 {x+1};
    let add2 = |x| x+1;

    println!("add and add2: {}, {}", add(2), add2(4));

    //closures are mean to be fast theyre not allocated on the heap unless you put them inside
    //a container like a vector

    /*
    let a=4;
    let b=5;
    println!("a.cmp(b) {:?} | b.cmp(a) {:?}",a.cmp(&b), b.cmp(&a))

    */
}

