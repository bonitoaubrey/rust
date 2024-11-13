A panic in Rust is a type of control flow mechanism that unwinds the call stack and terminates the current thread, triggered by an unrecoverable error or an assertion failure, allowing the program to propagete error information and clean up resources.

When a panic occurs, Rust's runtime environment is notified, and it begins to unwind the call stack, calling the drop method on each value that was allocated on the stack.
This process continues until the panic is caught by a catch_unwind function or until the program exits.
Panic means that the program stops before the problem happens.
Rust sees that the function wants something impossible and stops.

provide a way to handle errors that cannot be recovered from, allowing the program to exit cleanly and avoid undefined behavior.
debug programs by aborting the program and providing information about the error that occurred.
help ensure memory safety by preventing the program from accessing memory that has been deallocated or corrupted.

fn main() {
    panic!("Something went wrong!");
}

errors can be recovered from
    panics are unrecoverable
