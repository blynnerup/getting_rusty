
fn main() {
    let val1: i32 = 5;
    let val2: i32 = 2;
    let ans: i32;
    
    ans = val1 % val2;
    println!("{}", ans);
    
    let mut vec: Vec<i32> = vec![2,4,6,8,10];
    println!("{:?}", vec);
    vec.pop();
    vec.push(12);
    println!("{:?}", vec);
    
    let str: String = String::from("Hello");
    println!("{}", concat_string(str));
    
    control_flow(1);
    control_flow(34);
    control_flow(70)
}

fn concat_string(string: String) -> String {
    string + " World!"
}

fn control_flow(i: i32) {
    if i == 1 { println!("The value is one") }
    else if i > 50 { println!("The value is greater than 50") }
    else if i < 25 && i > 1 { println!("The value is less than 25") }
    else if i < 50 && i > 25 { println!("The value is greater than 25 but less than 50") }
    else { println!("There was a value") }
}
