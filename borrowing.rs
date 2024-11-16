Borrowing in Rust is a mechanism that allows a function or method to use a value owned by another part of the program without taking ownership of it.

Borrowing is implemented using a consept called "lifetime" and "borrow checker".
When a value if borrowed, the compiler creates a new reference to the value, which is valid for a specific lifetime.
The borrow checker ensures that:
    The borrowed value is not dropped while the reference is still in use.
    There are no multiple mutable references to the same value.
    The reference is not used after the borrowed value has been dropped.

Used to avoid unnecessary copies of data, to share between different parts of the code, and ensure memory safety.

We can borrow a value by using the & operator, which creates a reference to the original value.
You can also use the &mut operator to create a mutable reference.

Borrowing is similar to ownership in Rust, but instead of taking ownership of a value, borrowing allows you to use a value without taking ownership of it.
