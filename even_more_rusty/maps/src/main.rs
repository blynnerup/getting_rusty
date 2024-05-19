use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    println!("{:?}", map); // Prints {"two": 2, "three": 3, "one": 1}
    println!("{}", map.contains_key(&"two"));
    
    let kv = map.remove_entry(&"three");
    println!("{:?}", kv);
}
