# Rust Shenanigans

I'm just gonna use this space to journal my rust learning journey. 
I'm going to use the offical book ([The Book](https://doc.rust-lang.org/book)) from Rust.

It's been a while since I've picked up any programming language. My work, projects, etc., have all just been javascript/python. 

Now there's this hot new programming language called Rust. Memory? I have a vague memory of it from my college C programming days. I miss memory management. I remember the discomfort of not knowing what's happening behind the scenes in js or python. I'm excited to get back down to low-level stuff with Rust. This shall be fun. 

## Table of Contents
- [Rust Shenanigans](#rust-shenanigans)
  - [Table of Contents](#table-of-contents)
  - [Chapter 1 - Getting Started](#chapter-1---getting-started)
    - [Installation](#installation)
    - [hello-rust](#hello-rust)
  - [Chapter 2](#chapter-2)
    - [guessing_game](#guessing_game)
  - [Chapter 3 - Basics](#chapter-3---basics)
    - [Variables](#variables)
    - [Data Types](#data-types)
    - [Compound Types](#compound-types)
      - [Tuples](#tuples)
      - [Arrays](#arrays)
    - [Functions](#functions)
      - [Expressions](#expressions)
      - [Function returning values](#function-returning-values)
    - [Comments](#comments)
    - [Control Flow](#control-flow)
      - [if-else statement syntax:](#if-else-statement-syntax)
      - [bind variable to if-else returning expression](#bind-variable-to-if-else-returning-expression)
      - [infinite loop](#infinite-loop)
      - [while](#while)
      - [for in a collection](#for-in-a-collection)
      - [traditional for loop](#traditional-for-loop)
      - [Notes](#notes)
      - [fibonacci exercise](#fibonacci-exercise)
  - [Chapter 4 - Ownership](#chapter-4---ownership)
    - [String Type](#string-type)
    - [Ownership & Functions](#ownership--functions)
    - [References](#references)
    - [Mutable Reference](#mutable-reference)
      - [Note on scope of references](#note-on-scope-of-references)
      - [Rules](#rules)
    - [Slices](#slices)
      - [Substring](#substring)
      - [General subarray](#general-subarray)
      - [Mutable Slice](#mutable-slice)
  - [Chapter 5 - Structs](#chapter-5---structs)
    - [Definition](#definition)
    - [Defining methods](#defining-methods)
      - [Associated functions](#associated-functions)
  - [Chapter 6 - enums](#chapter-6---enums)
      - [Syntax](#syntax)
    - [Option enum](#option-enum)
    - [match](#match)
      - [Syntax](#syntax-1)
      - [Enum variant with data](#enum-variant-with-data)
      - [catch-all](#catch-all)
    - [if-let](#if-let)
  - [Chapter 7 - Packages](#chapter-7---packages)
  - [Chapter 8 - Common Collections](#chapter-8---common-collections)
    - [Vectors](#vectors)



## Chapter 1 - Getting Started
### Installation

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Well that was easy!

### hello-rust

That's a cute crab. Terminal text art is always awesome!!
```rust
< Hello fellow Rustacenas! >
 --------------------------
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
```

Onwards to reading the book!!!

## Chapter 2
### guessing_game
Ah, snake casing. I don't know if I'm a fan, but okay. 

This project was fun. They haven't really explained any concepts yet. And the paradigm seems weird. But I have a feeling this `match` functionality could potentially be very powerful. We'll see. I love writing code functionally. If this ends up being so that I can write out potential paths my procedures will follow as `enums` and use `match` to call approptiate functions, that would some extremely clean code. 

I do have to note that I'm sort of sketched out by the scoping. Using `::` repeatedly might get annoying. And I'm still not clear on how the `use` brings stuff into the scope. I gotta say I'm a big fan of ES6 style imports, and importing stuff into variables. If Rust ends up implicitly bringing things into scope with `use`, maintaining code and development speed will suck.

Anyway, let's continue down this road. I have Hans Zimmer's gladiator theme playing on loop in the background and I'm feeling ambitious. 

Ferris, Strength & Honor!

## Chapter 3 - Basics

### Variables

- `let` to create variables. Variables are immutable by default. I like this. 
- `mut` preceding the variable name makes it mutable. 
- `const` for constants. Immutable as well. But can't edit mutability with `mut`. Naming convention: `screaming snake case`. eg:
```rust
const I_CANNNOT_CHANGE: u32 = 2 * 4;
```
- stuff that can be assigned to `const` and operations that can be done before binding to constant at compile time: https://doc.rust-lang.org/reference/const_eval.html
- shadowing: use `let` again to create a new variable that shadows the first. If the scope of the second variable is over, but we're still in the scope of the first, it'll return back to the first copy of the variable. Nice. 

### Data Types
Usage:
```rust
let <varname>: <type> = <whatever-we-wanna-bind>;
```

- statically typed
- compiler can infer usually
- `parse` method requires expicit declaration of type
- all the [scalar types](https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types)
- integer overflow in `--release` compiled binaries perform two's complement integer wrapping. You can also do this in dev environment using: [integer overflow docs](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow)
- booleans are `true` & `false`. Praise the ***logos*** (looking at you python)
- rust `characters` are 4 bytes - represent Unicode Scalar value

### Compound Types

#### Tuples
```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
- can be access by destructuring `let (x,y,z) = tup;`
- also by indexing `let tuple_first_val = tup.0;`. Syntax looks ugly imo. 

#### Arrays

Fixed length arrays! Ooh brings back C memories. Ah, building my own resizable array data structure with ammortized O(1) inserts for CS 101. Dr. Brandon Dixon taught that class at 8:00 am while sipping on orange sunkist. Of course, rust gives us `vectors` too, so irrelavant.

Syntax:
```rust
let a = [1,2,3,4,5];
let a: [i32; 5] = [1,2,3,4,5]; //type & size explicit
let a = [3; 5]; //repeat same value: [3; 5] = [3,3,3,3,3]
```

### Functions

Syntax:
```rust
fn another_function () {
  println!("Another function.");
}
```

- Function parameters must have explicit types. No inference here. 

#### Expressions
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```
- `y` above is binded to an expression.
- expressions do not end with an `;`

#### Function returning values
If you don't specify any expression to return, by default functions return `()`, an empty tuple or `unit type`. 

You specify the return type and return something with the following syntax:
```rust
fn five() -> i32 {
    5
}
```

### Comments
```rust
// This is a single line comment

// This is a multiline comment. 
// Heh. With VSCode ⌘ + /, we'll be fine.
```

### Control Flow
#### if-else statement syntax:
```rust
let number: i32 = 5;

if number < 5 {
    println!("{} number is less than 5", number);
} else {
    println!("{} number is not less than 5", number);
}
```
> Note using `()` for if condition throws a warning. Rust doesn't require outermost parantheses

#### bind variable to if-else returning expression
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```
#### infinite loop
```rust
loop {
    println!("I will go on forever!"); //until break;
}
```
#### while
```rust
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```
#### for in a collection
```rust
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {}", element);
}
```
#### traditional for loop
```rust
for number in (1..4).rev() {
    //do whatever. Not inclusive of 4
}
```
#### Notes

Most of the control flow stuff was pretty standard. One interesting thing is you can use the `break` to return stuff to variables. For example this is valid syntax:
```rust
let result = loop {
    // repeatedly do something
    if <condition> {
        break <some-return-expression>;
    }
}
```
This is really nice.

#### fibonacci exercise

I tried utlizing the `break` return functionality to by creating a `loop` as the return statement. So that `loop` is the return expression, and what the `break` returns ends up bubbling out of the entire function's return. Some edgy syntax, nice!!

Alright, apparently there's something called 'ownership' that's unique to Rust. That's what's up. Let's take a break & have some dinner, and then we'll get back to it.



## Chapter 4 - Ownership

Rules:
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

### String Type

- String from `String::from("hello")` is apparently stored on the heap. Interesting. 
- In String type, the `from` method is doing the allocation. Once the heap allocated variable goes out of scope, some `drop` function is automatically called for us. Nice. 


```rust
let s1 = String::from("hello");
let s2 = s1
```
So changing `s2` will change `s1` too. Makes sense, `s1`,`s2` both reference the same memory block. Basic CS stuff so far. 

>What am I reading? If you perform `s2=s1` the ownership has been transferred, and `s1` is no longer valid. You can't use it anymore. So no double free errors. Hmmm....
>
>Shallow copies aren't a thing. Because you can't even copy anymore. It's called a move. I'm curious to think how this'd even be useful. My prediction:
>```
>let s = String::from("whatever");
>s = some_procedure(s)
>```
>I'm guessing this will be a powerful strategy, you can pass ownership off to further procedures and get it back after their done. In the meantime you can't do anything with it. Will with the above syntax, in a single-threaded process, it'll literally wait there anyway. Still, this is hard to wrap my mind around. 
>
>**virtūs et honos**!

- deepcopy: 
  ```rust
    let s1 = String::from("hello");
    let s2 = s1.clone();
  ```
- Rust has a `Copy` trait that we can annotate to types to allow stack copy operations. But the compiler won't let us use it if it has the `drop` method implemented.

### Ownership & Functions
```rust
fn main() {
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);
}
fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.
```

Wow, my prediction was correct. You can give an take back ownership by passing it to functions, and capturing their return values. I guess I have half-decent programming intuition. 

Scratch that, we are now onto `references` because passing and receiving ownership like that would be tedious. So Rust has implemented some features to help with this. 

### References
Instead passing ownership to a function, you can pass a reference that the function can use to access the memory without taking ownership. 
Syntax:
```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
```
Nice.

> You can also dereference with `*`. Okay, so it's just C pointers, except I just have to be aware that when whatever variable has the ownership goes out of scope, it'll have `free()` called on it. 

### Mutable Reference

If you want to mutate the variable while borrowing it, you'll need the variable to be mutable, and reference be mutable. 

- only one mutable reference at any given time.

#### Note on scope of references
```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point
let r3 = &mut s; // no problem
println!("{}", r3);
```
This is valid code because r1, r2 is not used after println!. So it's considered out of scope. I didn't expect this. This is called: Non-Lexical Lifetimes. I should read up on this. 

#### Rules

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

### Slices

Slices are an borrowed reference to a subset of memory of another variable. 
#### Substring
```rust
let s: String = String::from("Hahaha Helloo");
let sub_string: &str = &s[0...2]; //[starting-index ... ending-index-minus-1]
```

#### General subarray
```rust
let arr: [u32; 10] = [10,20,30,40,50,60,70,80,90,100];
let sub_arr = &arr[0..5]; //0 to 4
let sub_arr_2 = &arr[..] //whole
```

#### Mutable Slice
```rust
let mut array_mut: [u32; 8] = [10,20,30,40,50,60,70,80];
let sub_array_mut = &mut array_mut[0..5];
sub_array_mut[2] = 100;
```

## Chapter 5 - Structs

### Definition
```rust
struct User { // 1
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(u32, u32, u32) //2
struct Point(u32, u32, u32) //2

struct UnitLikeStruct; //3
```

- 1: Struct with data. We had to use `String` instead of slice `&str` because we need the data inside structs to be alive as long as the struct. So struct owner should own the data inside. To use references inside struct, we need to learn lifetimes later
- 2: Tuple Struct. Both `Color` and `Point` have the the same data schema. But are considered different types. Used like any other tuple. 
- 3: Unit-like struct. Struct with no data. It can be used to defined behavioe. We'll learn later. 

Bad:
```rust
let mut user2 = User {
  email: String::from("bobby_new@rust.com"),
  ..user
};
user.username = String::from("Bobbie-new"); // error
// because ownership went to user2
user.email = String:from("bobbdy_user@rust.com"); //valid
// because new String was created when user2 was created. So user still owns user.email. 
user.active = false; //valid
// because bool has Copy trait, so it was copied. Not ownership transfered like email. Stack data. 
```

### Defining methods
[Docs](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html)

```rust
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }
}
```

`impl` short for implementation allows one to implement methods. Methods take in a `self` which can be mutable or not mutable. It's usually **borrowed** instead of ownership transferred unless you'd want to transform it into something else and return a new object. 

#### Associated functions

This explains the `String::from("blah")` syntax. 
These and implementations of the struct, that don't take in a `self` and return an object of type the `impl` is associated with. Useful for stuff like constructors. 

Example for `Rect` from above:
```rust
impl Rect{
    fn square(size: u32) -> Rect {
        Rect {
            height: size,
            width: size
        }
    }
}
```

## Chapter 6 - enums

#### Syntax 
```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Enums with data encoded:
```rust
enum IpAddr {
    V4(String), 
    V6(String)
}
```

Enums with different types of data:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```
Quit has no data, Move is like a struct, Write has a String, ChangeColor is a tuple with scalars. 

You can also `impl` for enums. I'm not sure how the implmented functions are supposed to handle the different types of data associated with enums. 
Maybe that `match` will come into play, we'll see. 

### Option enum

Supposed to be replacement for `null`. `Option` can either have `None` variant (this is not `null`, just a simple enum variant, but serves like `null`. I suppose even `null` might be just a simple implementation. Gotta find out. ), or a `generic type` (more on this later). 

Standard Library Definition:
```rust
enum Option<T> {
    None,
    Some(T),
}
```

Usage:
```rust
let num = Some(5);
let string = Some("a word");

let absent_number: Option<u32> = None;
```


AHH, the book says we'll use `Options` by using `match`. This `match` is going to be so cool when we get there. 


### match

Lol, it was literally the next page. Yay!!

#### Syntax
```rust

match coin {
  Coin::Penny => 1,
  Coin::Nickel => 5,
  Coin::Dime => 10,
  Coin::Quarter => 25
}
```
#### Enum variant with data
If the enum variant had data associated with it:
```rust
//snip
  Coin::Quarter(data) => {
      //do stuff with data
      25
    }
```

#### catch-all
All the different possible variants of an enum must be handled in match. 
Example for catch-all:
```rust
//snip
  other => // do whatever
```
If we didn't want to use the data:
```rust
//snip
  _ => // handle all other arms
```

### if-let

This is essentially just match with two arms: one variant of enum , all other variants.

This code:
```rust
let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
```
becomes:
```rust
let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
}
```

You can also use `else` after the `if` block to handle all other cases instead of ignoring it. 

Nice stuff so far!!!

## Chapter 7 - Packages

I read through this whole chapter and worked out the examples. I highly doubt I'll remember this clearly. I'll just pick it up when I start coding bigger projects and start reading other packages. Until then, I'll just drop a link to the chapter below:


https://doc.rust-lang.org/stable/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html


## Chapter 8 - Common Collections

### Vectors

- Usual vectors. Created using `vec![1,2,3]`
- Mutable Vector:
  ```rust
  let mut v = Vec::new();
  v.push(5);
  ```
- Returns an Option<T> so that we can handle stuff without errors using `match`

Two ways to pull data out of vectors:
```rust
let v = vec![1, 2, 3, 4, 5]
let third: &i32 = &v[2];
println!("The third element is {}", third)
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}

```











