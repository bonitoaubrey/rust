Uninitialized variable is variable that is declared but nut assigned a value, and its memory is not initialized with a default value.

When a variable is declared in Rust, the compiler checks if it is initialized with a value.
If it's not, the compiler will prevent the code from compiling, because Rust does not allow the use of uninialized variables.
This is a safety feature to prevent bugs that can occur when using uninitialized memory.

You have a code block, and the value for your variable is inside it.
The variable needs to live outside of the code block.
You want people reading your code to notice the variable name before the block.

Write let and then the variable name and (if necessary) the type.
