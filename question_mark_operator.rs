The question mark operator (?) in Rust is a syntactic sugar for error propagation, which allows a function to return eartly with an error value if a precending computation results in an error.

The ? operator is just short for a match.
After anything that returns a Result, you can add ?

give what is inside the Result if it is Ok.
pass the error back if it is Err (this is called an early return.

You simply type ? to use it.
