# getting_rusty
Learning the basics of Rust.
This is a helpme guide, used to reference things in the future.
The guide is based on the course: https://www.udemy.com/course/ultimate-rust-crash-course/ and other information I discovered while learning.
Added another project to this one, where I follow along to https://www.udemy.com/course/rust-programming-the-complete-guide/

## Basics
Cargo is the bread and butter. It handles your packages, your build and testing… everything is cargo.

### Basic commands
$ cargo new hello
- Creates a new Rust project called 'hello'
- Creates the Cargo.toml file which is the configuration file
- Creates a subdirectory called src which contains the main.rs file which is the main file for running the project.
- The file contains a single function that prints 'hello world!'

In order to run it call the following in the project directory:
$ cargo run

This builds the project and creates a target folder with all the build artifacts. This should  be ignored by version control as it should not be stored centrally.

$ cargo run --release

This will allow cargo to compile without the debug symbols. This is a lot faster. This will also produce files in the release subdirectory.

## Variables
Variables are declared using the 'let' statement. You can also use a type statement.
It is possible to declare multiple variables at once;

`let (cards, players) = (52, 4);`

This will assign the value 52 to cards and 4 to players.

### Mutable and immutable
Variables are immutable by default in Rust, in order to make them mutable, this needs to be explicitly annotated on the variable.
- This is designed with safety, concurrency and speed in mind.
  - Compiler can make optimizations to variables that it know will not change.

 In order to make a variable mutable use the statement 'mut'
 
`let mut boxes = 5;` <-- this is mutable

### Const
The const keyword is even more immutable. It requires the following;
- The 'const' keyword instead of let
- The naming convention is to use screaming snake casing for consts
- The type annotation is required
- The value must be a constant expression that can be compiled at runtime

`const WARP_FACTOR: f64 = 9.9;`

The reasons behind this is to be able to use a const outside function at module scope and use it anywhere, it is global and immutable.
Const's values are inlined at compile time, they're really fast.

### Scope
Variables are scoped. Meaning they begin where they're created and extends to the end of the block in which they're created. Including nested blocks.
It is important to understand that variables are only available in the scope they're created in. They're dropped as soon as they're out of scope. Rust have no garbage collection. Lucky the compiler declares such problems to us at compile time.

#### Shadow variables
Variables can be shadowed, meaning the same variable name can be declared at different levels of nesting and assume different values.

~~~
fn main(){
  let x = 5;
  {
    let x = 99;
    println!("{}", x); // Prints "99"
  }
  println!("{}", x); // Prints "5"
}
~~~

It is possible to shadow variables in the same scope
~~~
fun main() {
  let mut x = 5; // x is mutable
  let x = x; // x is now immutable
}
~~~
The compiled will optimize away the redundant operation, so it will not take effect.

You can even shadow variable to a different type in the same scope, and discard intermediate representations.
~~~
fun main() {
  let meme = "More cowbell!";
  let meme = make_image(meme);
}
~~~

## Memory Safety
Rust ensures memory safety at compile time. It does this through the borrow system.
This strikes a happy medium between garbage collection and memory access and control. You retain freedom to allocate memory, but Rust will deal with freeing memory.
More on this in the Ownership section.


## Functions
The Rust style guide says to use snake case for function names.
~~~
fun do_stuff() {
}
~~~

Function parameters are specified by name: type and seperated by comma.

The return type is specified by an "arrow" ->

Using the return keyword for the return value. The return value can be shorthanded by omitting the return keyword and semicolon from the last statement in the block, and the compiler will use this as return value, this is also called the tail expression.

~~~
fun do_stuff(qty: f64, litres: f64) -> f64 {
  return qty * litres; // Normal return statement
}

fun do_other_stuff(tank: f64, range: f64) -> f64 {
  tank * range // Short hand return statement
}
~~~

### Macros
Macro names looks like function names, but are always postfix with an exclamation point.
~~~
println!("{}", x); // println! is a macro
~~~

## Scalar Types
There are four scalar types in Rust
- integer
- float
- boolean
- characters

### Integers
There are many integer types;

| Unsigned | Signed |
|----------|--------|
| u8       | i8     |
| u16      | i16    |
| u32      | i32    |
| u64      | i64    |
| u128     | i128   |
| usize    | isize  |

The u- and isize has the size of the platforms integer bits and can be used to access memory. If you don't initialize an integer, it will default to i32, as this is the fastest. The integer types may not be universally supported, it depends on system architecture.

### Integer Literals
| Type           | Specification |
|----------------|---------------|
| Decimal        | 10000         |
| Hex            | 0xdeadbeef    |
| Octal          | 0o71234232    |
| Binary         | 0b 11100110   |
| Byte (u8 only) | b'A'          | 

A feature in Rust is that underscores can be added inside or at the end of an integer to ease readability. For instance 1_000_000 reads as 1000000 when compiling or could be done like so 1_0_0_0_0_0_0_.

### Floats
Float comes as f32 and f64, with 32 and 64 bits of precision respectively. The default value for floats are f64. These can however be really slow on less than 64 bit architecture, so choosing the right size matters.

### Floating point Literals
Follows the IEEE-754 standard, looks like this 3.14159. No special annotation of float is required.

