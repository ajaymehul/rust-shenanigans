# Rust Shenanigans

I'm just gonna use this space to journal my rust learning journey. 
I'm going to use the offical book ([The Book](https://doc.rust-lang.org/book)) from Rust 
It's been a while since I've picked up any programming language. My work, projects, etc., have all just been javascript/python. 

Now there's this hot new programming language called Rust. Memory? I have a vague memory of it since my college C programming days. I miss memory management. I remember the discomfort of not knowing what's happening behind the scenes in js or python. I'm excited to get back down to low-level stuff with Rust. This shall be fun. 

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



## Chapter 1 - Getting Started
### Installation

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Well that was easy!

### hello-rust

That's a cute crab. Terminal text art is always awesome!!
```
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
```
const I_CANNNOT_CHANGE: u32 = 2 * 4;
```
- stuff that can be assigned to `const` and operations that can be done before binding to constant at compile time: https://doc.rust-lang.org/reference/const_eval.html
- shadowing: use `let` again to create a new variable that shadows the first. If the scope of the second variable is over, but we're still in the scope of the first, it'll return back to the first copy of the variable. Nice. 

### Data Types
Usage:
```
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
```
let tup: (i32, f64, u8) = (500, 6.4, 1);
```
- can be access by destructuring `let (x,y,z) = tup;`
- also by indexing `let tuple_first_val = tup.0;`. Syntax looks ugly imo. 

#### Arrays

Fixed length arrays! Ooh brings back C memories. Ah, building my own resizable array data structure with ammortized O(1) inserts for CS 101. Dr. Brandon Dixon taught that class at 8:00 am while sipping on orange sunkist. Of course, rust gives us `vectors` too, so irrelavant.

Syntax:
```
let a = [1,2,3,4,5];
let a: [i32; 5] = [1,2,3,4,5]; //type & size explicit
let a = [3; 5]; //repeat same value: [3; 5] = [3,3,3,3,3]
```

### Functions

Syntax:
```
fn another_function () {
  println!("Another function.");
}
```

- Function parameters must have explicit types. No inference here. 

#### Expressions
```
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
```
fn five() -> i32 {
    5
}
```

### Comments
```
// This is a single line comment

// This is a multiline comment. 
// Heh. With VSCode ⌘ + /, we'll be fine.
```

### Control Flow
#### if-else statement syntax:
```
let number: i32 = 5;

if number < 5 {
    println!("{} number is less than 5", number);
} else {
    println!("{} number is not less than 5", number);
}
```
> Note using `()` for if condition throws a warning. Rust doesn't require outermost parantheses

#### bind variable to if-else returning expression
```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```
#### infinite loop
```
loop {
    println!("I will go on forever!"); //until break;
}
```
#### while
```
while number != 0 {
    println!("{}!", number);
    number -= 1;
}
```
#### for in a collection
```
let a = [10, 20, 30, 40, 50];
for element in a {
    println!("the value is: {}", element);
}
```
#### traditional for loop
```
for number in (1..4).rev() {
    //do whatever. Not inclusive of 4
}
```
#### Notes

Most of the control flow stuff was pretty standard. One interesting thing is you can use the `break` to return stuff to variables. For example this is valid syntax:
```
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

Alright, apparently there'something called 'ownership' that's unique to Rust. That's what's up. Let's take a break & have some dinner, and then we'll get back to it.g










