//marcos are essentially a way of writing code that writes other code (meta programming)
macro_rules! gcd {
    ($a: expr , $b: expr) => {
        {
            let mut m = $b;
            let mut n = $a;

            while m !=0 {
                if m<n{
                    let t = m;
                    m = n ;
                    n = t;

                }
                m = m % n ;
            }
            n
        }
    };
}

macro_rules! op {
    ($a: expr, $b: expr, $c: expr) => {
        match $c {
            '+'=> $a+$b,
            '-'=> $a-$b,
            '*'=> $a*$b,
            '/'=> $a/$b,
            '%'=> $a%$b,
            _ => -1,
        }
    };
}

fn main() {
    println!("{}", gcd!(27, 90));
    println!("{}", op!(27,11,'+'));
    println!("{}", op!(27,11,'-'));
    println!("{}", op!(27,11,'*'));
    println!("{}", op!(27,11,'/'));
    println!("{}", op!(27,11,'%'));
}
