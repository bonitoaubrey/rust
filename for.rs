A for loop in Rust is a control flow construct that iterates over a sequence of values, executing a block of code repeatedly for each value, until the sequence is exhausted or a termination condition is met.

When a for loop is executed, the Rust compiler translates in into a loop that uses an iterator to travers the sequence of values.
The iterator is responsible for keeping track of the current position in the sequence and porviding the next value on each iteration.
The loop body is executed for each value yielded by the iterator, until the iterator indicates that there are no more values.
for loop doesn't need a condition to stop.

processing each element of a collection
transforming data from one format to another
searching for a specific value in a collection

// The basic syntax of a for loop in Rust
for pattern in iteration {
    //loop body
}
We use the for keyword to create a for loop.
If you don't need a pattern, use _ (an underscore).
If you give a pattern and don't use it, Rust will tell you.
Rust suggests writing _pattern instead of _.
