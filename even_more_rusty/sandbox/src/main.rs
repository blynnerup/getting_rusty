fn main() {
    let my_str = String::from("I got owned");
    println!("{}", my_str);
    takes_ownership(my_str);
    //println!("{}", my_str); fails as the ownership of the value of my_str has been moved to the called function
    
    let val = 1;
    println!("{}", val);
    make_copy(val);
    println!("{val} still here"); // Works
    
    let return_str = give_ownership();
    println!("{return_str}");
    
    let s = String::from("This is my string");
    let x = MyString{fd: s.as_str()};
}

struct MyString <'a>{
    fd: &'a str,
}

fn takes_ownership(s: String) {
    println!("{} anew", s);
}

fn make_copy(i: i32){
    println!("{} copied", i);
}

fn give_ownership() -> String {
    "given".to_string()
}
