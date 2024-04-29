fn main() {
   mutable_reference();
    test_unwrap();
    
    let robot = Robot {};
    robot.run();
}

fn mutable_reference() {
    let mut s1 = String::from("abc");
    do_stuff_to_mut(&mut  s1);
    println!("{}", s1);
}

fn do_stuff_to_mut(s: &mut String) {
    s.insert_str(0, "Hi, ");
    *s = String::from("Replacement");
}

fn test_unwrap () {
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4, "Test {}", k);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
}

trait Run {
    fn run(&self) {
        println!("I'm running!");
    }
}

struct Robot {}

impl Run for Robot {}


