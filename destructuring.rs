Destructuring is a syntax feature that allows to break down complex data structure, such as tuples, arrays, and structs, into their individual components, allowing you to bind them to separate variable.

When you use destructuring, Rust creates a new scope for the individual components, and binds each component to a new variable.
Destructuring in Rust works by pattern matching the structure of the data against a pattern, which specifies how to break down the data into its individual components.
When a match is found, the components are bound to the corresponding variables, allowing you to access and manipulate them individually.
You can't destructure if the pattern doesn't match.

When working with tuples, arrays, and structs, and you need to access their individual components.
When you need to bind multiple values to separeta variables in a single statement.
When you need to pattern match against complex data structures.
