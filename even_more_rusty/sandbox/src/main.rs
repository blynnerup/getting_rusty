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

    let map1 = TacticalMap{ map_name: String::from("Braunsmark"), map_id: 23 };
    let map2 = SeaChart { map_name: String::from("Veiled Sea"), map_id: 42 };
    
    call_overview(&map1);
    call_overview(&map2);
    
    drop(map1);
    call_overview(&map1);
}

struct MyString <'a>{
    fd: &'a str,
}

trait Overview {
    fn overview(&self) -> String {
        String::from("This the overview message")
    }
}

struct TacticalMap {
    map_name: String,
    map_id: i32,
}

struct SeaChart {
    map_name: String,
    map_id: i32,
}

impl Overview for TacticalMap {
    fn overview(&self) -> String {
        format!("{}, {}", self.map_name, self.map_id)
    }
}

impl Drop for TacticalMap{
    fn drop(&mut self){
        println!("Dropping: {}", self.map_name)
    }
}

impl Overview for SeaChart { }

fn takes_ownership(s: String) {
    println!("{} anew", s);
}

fn make_copy(i: i32){
    println!("{} copied", i);
}

fn give_ownership() -> String {
    "given".to_string()
}

fn call_overview(item: &impl Overview){
    println!("Overview: {}", item.overview())
}
