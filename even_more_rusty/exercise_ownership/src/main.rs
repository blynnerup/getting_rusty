
fn main() {
    // Pass vector value to function and test first value
    let mut test_vec: Vec<i32> = vec![1,3,5,7];
    let is_one = first_is_one(&mut test_vec);
    println!("{is_one}");
    test_vec.push(15);
    println!("{is_one}");
    
    // Test if reference is needed in order to retain ownership of i8 in main when passing value to function.
    let test_i: i8 = 2;
    add_two(test_i);
    println!("{test_i}");
}

fn first_is_one(vec: &mut Vec<i32>) -> bool {
    if vec[0] == 1 { true }
    else { false } 
}

fn add_two(i: i8){ // No reference is needed since it's a copy
    let _ = i + 2;
}
