pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    
    #[test]
    fn if_fails() {
        panic!("TEST FAILED!")
    }
    
    #[test]
    fn call_simple_add() {
        assert!(simple_add())
    }
}   

fn simple_add() -> bool {
    if 2 + 2 == 4 { true }
    else { false }
}