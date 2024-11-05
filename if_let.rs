if let is a syntax sugar in Rust that allows you to combine pattern matching with a conditional statement, making you code more concise and expressive.

When the Rust compiler encounters an if let statement, it desugars it into a match statement with a single arm that matches the pattern.
If the pattern matches, the code inside the if let block is executed.
The Rust compiler generates code that checks whether the value matches the pattern, and if it does, executes the corresponding code.

If we weren't doing anything in case of None because we were only interested in what happens when we get a Some, but we still had to tell Rust what to do in case of None.
Here we can make the code shorter by using if let.
Using if let means "do something if it matches, and don't do anything if it doesn't".
if let is for when you don't care about matching for everything.
if let is nessesary because it provides a concise way to handle situations where you only care about one specific pattern out of many possibilities.
Without if let, you would have to write a full match statement, which can be verbose and cluttered.

let x = Some(5);
if let Some(y) = x {
    println!("x is Some({})", y);
}

match is exhaustive, meaning that it must cover all possible patterns, whereas
    if let is not exhaustive, meaning that it only checks for a single pattern.
let x = Some(5);
//if let
if let Some(y) = x {
    println!("x is Some({})", y);
}
match x {
    Some(y) => println!("x is Some({})", y),
    None => (), //do nothing
}
//if with pattern guard
if x.is_some() {
    let y = x.unwrap();
    println!("x is Some({})", y);
}
In summary, if let is a concise and expressive way to handle pattern matching in Rust, and is particularly useful when you only care about one specific pattern out of many possibilities.
