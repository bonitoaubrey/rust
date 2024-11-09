Methods in Rust is a function that is associated with a type, allowing you to perform actions on values of that type in a more object-oriented way.

There are two kinds of methods in an impl block:
    Methods - These take self in some form (&self, or &mut self or self). Regular methods use a . (a period).
    Associated functions - These do not take self. It's use a :::
In Rust, methods are implemented as functions that take a reference to the type they are associated with as their first argument, known as the self parameter.
When a method is called, the compiler generates code that passes the self reference to the method function, allowing the method to access and modify the type's data.
Rust ownership system ensures that methods can only access and modify the data of the type they are associated with.
When you define a method on a type, Rust generates a special kind of function called a "closure" that has access to the type's fields and other methods.
When you call a method on a value, Rust creates a new closure that captures the value's fields and passes it to the method's implementation.
Self means the type Self, and self means the variable called self that refers to the object itself.
When you use a methog, Rust will dereference for you until it reaches the original type.
The . in a method is called the dot operator, and it does dereferencing for free.

To write function for a struct or an enum, use the impl keyword and then a scope with {} (impl block) to write the function.
