use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let mut map = HashMap::new();
    
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    
    println!("{:?}", map); // Prints {"two": 2, "three": 3, "one": 1}
    println!("{}", map.contains_key(&"two"));
    
    let kv = map.remove_entry(&"three");
    println!("{:?}", kv);
    
    let mut bt_map = BTreeMap::new();
    
    bt_map.insert(4, 4);
    bt_map.insert(5, 5);
    bt_map.insert(6, 10); 
    
    println!("{:?}", bt_map); // Prints {4: 4, 5: 5, 6: 10}
    println!("{}", bt_map.contains_key(&4)); // Prints true
    
    let bt_kv = bt_map.remove(&5);
    println!("{:?}", bt_kv); // Prints Some(5)
    
    println!("{}", bt_map.contains_key(&6)); // Prints true
    
}
