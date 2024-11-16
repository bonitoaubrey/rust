A while loop is a control flow statement that continues to execute a block of code as long as a certain condition is true.

The loop condition is evaluated before each eteration, and if it is false, the loop exits.
If the condition is true, it runs the code, and then checks the condition again.
This process repeats until the condition becomes false.

help us write more efficient and concise code by avoiding the need to write the same code multiple times.
implementing iterative algorithms that require repeated execution of a block of code
conditional repetition, where the number of iterations is not fixed

// You can use the while loop in Rust as follows
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}
This code will print the numbers 0 to 4.

for loop iterate over a sequence (like an array or a vector) while
    while loops continue to execute as long as a condition is true.
while loop has a explicit condition that must be true for the loop to continue, whereas
    a loop statement has as implicit condition that is always true.
while loop will exit when the condition becomes false, whereas
    a loop statement will run indefinitely until a break statement is encountered.
