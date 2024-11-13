Lifetime in Rust is a concept that defines the scope for which a reference to a value is valid, ensuring that references are always valid and preventing common errors like dangling pointers or use-after-free bugs.

Lifetime are based on the concept of ownership and borrowin in Rust.
When a value is created, it has owner that is responsible for dealocating it.
When a reference to a value is created, it borrows the value from its owner.
Lifetime ensure that the borrowed value lives at least as long as the reference to it.
Rust borrow checker enforces lifetimes at compile-time, preventing invalid references.

Lifetimes are used whenever we work with references in Rust such as:
    Function parameters and return types
    Struct fields
    Trait bounds
    Generic types

Lifetimes are specified using the ' symbol followed by a lifetime name.
