References are a temporary borrow of a value that allow you to access the value without taking ownership of it.

When you create a reference to a value, Rust stores the memory address of the value in a new variable, which is the reference.
The reference is essentially a pointer to the original value, but it does not own the value.
Instead, it borrows the value, allowing you to access and modify it temporarily.
When the reference goes out of scope, the borrow ends, and the original value is no longer accessible through that reference.

Rust's reference rules:
1. You can have as many immutable references as you want.
2. You can only have one mutable reference.

When you want to access a value withou taking ownership of it.
When you want to modify a value temporarily without changing its ownership.
When you want to pass a value to a function without transferring ownership.
When you want to return a value from a function without transferring ownership.

You can create a reference to a value using the & operator.
You can create a mutable reference by using the &mut keyword.
You can dereference a reference using the * operator to get the underlying value.
