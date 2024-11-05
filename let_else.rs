if let checks to see whether my_vec.get() gives the pattern Some.
If it gets a Some, it calls the variable inside it number and opens up a new scope inside the {} curly brackets.
Inside this scope, you are guaranteed to have a variable called number.
If .get() doestn't give the pattern Some, it will simply do nothing and go to the next line.
let else tries to make a variable number from the pattern Some.
If you take out the else part for a moment, you can see that it is tyring to do this: let Some(number) = my_vec.get();.
In other words, it is trying to make this variable called number.
