Dereferencing in Rust is the process of accessing the value stored at a memory  location pointed to by a reference, which involves resolving the memory address and retrieving the value stored at that location.

When you use a method, Rust will dereference for you until it reaches the original type.
The . in a method is calles the dot operator, and it does dereferencing for free.

Dot operator will dereference as much as needed, so you don't have to write * and & everywhere just to use the methods for a type.
