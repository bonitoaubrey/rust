In Rust, char is a primitive type that represents a single Unicode scalar value, which is a unique code point in the Unicode Standard, used to represent a character, symbol, or modifier in a computing system.

char is represented as a 32-bit unsigned integer, using the UTF-32 encoding scheme.
That means that each char value is a unique 32-bit code point, which can represent any Unicode character, including letters, digits, symbols, and emojis.
When you create a char value, Rust store it as a 32-bit integer, and when you use it, Rust performs the necessary Unicode-related operations, such as encoding and decoding, to ensure that the character is represented correctly.
Every char has a number.
The list of numbers is called Unicode.
The characters used most (called ASCII) are represented by numbers less than 256, and they can fit into a u8.
    This means that Rust can safely "cast" a u8 into a char, using as.
All chars use 4 bytes of memory, since 4 bytes are enough to hold any kind of character.

Used when we need to work with single characters in a string or data.
representing single Unicode character.

We create a char variable and assign it a value, such as 'a', which represents a single character.
char is always one character and uses ' ' (single quotes).
let c = 'a'; // creates a char value representing the letter 'a'
println!("{}", c); // prints 'a'

The char represent a single character, while the u8 represents a single byte.
char type similar to the u32, as both represent 32-bit unsigned integers, but
    char is specifically designed to represent Unicode scalar values, whereas
    u32 is a general-purpose integer type.
While you can cast a char value to a u32 value, the reverse is not always true
    as not all u32 values represent valid Unicode characters.

In summary, char in Rust is a specialized type for representing single Unicode characters, with build-in support for Unicode-related operations, whereas u32 is a general-purpose integer type.
