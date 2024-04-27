fn main() {
   mutable_reference();
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
