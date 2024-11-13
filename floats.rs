Float are primitive type that represents a fractional number.

Floats in Rust are represented using the IEEE 754 floating-point standard, which derines how to store and manipulate floating-point numbers in binary format.
The standard uses a combination of bits to represent the sign, exponent, and mantissa of the number.
Rust supports two types of floating-point numbers:
    f32: is a 32-bit floating-point number and has precision of about 7 decimal places.
    f64: is a 64-bit floating-point number and has a precision of about 15 decimal places.
By default, it will choose f64.
Keep in mind that floating-point numbers can have rounding errors due to their binary representatin.
This means that some decimal numbers cannot be represented exactly as floating-point numbers, which can lead to small errors in calculations.

Use f32 when you need to store a large number of floating-point numbers and don't need high precision.
    This can help reduce memory usage.
Use f64 when you need high precision, such as in scientific simulations or financial calculations.
5.5
5.0
5.

Used to represent decimal numbers.
