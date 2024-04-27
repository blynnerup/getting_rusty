# getting_rusty
Learning the basics of Rust.
This is a helpme guide, used to reference things in the future.

## Basics
Cargo is the bread and butter. It handles your packages, your build and testing… everything is cargo.

### Basic commands
$ cargo new hello
- Creates a new Rust project called 'hello'
- Creates the Cargo.toml file which is the configuration file
- Creates a sub directory called src which contains the main.rs file which is the main file for running the project.
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
Const values are inlined at compile time, they're really fast.

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
Rust ensures memory safety at compile time.

## Functions
The Rust style guide says to use snake case for function names.
~~~
fun do_stuff() {
}
~~~

Function parameters are specified by name: type and seperated by comma.

The return type is specified by an "arrow" ->

Using the return keyword for the return value. The return value can be shorthanded by omitting the return keyword and semi colon from the last statement in the block, and the compiler will use this as return value, this is also called the tail expression.

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

## Module System
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

## Scalar Types
There are four scalar types in Rust
- integer
- float
- boolean
- characters

### Integers
There are many integer types;

| Unsigned | Signed |
| -------- | ------ |
|   u8     | i8    |
| u16 | i16 |
| u32 | i32 |
| u64 | i64 |
| u128 | i128 |
| usize | isize |

The u- and isize has the size of the platforms integer bits and can be used to access memory. If you don't initialize a integer, it will default to i32, as this is the fastest. The integer types may not be universally supported, it depends on system architechture.

### Integer Litterals
| Type | Specification |
| ---- | ---- |
| Decimal | 10000 |
| Hex | 0xdeadbeef |
| Octal | 0o71234232 |
| Binary | 0b 11100110 |
| Byte (u8 only) | b'A' | 

A feature in Rust is that underscores can be added inside or at the end of a integer to ease readability. For instance 1_000_000 reads as 1000000 when compiling or could be done like so 1_0_0_0_0_0_0_.

### Floats
Float comes as f32 and f64, with 32 and 64 bits of precision respectively. The default value for floats are f64. These can however be really slow on less than 64 bit architecture, so chosing the right size matters.

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
Booleans are not integers, but can be casted as such.

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

Indexing is done by using square brackets. Arrays are limited to 32 elements, after which they lose some of their functionality. Instead Vectors can be used.

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

str is a string slice and will often be seen as &str which is the _borrowed string slice_. A literal string is always a borrowed string slice. A borrowed string slice &str is often refered to as a string. This can be confusing as the other commonly used string type is String. The biggest difference between the two is that the data in a borrowed string slice cannot be modified, whereas the String data can. A borrowed string slice is internally made up of a pointer to some bytes and a length. A String is made up of a pointer to some bytes, a length and a capacity, that may be higher than what is currently being used. In other words a borrowed string slice is a subset of a String in multiple ways. Both are valid UTF-8. Strings cannot be indexed by char position.


## Ownership
Ownership is a unique feature in Rust, enabling memory safety while still being a systems programming language.
There are three rules to ownership;

1. Each value has an owner. There is no value or data in memory that does not have a variable that owns it.
2. There is only one owner of a value. There is no shared ownership. Other variables may borrow a value.
3. When the owner goes out of scope, the value gets dropped immediately.

In order to understand how ownership of a value can change, consider the following:

~~~
let s1 = String::from("abc");
let s2 = s1;

println!("{}", s1); // This results in an error seen below

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

Another important thing to understand is when Rust uses Copy and Clone. If both s1 and s2 were supposed to have the same value, s2 could have been initialized as such:
~~~
s2 = s1.clone();
~~~
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
Instead of moving a variable, references can be used. To reference a value use _&_

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

_Lifetimes_ can be summed up as a rule stating that references must always be valid. This means that the compiler will not allow a value to outlive the reference to it.
And you can never point to _null_.

References default to a immutable value. However if we make a mutable reference to a mutable value, then we can use the reference to change the value as well.

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

