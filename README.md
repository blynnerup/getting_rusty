# getting_rusty
Learning the basics of Rust.
This is a helpme guide, used to reference things in the future.

## Basics
Cargo is the bread and butter. It handle your packages, your build and testing.. everything is cargo.

### Basic commands
$ cargo new hello
- Creates a new Rust project called 'hello'
- Creates the Cargo.toml file which is the configuration file
- Creates a sub directiory called src which contains the main.rs file which is the main file for running the project.
- The file contains a single function that prints 'hello world!'

In order to run it call the following in the project directory:
$ cargo run

This builds the project and creates a target folder with all the build artifacts. This should  be ignored by version control as it should not be stored centrally.

$ cargo run --release

This will allow cargo to compile without the debug symbols. This is a lot faster. This will also produce files in the release subdirectory.

## Variables
Variables are declared using the 'let' statment. You can also use a type statement.
It is possible to declare multiple variables at once;

`let (cards, players) = (52, 4);`

This will assign the value 52 to cards and 4 to players.

### Mutable and immutable
Variables are immutable by default in Rust, in order to make them mutable, this needs to be explicially annotated on the variable.
- This is designed with safety, concurrency and speed in mind.
  - Compiler can make optimizations to variables that it know will not change.

 In order to make a variable mutable use the statement 'mut'
 
`let mut boxes = 5;` <-- this is mutable

### Const
The const keyword is even more immutable. It requires the following;
- The 'const' keyword instead of let
- The naming convention is to use screaming snake casing for consts
- The type annotation is required
- The value must be a contant expression that can be compiled at runtime

`const WARP_FACTOR: f64 = 9.9;`

The reasons behind this is to be able to use a const outside function at module scope and use it anywhere, it is global and immutable.
Const values are inlined at compile time, they're really fast.

### Scope
Variables are scoped. Meaning they begin where they're created and extends to the end of the block in which they're created. Incluing nested blocks.
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
Macro names looks like function names, but are always postfixed with a exclamation point.
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
All methods in a module are private by default. Adding the pub keyword to it, enables the method to be public and as such can be called from the main.rs file. Using the abselout path and then the function name.
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
- integers
- float
- boolans
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

### Floating point Litterals
Follows the IEEE-754 standard, looks like this 3.14159. No special annotation of float is required.

### Numerical Litteral Suffixes
A numerical litteral can optionally include the type as a suffix.
~~~
let x: u16 = 5;
let y: f32 = 3.14;

let z = 3.14f32; // Includes the float 32 type declaration as a suffix
let xyz = 3.14_f32 // Same as above, but includes underscore for readability
~~~

This can be useful when passing on values to a generic function which accepts multiple litteral types as parameters.

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
In Rust, if is a expression not a statement. Statements don't return values, expressions do. 
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
In the Rust standard library there are six different types of Strings, however two are most commenly used.

str is a string slice and will often be seen as &str which is the _borrowed string slice_. A litteral string is always a borrowed string slice. A borrowed string slice &str is often refered to as a string. This can be confusing as the other commonly used string type is String. The biggest difference between the two is that the data in a borrowed string slice cannot be modified, whereas the String data can. A borrowed string slice is internally made up of a pointer to some bytes and a length. A String is made up of a pointer to some bytes, a length and a capacity, that may be higher than what is currently being used. In other words a borrowed string slice is a subset of a String in multiple ways. Both are valid UTF-8. Strings cannot be indexed by char position.


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

println!("{}", s1); // This results in an error