### Numerical Literal Suffixes
A numerical literal can optionally include the type as a suffix.
~~~
let x: u16 = 5;
let y: f32 = 3.14;

let z = 3.14f32; // Includes the float 32 type declaration as a suffix
let xyz = 3.14_f32 // Same as above, but includes underscore for readability
~~~

This can be useful when passing on values to a generic function which accepts multiple literal types as parameters.

### Booleans
The type is specified as _bool_
Booleans are not integers, but can be cast as such.

~~~
true as u8
false as u8
~~~

### Characters
Are a polymorphic type, which can assume many values. It can be a single char, a sign from the japanese alphabet, an emoji, a unicode character and so on. It is always 4 bytes (32 bits) long. Which effectively makes an array of chars a UTF32 string. They're specified using single quotes and are rarely used.

Strings are UTF8 and characters are not, so strings does not use characters internally. 

## Compound Types
Compound types gather multiple values of other types into one type.

### Tuples
Tuples stores multiple values of any types. Creation of tuple is as follows:
~~~
let info: (u8, f64, i32) = (1, 3.3, 666);
~~~

There are two ways of accessing members of a tuple
~~~
let info: (u8, f64, i32) = (1, 3.3, 666);
let jets = info.0;
let fuel = info.1;
let ammo = info.2;

let (jets, fuel, ammo) = info;
~~~
The tuple can contain up to 12 items, after which some functionality of the tuple becomes restricted.

### Arrays
Arrays stores multiple values of the same type.
You can specify them as follows
~~~
let buf = [1, 2, 3];
let nums = [0; 3]; // Creates an array with 3 0s [Value; How Many]
let buffs: [u8; 3] = [1, 2, 3];
~~~

Indexing is done by using square brackets. Arrays are limited to 32 elements, after which they lose some of their functionality. Instead, Vectors can be used.

## Control Flow

### If Expression
In Rust, if is an expression not a statement. Statements don't return values, expressions do. 
Consider the following
~~~
if num == 5 {
  msg = "five";
} else if num == 4 {
  msg = "four";
} else {
  msg = "other";
}

// The above can be rewritten into the following because of if being an expression
msg = if num == 5 {
  "five"
} else if num == 4 {
  "four"
} else {
  "other"
}
~~~

### Unconditional loop
If the compiler knows the loop is unconditional, you can have some nice control features.

Consider the following
~~~
// The simple loop, end with break
loop {
  break;
}

// In case you want to break out of a nested loop, you can label the loop and once the loop hits the break it will break out of that loop
// A label is annotated with a single quote
'outer: loop {
  loop {
    loop {
      break 'outer; // This breaks out of the 'outer loop
    }
  }
}

// Continue by itself continues the current loop, but can also be applied with a label
'best: loop{
  loop {
    continue 'best;
  }
}

// The while loop operates in the same way
while running() {
  // do stuff
  // breaks when the expression wequals true
}

// do while constructs does not exist in Rust, but can manually be implemented

// for loop
for num in [1, 2, 3, 4].iter() {
  // do stuff with num
}

// for loop can take a pattern and destructure the items it receives and bind the inside parts to variables
let array = [(1, 2), (3, 4)];

for (x, y) in array.iter() {
  //do stuff with x and y
}

// loops can work with ranges, which are marked with two dots
// The start is inclusive and the end is exclusive
for num in 0..50 {
  // do stuff with num (0 - 49)
}

// An equal sign can be used to make the last in the range inclusive
for num in 0..=50 {
  // do stuff with num (0-50)
}
~~~

## Strings
In the Rust standard library there are six different types of Strings, however two are most commonly used.

str is a string slice and will often be seen as &str which is the _borrowed string slice_. A literal string is always a borrowed string slice. A borrowed string slice &str is often referred to as a string. This can be confusing as the other commonly used string type is String. The biggest difference between the two is that the data in a borrowed string slice cannot be modified, whereas the String data can. A borrowed string slice is internally made up of a pointer to some bytes and a length. A String is made up of a pointer to some bytes, a length and a capacity, that may be higher than what is currently being used. In other words a borrowed string slice is a subset of a String in multiple ways. Both are valid UTF-8. Strings cannot be indexed by char position.


## <a id="ownership"></a> Ownership
Ownership is a unique feature in Rust, enabling memory safety while still being a systems programming language.
The Rust compiler is going to verify that your program is free of memory safety errors, such as dangling pointers, double freeze, using uninitialized memory.
This concept is what drives the Rust language, as it sought to eliminate some of the pitfalls of languages like c++, where memory handling sometimes becomes cause for runtime errors.

There are three rules to ownership;

1. Each value has an owner. There is no value or data in memory that does not have a variable that owns it.
2. There is only one owner of a value. There is no shared ownership. Other variables may borrow a value.
3. When the owner goes out of scope, the value gets dropped immediately (freeing of memory).

In order to understand how ownership of a value can change, consider the following:

~~~
let s1 = String::from("abc");
let s2 = s1;

println!("{}", s1); // This results in the error seen below as ownership of the data from s1 has been passed to s2 and s1 in now uninitialized.

error[E0382]: borrow of moved value: `s1`                                                                                                                                                                                                                                                                       
 --> src\main.rs:7:20
  |
