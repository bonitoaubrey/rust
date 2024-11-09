An enum in Rust is way to define a set of named values that have a specific meaning, allowing you to create a new type that can take on one of a limited number of possible values.

An enum is short for enumerations.
When you define an enum, Rust creates a new type that can take on one of the values you specify.
The enum is represented as a single value that is stored in memory, which is usually an integer.
Each variant of the enum is assigned a unique integer value, which is used to store the value in memory.
If an enum doesn't contain any data, then its variants can be cast into an integer.
enums can have associated values, which are values that are stored along with the enum variant.

help prevent errors by ensuring that only valid values can be used.
hold different types inside a collection.

To use an enum, you define it using the enum keyword, followed by the name of the enum and the variants it can take on.
To make a choice when using an enum, use the enum name, followed by two :: (colons), and then the name of the variant.

structs are used to group related data together, while
enums are used to define a set of named values.