Result in Rust is a enum type that represent a value that may or may not be present, allowing for explicit handling of errors or successes in a computation.

Result is an enumeration type in Rust, which means it can have one of two possible values: Ok(value) or Err(error).
When a fuction returns a Result, it indicates that the computation may have succeeded (returning a value wrapped in Ok) or failed (returning an error wrapped in Err).
The Result type is designed to be used with pattern matching, allowing developers to explicitly handle both the success and error cases.

intended for handling errors and successes in a computation, providing a way to explicitly manage and propagate errors through the program.
used as a return type for function that may encounter errors, such as I\O operations, parsing, or computations that may fail.

enum Result<T, E> {
    Ok(T),
    Err(E),
}
Result has a value inside of Ok and inside of Err.
Result<T, E> means you need to think of what you want to return for Ok and what you want to return for Err.

Option holds a Some or None (value or no value).
    Result holds an Ok or Err (okay result or error result).