4 |     let s1 = String::from("abc");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
5 |     println!("{}", s1);
6 |     let s2 = s1;
  |              -- value moved here
7 |     println!("{}", s1);
  |                    ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
6 |     let s2 = s1.clone();
  |   
~~~

The reason behind this is because of memory safety. When s1 is created, a capacity, a size and a pointer to the value of s1 on the heap.
When s2 created and is set equal to the value of s1, the pointer reference is then moved to s2, leaving s1 in the compilers eyes as an initialized variable. Because of the rule about only having one owner.
If both variables could "own" the value, memory safety stops.

Another point of memory allocation is knowing that variables of fixed size gets pushed onto the stack, whereas variables of varying size is placed on the heap.
~~~
let var = 1; // Gets pushed onto the steack as the type of var (i32) is of fixed size)
let mut s = "string with growth potential".to_string(); // Is created on the heap as the String type is undefined in size and can grow and shrink depending on the value stored.
~~~

### Move, Copy and Clone
Move is defined as moving ownership of data from one variable to another. This essentially happens everytime a variable is set equal to the value of another variable.
This was illustrated in the example above where s2 was set equal to s1.\
Another important thing to understand is when Rust uses Copy and Clone.
If both s1 and s2 were supposed to have the same value, s2 could have been initialized as such:
~~~
s2 = s1.clone();
~~~
It is worth noting that clone can be an expensive operation. It is also worth noting that setting a value of a type already on the stack will do a copy and not a move.
This means that that types like integers, booleans, float, char. A tuple can also have the copy _trait_ if every item in the tuple is already on the stack.

When talking about values, Rust _clones_ heap data and pointer updates, but will _copy_ stack data. When a value is dropped Rust performs the following actions:
1. Destructor is run (if one is present)
2. The heap portion is immediately freed
3. The stack portion is immediately popped

This in turn ensures; No leaks (of memory), no dangling pointers (null pointer reference) -> Memory safety.

Another example of moving ownership is when passing a value as parameter to a function.
~~~
let s1 = String::from("abc");
do_stuff(s1);
println!("{}", s1); // Produces an move error, just like above.

fn do_stuff(s: String){
  // do stuff
}

~~~

A way to fix this could be to make s1 mutable and return a value to the variable from the function.
~~~
let mut s1 = String::from("abc");
do_stuff(s1);
println!("{}", s1); // Now prints "abc"

fn do_stuff(s: String) -> String {
  s // returns s as a tail expression
}
~~~
This is however oftentimes not what you want, as the value passed to the function will oftentimes be consumed by the function.
For most other cases you would use references.

## References and Borrowing
Instead of moving a variable, references can be used. To reference a value use _&_. A reference allows to pass that reference to a value without moving it to another variable.
There are two types of references; shared and mutable. References default to an immutable value. However, if we make a mutable reference to a mutable value, then we can use the reference to change the value as well.

~~~
let s1 = String::from("abc");

fn do_stuff(s: &String) {
  // do stuff
}
~~~
When passing in s: &String we pass a reference to the s1 reference, the function do_stuff _borrows_ the value of s1, but s1 retains ownership to the value.
Once the function is finished, the reference to s1 gets dropped, not the value of s1.
When creating a reference, Rust creates a pointer to the value. However, unlike other low level languages like C, in Rust the word pointer is rarely used as Rust handles the creation and destruction of pointers for us.
Rust also makes sure that these pointers are always valid using a concept called _Lifetimes_.

### Lifetime

_Lifetimes_ can be summed up as a rule stating that references must always be valid. This means that the compiler will not allow a value to outlive the reference to it.
And you can never point to _null_. The mean idea behind this is to prevent dangling references. 

It is possible to annotate lifetime for references. This does not change how long a reference will live, but it will help provide some rules that the code needs to live by.
Lifetime annotation is done by a single tick '

Consider the following
~~~
fn example<'a>(x: &'a str) -> &'a str {
    // do stuff
}
~~~
In this example, we say that the function takes a reference with the lifetime a which needs to live as long as the return str value from the function.

An example of when lifetime references are import is seen below
~~~
struct MyString<'a> {
    text: &'a str,
}

fn main() {
    let str1 = String::from("This is my string");
    let x = MyString{text: str1.as_str()};
}
~~~ 
When we create the _MyString_ struct, we say that the _text_ field is a reference to a string. So when creating a new instance of _MyString as we do in the main function. We need to make sure that the lifetime of _str1_ does not outlive the lifetime of _x_, because then we would have a dangling reference.

#### Static lifetime
It is possible to have a static lifetime reference. This will make a reference live as long as the program is running. This can be useful for error messages, but while this can have some advantages, it is in general encouraged to consider if a static lifetime really is needed.\
`let s: &'static str = "I have a static lifetime";`

~~~
let mut s1 = String::from("abc"); // s1 created as mutable
do_stuff(&mut s1); // passing the s1 as a mutable reference
println!("{}"s1); // Now prints "Hi, abc" 

fn do_stuff(s: &mut String) { // funtion accepting a mutable reference to a String
  s.insert_str(0, "Hi, "); // update the value of the string
}
~~~

When dealing with dot operators like in the example above, there is no need to dereference. The dot operator for a method or a field
auto dereferences down to the value of the reference. So as long as we're dealing with dot operators no need to worry about dereferencing.
In cases where you need to dereference, use the _*_ in order to dereference. The dereference operator has pretty low precedence, and putting parentheses around the dereference operator can sometimes be needed.


