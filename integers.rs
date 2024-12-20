Integers are primitive types that represent whole numbers with no decimal point.

There are two types of integers: signed integers and unsigned integers.
Signed means + (plus sign) and - (minus sign). Signed integers can be positive or negative or zero.
The signed integer types are i8, i16, i32, i64, i128, and isize.
The unsigned integers types are u8, u16, u32, u64, u128, and usize.
Unsigned can only be nonnegative.
The number after the i or the u means the number of bits for the number.
Number types with more bits can hold much larger numbers.
Signed integers have a maximum value that is only half that of an unsigned type of the same number of bits because they also have to represent negative numbers.
isize and usize are two types that have a number of bits depending on architecture of your computer.

Used to represent whole numbers, either positive, negative, or zero.

Integer Literals can be written in decimal, hexadecimal, octal, or binary notation:
    Decimal: 123
    Hexadecimal: 0x123
    Octal: 0o123
    Binary: 0b101
