Option is a build-in enum in Rust that represent an optional value: every Option is either Some and contains a value, or None, and does not.

Option is an enum with two variants: Some(T) and None.
When a value exist, it is Some(value), and when it doesn't, it's None.
enum Option<T> {
    None,
    Some(T),
}
When you create an Option, you can either wrap a value of type T in Some(T) or create a None value.
The Option enum is implemented using a simple struct that contains a discriminant (a small interger that indicates which variant is present) and a payload (the actual value, if present).

use when something might or might not exist.
it allows you to explicitly handle cases where a value is absent or invalid, preventing common errors like null pointer dereferences or unexpected behavior.

We can get the value inside an Option with a method called .unwrap()
You only want to .unwrap() if you are sure.
If you unwrap a value that is None, the program will panic.
We can use a match instead.
With match, we can print the value if we have Some and not touch it if we have Nove.

//In this example, we created an Option value x that wraps an i32 value 5.
//We then use a match statement to handle the two possible cases: Some(y) and None.
let x: Option<i32> = Some(5);
match x {
    Some(y) => println!("x is Some({})", y),
    None => println!("x is None"),
}

Result is used to handle errors, while
    Option is used to handle cases where a value is absent or invalid.

//Here is example that illustates the difference between Option and Result
//Option
let x: Option<i32> = Some(5);
match x {
    Some(y) => println!("x is Some({})", y),
    None => println!("x is None"),
}
// Result
let x: Result<i32, String> = Err("error".to_string());
match x {
    Ok(y) => println!("x is Ok({})", y),
    Err(e) => println!("x is Err({})", e),
}

In summary, Option is a fundamental concept in Rust that allows you to safety and explicitly handle cases where a value is absent of invalid.
