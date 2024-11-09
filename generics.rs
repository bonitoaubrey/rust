Generics in Rust is a way to write functions, structs, and other code that can work with multiple types, allowing you to write reusable and flexible code.

Generics basically means "maybe one type, maybe another type".
When you define a generic fuction or struct, Rust creates a template for the code that can be instantiated with different types at compile-time.
The compiler generates specialized versions of the code for each type used, a process called "monomorphization".
Rust turns generic functions into concrete ones at compile time.
    There's nothing extra that happens at run time.

Generics allow you to write code that can be reused with multiple types, reducing code duplication and improving maintanability.
enable you to write code that can work with different types, making it easier to adapt to changing requirements.
without them, you would need to repeat your code every time you wanted a different type.

For generics, you use angle brackets with the type inside, like this: <T>.
This means "any type you put into the function."
Rust programmers usually use one capital letter for generics (T, U, V, etc.).
To make generic functions easier to read, we can also use the keyword where right before the code block.
After T, you write what traits the type will have.
    Having more tratis means T can do more things.
    But it also means that the function can take fewer types because any type needs all of the traits you wrote.
