A const is a name assigned to a value, that cannot be changed.

When the Rust compiler encounters a const declaration, it evaluates the expression and replaces all occurences of the constant with its actual value.
The value must be a constant expression that can be evaluated at compile time.
Onece a constant is declared, its value cannot be changed.

consts are necessary because they provide a way to define values that are guaranteed to remain the same throughout the execution of your program.
const is for values that don't change and are created at compile time.

A const in Rust is declared using the const keyword, followed by the name of the constant and its value.

consts are similar to let bindings, but the key difference is that
    consts are evaluated at compile time and cannot be changed at runtime, while
    let bindings are evaluated at runtime and can be changed.
consts are also similar to statics, but the key difference is that
    consts are evaluated at compile time and can be used in constans expression, while
    static are initialized at runtime and cannot be used in constant expressions.
