Mutability is ability of a value to be changed after it's created.

In Rust, varibales are immutable by default, which means their value cannot be changed once they're created.
To make variable mutable, you need to use the mut keyword when declaring it.
When a variable is mutable, you can reassign a new value to it.
You cannot change the type of a variable even if you declare it as mut.

To use mutability in Rust, you simply add the mut keyword before the variable name when declaring it.

Mutability is similar to borrowing in Rust, as both concepts deal with accessing and modifying values.
However, the key difference is that mutability allows the owner of a value to modify it directly, whereas borrowing allows multiple parties to access a value without taking ownership of it.
