fn main() {
    let mut nums: Vec<i32> = vec!{};
    
    nums.push(1);
    nums.push(2);
    nums.push(3);
    nums.push(4);
    
    let pop_val = nums.pop();
    // As the pop methods returns a generic collection we need to iterate the return value when printing.
    println!("{:?}", pop_val); // Returns Some<4>
    println!("{:?}", nums);
    
    let copy_val = nums.first();
    println!("{:?}", copy_val);
    println!("{:?}", nums);
    
    // let mut_val = nums.first_mut();
    
    println!("{}", nums.len());
    println!("{}", nums.is_empty());
    
    nums.insert(0, 10); 
    nums.insert(3, 12);
    nums.insert(2, 25);
    
    println!("{:?}", nums);
    nums.remove(3);
    println!("{:?}", nums);
    nums.sort();
    println!("{:?}", nums);
}

 