However, when using most other operators, you will need to manually dereference the reference, if you want to read from or write to the actual value.
~~~
let mut s1 = String::from("abc"); // s1 created as mutable
do_stuff(&mut s1); // passing the s1 as a mutable reference
println!("{}"s1); // Now prints "Replacement" 

fn do_stuff(s: &mut String) { // funtion accepting a mutable reference to a String
  s.insert_str(0, "Hi, "); // update the value of the string
  *s = String::from("Replacement"); // Change the actual value of s1
}
~~~

So to sum up:
1. _x_ is a variable
2. _&x_ creates an immutable reference to the value of x
3. _&mut x_ creates a mutable reference to the value of x

And the same goes for types (eg. i32, &i32, &mut i32). 

Going the other way around, if your variable _x_ is a mutable reference to a value, then dereferencing _x_ gives you mutable access to that value.
And if the variable is immutable the dereference will immutable as well.
~~~
x: &muti32
*x // a mutable i32

y: &i32
*y // an immutable i32
~~~

As reference is implemented via pointers, Rust uses a special rule to keep everything safe.
You can have either;
1. Exactly one mutable reference to a value
2. Any number of immutable references


## Structs
As Rust follows many of the paradigms of functional programming, Rust doesn't have classes, but structs.
A Struct can have data fields, methods and associated functions. The syntax and implementation of a struct is as follows:

Rust have three types of structs;
1. A named field
   - Gives a name to each component
2. A tuple like
   - Identifies them in the order in which they appear
3. A unit like
   - Has no components at all

Base implementation of them are like follows

~~~
// This is a unit like struct
struct UnitStruct;
// An example of a unit like struct is the Range part of a for loop 1..5 -> this is short for Range {start: 1, end: 5}

// This is a tuple like struct
struct Coordinates(i32, i32, i32);

let coords = Coordinates(1,3,6);

// This is a named field struct
struct RedFox { // name of a struct is in captial camel case.
  enemy: bool,
  life: u8,
}

// The verbose implementation
let fox = RedFox {
  enemy: true,
  life: 70,
}

// Typically a struct would have an associated function, that can be implemented as a constructor. This has assigned default values, and then call that.
// Methods in an associated function is defined in an impl block. This is seperate from the struct definition.
// First is the keyword impl (implement) followed by the name of the struct you wish to implement.
// The new() is an associated function, as it doesn't have a form of Self as its first parameter. The new name is optional, but often used to imply constructor like method.
impl RedFox { // implement RedFox struct
  fn new() -> Self { // Self can be replaced with the name of the struct (RedFox in this case)
    Self {
      enemy: true,
      life: 70,
    }
  }
}

// In order to create a new RedFox, this is how

let fox = RedFox::new();

// Another example

struch Square {
    width: u32,
    height: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn return_width(&self) -> u32 {
        self.width
    }
    
    fn change_width(&mut self, new_width: u32) { // Need to add the mut to the reference of self, as it is n ow being updated
        self.width = new_width
    }
}

fn calc_square() {
    let mut sq = Square { width: 5, height: 5 }; // Has to be mutable in order to allow the function change_width to make changes. Not needed for other operations.
    println!("{}", sq.area()); // Prints out 25
    println!("{}", sq.return_width()); // Prints out 5
    sq.new_width(sq, 7);
}
~~~~

The scope operator in Rust is double colon (::), it is used to access part of namespace like things. It has been used before to access items inside modules.
Once you have an instance, use dot syntax like in other languages `fox.life`. Other methods are also defined in the implementation block. They always take some form of self as their first argument.

~~~
impl RedFox {
  // associated function
  fn funtion() ...
  // methods
  fn move(self) ...
  fn borrow(&self) ...
  fn mut_borrow(&mut self) ...
}
~~~

In Rust there is no inheritance for structs. Instead, Rust have _traits_.

## Traits
A trait represents a capability, something a type can do, that can be shared with other types.
Traits works like interfaces in other languages. Rust takes the composition over inheritance approach. 

Traits define required behaviour. In other words, functions and methods that a struct must implement if it wants to have that trait.

~~~
struct RedFox {
  enemey: bool,
  life: u32,
}

trait Noisy {
  fn get_noise(&self) -> &str;
}

impl Noisy for RedFox { // Implement Noisy for RedFox
  fn get_noise(&self) -> &str { "Meow?" } // The implementation then returns "Meow?"
}
~~~

The method could of course have been implemented directly onto the struct, but when implementing _traits_ we get the added bonus of being able to write generic functions that accepts any value that implements that trait.

For instance, we could make a function that would accept an item of type T, which is defined to be anything that implements the Noisy trait.
The function can then use any behaviour on item that the Noisy trait defines.
`fn print_noise<T: Noisy>(item: T) { println!("{}", item.get_noise());`

