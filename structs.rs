A struct in Rust is a composite data type that represents a collection of values of different types, stored in a single unit of memory, enabling the creation of complex data structures and objects.

In Rust, s struct is implemented as a contiguous block of memory, where each field is stored in a specific location, and the size of the struct is determined by the sum of the sizes of its fields.
When a struct is created, Rust allocates memory for it on the stack or heap, depending on the context, and initializes its fields with the provided values.
When you define a struct, Rust creates a new type that represents a collection of fields, which are essentially variables that are stored together in memory.
The struct is stored in memory as a contiguous block of bytes, with each field stored in a specific order.
There are three types of structs.
    One is a unit struct. (doesn't have anything).
    The next type is a tuple struct, or an unnamed struct.
    The third type is the named struct, which is the most common struct.

can create your own type.
structs are necessary because they allow you to organize data in a way that makes sence for your program, making it easier to write, read, and maintain.
let create complex data types that can be used throughout you program, reducing code duplication and improving code reusability.

To use a struct, you define it using the struct keyword, followed by the name of the struct and the fields it contains.
The name of a struct should be in UpperCamelCase.
You can then create instances of the struct using the struct name and field values.
You separate fields by commas in a named struct.
If the field name and variable name are the same, you don't have to write both.
You can access the fields of a struct using dot notation.

enums are used to define a set of named values rather than a collection of fields.
