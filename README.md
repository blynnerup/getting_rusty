# getting_rusty
Learning Rust

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
