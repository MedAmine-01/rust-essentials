pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    //user everything outside of mod tests{}
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert_eq!(result, 4);
    }

    #[test]
    //#[ignore] // ignored
    #[should_panic] // expected to panic and test pass
    fn it_fails(){
        panic!("test failed");
    }

    #[test]
    fn call_simple_add(){
        assert!(simple_add());
    }

    #[test]
    fn call_assert_not_equale(){
        assert_ne!(4,5);
    }
}

fn simple_add()-> bool{
    if 2+2 == 4 {
        true
    }
    else {
        false
    }
}

//cargo test it (test it_works it_fails_
//cargo test call_simple_add
//...