Traits can be defined on any struct in the project, including built-ins or types from imported packages. And on your structs you can implement any trait from any package or built-in
~~~
// Here we implement the Noisy trait for the u8 built in type (byte)
fn print_noise<T: Noisy>(item: T) { println!("{}", item.get_noise());

impl Noisy for u8 {
  fn get_noise(&self) -> &str { "BYTE!" }
}

fn main() {
  print_noise(5_u8); // prints "BYTE!"
}
~~~

Traits can inherit from other traits. A struct implementing a trait with inheritance must also implement the trait methods from the parent traits.

### Default behaviour

Traits can have default behaviors, so if design of traits are done carefully, you may not have to implement some of that trait at all.

~~~
trait Run {
  fn run(&self) { // Instead of ending the defintion with a semicolon, make a block ...
    println!("I'm running!"); // .. and implment default behaviour here.
  }
}

// Once implementing the trait, just dont provide a new definition for the method you want to implement
// the precense of an implementation will override the default.

struct Robot {}
impl Run for Robot {}

// In order to make the Robot run, we can instantiate in the main method

fn main() {
  let robot = Robot {};
  robot.run();
}
~~~

There cannot be set Fields in traits. The way around this is to implement getter and setter methods in the traits.

A trait can be passed as a parameter as long as a type implements that trait.
In the bottom of this example is shown how to call the call_overview function with generics.
~~~
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

impl Overview for SeaChart { } // This results in the default behaviour being called

fn main(){
    let map1 = TacticalMap{ map_name: String::from("Braunsmark"), map_id: 23 }
    let map2 = SeaChart { map_name: String::from("Veiled Sea"), map_id: 42 }
    
    call_overview(&map1);
    call_overview(&map2);
}

fn call_overview(item: &impl Overview){
    println!("Overview: {}", item.overview())
}

// These two function calls do the same thing, but with some key differences
// fn call_overview(item1: &impl Overview, item2: &impl Overview) <- This allows for item1 and item2 to be of different types
// fn call_overview<T: Overview>(item: &T, item2: &T) <- This requires item1 and item2 to be of the same type

// This function allows for having a parameter with multiple trait bounds
// fn call_overview(item: &impl Overview + AnotherTrait)
~~~

### Utility Traits
These traits are a grabbag of various traits from the standard Rust library.

#### Drop
Drop handles the freeing of memory as when an element goes out of scope. This allows for even more tight memory management.
After implementing the Drop trait, it is always run when the item goes out of scope and therefore we don't manually need to call it.
~~~
// using the data from the example above for abbreviation

impl Drop for TacticalMap{
    fn drop(&mut self){
        println!("Dropping: {}", self.map_name)
    }
}

// Continue of main function
fn main() {
    drop(map1); // This drops the map1 referene and the memory is freed.
}
~~~

#### Clone
The clone trait has been mentioned earlier, but here in more detail.\
Clone is for types that can make copies of itself. In order for this to work, it needs to make an independent copy using the self keyword and then return it.
Cloning values means allocating copies of anything it owns, which can make cloning a very memory heavy operation.

#### Copy
Simple types that does not own any values cannot implement the copy trait.

#### From and Into
From and into traits allows us to do conversions of one type and then into another.\
From is defined as `fn into(self) -> T`, so takes self and return a value of type _T_.\
Into is similar as defined as `fn from(T) -> self` takes a value of type _T_ and then returns self.\
From is going to act as a generic constructor for when we're a producing an instance of a type from some other single value.
There are also TryFrom and TryInto, which works as error catchers for the short trait names, which can be handy when for instance using from to convert an i64 to an i32 which would result in loss of bytes.
The Try traits will produce a Result return type which will allows to handle any errors which would occur while doing conversions.

## Collections
A rundown of some collections in the standard library.

### Vector - Vec\<T>
Vectors is a generic collection that holds elements of one type, and is instead of arrays and lists from other languages.
Much like in C# creation of a vector is done by specifying the type the vector will store in angle brackets.

`let mut v: Vec<i32> = Vec::new();`

Once the vector is created, using push and pop methods will add and remove items from the vector and return it.
~~~
let mut v: Vec<i32> = Vec::new();

v.push(2);
v.push(4);
v.push(6);
let x = v.pop(); // x is 6
~~~

As vectors store objects of known size next to each other in memory you can index into it.

`println!("{}", v[1]); // prints "4"`

If you index out of bounds Rust will panic.
Vectors can be created from literals by using the macro _vec!_

`let mut v = vec![2, 4, 6];`

#### Standard methods within Vector

~~~
fn main(){
    let mut nums: Vec<i32> = vec![]M;
    
    nums.push(1); // push adds elements to the top of the vector
    nums.push(2);
    nums.push(3);
    
    // pop will remove from the top -> Last in first out (Lifo)
    let val = nums.pop(); // This returns a Option<T>, either Some or None
    
    // It is possible to index into a vector
    let index_val = nums[1]; // This results in a copy
    // The above line is available because i32 implements copy
    // If the types of the items in the vector does not implement copy, we index with a reference
    // let index_ref_val = &nums[1];
    
    // It is possible to return the first element in a vector by using .first()
    let first_val = nums.first(); // Returns Option<T>, None if vector is empty, otherwise Some<T>
    
    // It's also contains a .last() method to get the last element of the vector
    // .first_mut and .last_mut borrows a mutable reference
    
    // Values can be inserted and removed from a vector by index, insert will move any elements after the given index down the vector (add 1 to the index of that element)
    nums.insert(0, 10); // Inserts 10 at indenx 0
    nums.insert(3, 12);
    nums.insert(2, 25);
    
    nums.remove(3); // Removes at index 3   
    
    nums.sort(); // Sorts the vector acending
    nums.reverse(); // Sorts the vector decending
    
}

~~~

### HashMap - HashMap<K, V>
The _HashMap_ is a generic collection where you specify a type for the key, and one for the value and can index into the hashmap values by looking up the key.
This is a Dictionary in C#.
Creating a hashmap is done like so
~~~
let mut h: HashMap<u8, bool> = HashMap::new();

h.insert(5, true);
let have_five = h.remove(&5).unwrap();
~~~
Creating new pairs into the hashmap id done by the insert method. Removing from the hashmap is done by the remove method, which returns an enum called _Option_.

## Enums
Enum is short for enumeration, allows you to define a type by enumerating its possible variants.
An enum is like an enum in C#, and created as such

~~~
enum Colour {
  Red,
  Green,
  Blue,
}

let red = Color::Red;
~~~
In Rust, it is possible to associate data and methods with the variants. It is possible for enum named variants to have a many different types;
- No data
- Single type of data
- Tuple of data
- Anonymous struct of data

Enums can be used with generics and can implement functions and methods.
~~~
enum DispenserItem {
  Empty,
  Ammo(u8),
  Things(String, i32),
  Place {x: i32, y: i32},
}

impl DispenserItem {
  fn display(&self) { }
}

// This is an enum in the standard library that will be used often.
enum Option<T> {
  Some(T),
  None,
}
~~~
### Option

Option is a special type of enum and is used whenever something may be absent. A value can be checked if is _some_ or _none_ by using the dot operator `.is_some` or `.is_none`.
Option implements the IntoIterator trait, so it can be treated like a vector of 0 or 1 items.

The Option enum is important as it allows a value to be either something or nothing. The nothing aspect is comparable to null.
Unlike in C# there is no _null_ value in Rust, so use the Option instead. You either have a value wrapped in the Some variant or you have _none_.

And enums can be used with structs as well
~~~
enum IpType{
    V4(String),
    V6(String),
}

struct IpAddress{
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let home = IpType::V4(String::from("127.0.0.1"));
    
    let loopack = IpAddress {
        kind: IpType::V6,
        adress: String::from("::1"),
    }
}
~~~
If you want to check for a single variant, you use the _if let_ expression, this is because enums can represent all sorts of data, therefore you need to use pattern matching.
~~~
if let Some(x) = my_variable {
  println!("value is {}", x);
}
~~~

### Match

In case you watch to match for all the variants use _match_.

~~~
match my_variable {
  Some(x) => {
    println1!("value is {}", x);
  },
  None => {
    println!("no value");
    },
}
~~~
The match expression requires that all possible matches are covered, so the expression must be exhaustive. This means that you match for all expected cases and then have a broader case catching the rest. The underscore can be used for anything, so it can be used for a default or an any-else branch.
The match expression can check for all kinds of types, but all arms of the match must return the same type.

~~~
let x  = match my_variable { // my_variable must be a variable that supports enums
  Some(x) => x.squared() + 1,
  None => 42,
};
~~~
If the return value isn't needed, you can leave out the semicolon after the final bracket. If you do use the return value, the semicolon must be there.

### Result
Result is used when an item might have a useful result or might have an error.
The implementation of Result looks like this:
~~~
#[must_use]
enum Result<T, E> {
  Ok(T),
  Err(E),
}
~~~
Both Ok and Err are wrapped by generics, but are independent of each other. The must_use annotation makes it a compiler warning to silently drop a result.
Rust encourages to handle all possible errors and will throw a warning if a Result is being dropped without handle.

~~~
fn load_file() {
    File::open("Text for phone.txt");
}

// Running this produces the following warnings
warning: function `load_file` is never used                                                                                                                                                                                                                                                                     
  --> src\main.rs:39:4
   |
39 | fn load_file() {
   |    ^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: unused `Result` that must be used                                                                                                                                                                                                                                                                      
  --> src\main.rs:40:5
   |
40 |     File::open("Text for phone.txt");
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default
help: use `let _ = ...` to ignore the resulting value
   |
40 |     let _ = File::open("Text for phone.txt");
   |     +++++++

warning: `sandbox` (bin "sandbox") generated 2 warnings                                                                                                                                                                                                                                                         
    Finished dev [unoptimized + debuginfo] target(s) in 1.85s
     Running `target\debug\sandbox.exe`

~~~

In the above example we get a warning as the file open operation might throw an error, and we should handle that situation.
So in order to get rid of the warning, lets consume the Result and produce some output it.

~~~
// This will provide a file struct if the result is Ok, otherwise it will crash.
fn file_open() {
  let res = File::open("foo");
  let f =  res.unwrap(); 
}

// This will do the same as above, but in case of a crash it will also print the content of the expect ()
fn file_open2() {
  let res = File::open("foo");
  let f = res.expect("error message");
}

// This prevents the crash by first checking if the Result is ok
fn file_open3() {
  let res = File::open("foo");
  if res.is_ok() {
    let f = res.unwrap();
  }
}

// Opening file by pattern matching
fn file_open4() {
  let res = File::open("foo");
  match res {
    Ok(f) => { /* do stuff with the file struct */ }
    Err(e) => { /* do stuff if the file open fails and produces an error */ }
  }
}
~~~
## Generics
Generics are types that will accept any type as parameter, meaning they're abstract standards for concrete types.
~~~
struct Point<T> { // This struct will accept any type as long as it is the same type for both x and y
    x: T,
    y: T,
}

struct AdvancedPoint<T, U> { // Here we can use two different types for the generic
    x: T,
    y: U
}

fn main() {
    let coord = Point { x: 5.0, y: 6.2 } // For this variable the type is float
    let coord2 = Point { x: 3, y: 8 0 } // For this variable the type is i32
    
    let advanced_coord = AdvancedPoint { x: 4, y: 3.5 } // x is a i32 and y is a float
}
~~~
Generics are therefore placeholders until we fill them with concrete types.


## Closure
A closure is an anonymous function that can borrow or capture some data from the scope it is nested in. The syntax is a parameter list between two pipes without type annotations, followed by a block.
This creates an anonymous function called closure that can be called later.
The types of the arguments and the return value are all inferred from how you use the arguments and what you return.

~~~
// Abstract implementation of a Closure
| x, y | { x + y }

// Implementation
let add = | x, y | { x + y };

// Call the closure
add(1,2); // returns 3
~~~

You don't need any parameters for a closure, and can even leave the block empty, but that provides little of interest.\
The closure will borrow a reference to the values in the enclosing scope.\
As the closure only borrows the reference to a valuable as long as they're in scope, the closure will normally not outlive the variable.
However, it is possible to let the closure take ownership of the variable by using the _move_ command, by moving them into itself.\
~~~
let s = "I'm being moved!";
let f = move || {
  println!("{}", s);
};

f();
~~~
This allows the closure to be sent over to another thread or whatever we what with it, now that it no longer is borrowing a reference to a variable, but is totally self enclosed.
Closures allow for neat methods on elements.
~~~
let mut v = vec![2, 4, 6];

v.iter() // Creates an iterator for the closure
  .map(|x| x * 3) // Multiplies every element in the vector with 3
  .filter(|x| *x > 10) // Removes any item that is greater than 10
  .fold(0, |acc, x| acc + x); // Add the values of the remaining items together
~~~

## Threads
In order to do threading in Rust, we must use the std::thread module. After that we can spawn a new thread and let that do work while the rest of the program continues.
Also consider when to use threads and when to use async, whichever yields the best performance for the required task. A thread::spawn takes a closure as an argument.
~~~
use std::thread;

fn main() {
  let handle = thread::spawn(move || { // thread.spwn takes a close with no arguments - this closure is executed as the main function of the thread, other functions or other work can then be called from here.
    // Do stuff in a child thread
  });
  
  // Do other stuff in the main thread simultaneously
  
  // Wait untill thread has exited
  handle.join().unwrap(); // spawn returns a join handle, which pauses the current thread, untill the joining thread has completed and exited.
}
~~~
As the _Result_ from the thread closing can be either an Ok result or an error in case the thread panicked, we need to unwrap this and handle whatever situation the thread produced.
As switching threads is heavy in computational terms, it is advised to consider if threading is needed, or you would be better off using an async pattern.

### Printing collections
When having to print the contents of an element which is a collection the println! needs some help in order to be able to print that collection.
We need to use a special operator in the println! macro in order to let the compiler go into debug mode and then also tell it to push elements of the collection onto the print.
For instance printing a vector would look like this
~~~
let nums = vec![1, 2, 3];  // Initialize a vector using the macro and litterals
println!("{:?}", nums); // The ? tells the compiler to go into debug mode and the : pushes the items in the collection into the print statement, allowing us to print the contents of the vector
~~~

## Organize code
Writing bigger projects requires codes to be structured to avoid massive code files. In Rust code can be organized using _packages_, _crates_, _modules_ and _paths_.

- Packages allow us to build, share and test crates.
- Crates are a trio of modules that produces a library or an executable.
- Modules allows for handling the organization, scope and privacy of _paths_.
- Paths is a way of naming an item such as a struct, function or module.

### Packages
A package is a collection of crates which provides functionality. The package contains a cargo.toml file which contains information on how to build those crates.
The .toml file contains information about dependencies and useful information about the package. The cargo new command creates a new package.
During the creation a cargo.toml file is created alongside a _src_ directory. This directory is not mentioned in the toml file because of the convention that Cargo follows.
Which is that src/main.rs is the crate root of the binary crate and src/library.rs defines a library crate.
The crate allows for bundling related functionality together, which helps reusability.

### Modules
Where packages are about code sharing between projects, modules are about code sharing within a project. Modules are a collection of named features, like structs or functions.
A module is created by using the `cargo new --lib` command.

You can create modules by specifying the path to the module. You can place a library file (lib.rs) in the root folder and use (import) it in the file.

Consider the following; Using the hello project created in the beginning, it is possible to add a lib.rs file in to the source directory. In the lib.rs file a single method will be added _greet_

~~~
pub fn greet() {
    println!("Hi!");
}
~~~
All methods in a module are private by default. Adding the pub keyword to it, enables the method to be public and as such can be called from the main.rs file. Using the absolute path and then the function name.
hello being the name of the project in toml file, the scope operator which is double colons and then the function name in the lib.rs, greet.
~~~
fn main() {
    hello::greet();
}
~~~

To prevent having to call the scope all the time, a _use_ statement can be added. This is like an import in Python or using in C#.

~~~
use hello::greet;

fn main() {
    greet();
}
~~~

Rust comes with a standard library, which contains the most used methods. If you want to use a vector _use_ it at the top of the file.

`use std::vec::Vec;`

In case you need something outside the standard library, for this go to crates.io, this is the package collection for Rust.

Below is a bigger module example:
First create the module, this can be done from the terminal using `cargo new todo --lib` where _todo_ will be the name of the module or via the menu in Rust Rover and selecting from the dropdown "Library" instead of "Binary".
This will create a new module with a cargo.toml file and a src folder with a lib.rs file in it.
After that a module is created alongside some logic

~~~
mod list{ // Create module
    pub struct Tasks { // public struct
        pub item: String, // public field
    }
    
    mod things_todo { // private module
        fn add_activity() {} // with private fields
        fn update_activity(){}
        fn marked_complete(){}
    }
    
    mod items_completed{
        fn remove_task(){}
        fn move_back_todo(){}
    }
}

fn lets_add_task(){
    let task = list::Tasks {item: String::from("Tasks")}; // this will work - creates a Tasks object
    list::things_todo::add_activity(); // will cause a compile error as the module is private. Note both module and element within it needs the pub keyword in order to be public accessible.
    list::items_completed::remvoe_task(); // relative path
    create::list::items_completed::remove_task(); // absolute path as we start at the root crate
}
~~~

#### Export
In order to keep the libraries tidy and not clutter it with code as the project grows we can add exports to keep the code base tidy and structured.
Working with the same example as above, we add a things_todo.rs file to the _src_ folder and call that from the code. This will tell Rust to look for that file in the src folder

~~~
// lib.rs file
mod list{ // Create module
    pub struct Tasks { // public struct
        pub item: String, // public field
    }
    
    mod items_completed{
        fn remove_task(){}
        fn move_back_todo(){}
    }
}

// note that the things_todo has been removed

mod things_todo; // Tells Rust to look for a file named that (and in that is our module with the same name)
use crate::things_todo::add_activity // Shortnes the namespace so we can call only add_activity(); when wanting to use it

fn lets_add_task(){
    let task = list::Tasks {item: String::from("Tasks")}; 
    add_activity(); // Because of the module and use statement, the absolute and relative paths are now identical
}

// things_todo.rs file
pub fn add_activity() {} 
fn update_activity(){}
fn marked_complete(){}
~~~

#### Nested modules
Nested modules are declared in the module file and Rust will search for them in the tree structure.\
First we crate a things_todo folder and in that we'll create a items_completed.rs file and move the items_completed module into that file.

~~~
// lib.rs file
mod things_todo;
use crate::things_todo::{add_activity, items_completed, items_completed::test::test}; // method in things things_todo, submodule and sub-submodule from the items_completed file

mod list{
    pub struct Tasks {
        pub item: String,
    }
}

fn lets_add_task(){
    let task = list::Tasks {item: String::from("Tasks")};
    add_activity();
    items_completed::remove_task();
    
// items_completed.rs file (located in the same name folder, subfolder to the things_todo folder)
pub fn remove_task(){}
fn move_back_todo(){}

pub mod test{
    pub  fn test(){}
}
~~~

If the cargo-modules is installed, the console can generate a tree structure to visualize the structure of modules / files.

~~~
cargo-modules structure --package todo

crate todo
├── fn lets_add_task: pub(crate)
├── mod list: pub(crate)
│   └── struct Tasks: pub
└── mod things_todo: pub(crate)
    ├── fn add_activity: pub
    ├── mod items_completed: pub
    │   ├── fn move_back_todo: pub(self)
    │   ├── fn remove_task: pub
    │   └── mod test: pub
    │       └── fn test: pub
    ├── fn marked_complete: pub(self)
    └── fn update_activity: pub(self)
~~~

## Binary Heap
This is a collection which has loosely organized elements. The greatest value always bubbles up to the front.
Works a lot like a vector with methods like push, pop and peek. A difference between the vector and a binary heap is that elements gets organized whenever an element are pushed or popped.

~~~
use std::collections::BinaryHeap;

fn main() {
    // Create a new binary heap
    let mut bheap = BinaryHeap::new(); 
    
    // Add elements to it and check how it is ordered
    bheap.push(2);
    bheap.push(9);
    bheap.push(0);
    bheap.push(22);
    println!("{:?}", bheap); // Prints [22, 9, 0, 2]
    bheap.push(13);
    println!("{:?}", bheap); // Prints [22, 13, 0, 2, 9]
    bheap.push(16);
    println!("{:?}", bheap); // Prints [22, 13, 16, 2, 9, 0]

    bheap.pop(); // This will pop 22, and reorder the heap so that 16 is in front
    println!("{:?}", bheap); // Prints [16, 13, 0, 2, 9]
    
    // Peek allows to get a see a value from the font of the heap without removing it.
    println!("{:?}", bheap.peek()); // Prints Some(16). So peek returns an Option<T>
}
~~~

The binary heap is useful in collections where we care about order.
