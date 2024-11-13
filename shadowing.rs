Shodowing is a feature in Rust that allows you to declare a new variable with the same name as an existing variable in the same scope.

When you declare a new variable with the same name as an existing variable, the new variable "shadows" the existing one, making it inaccessible from the current scope.
The existing variable is not modified or deleted, but it's temporarily hidden.
When the new variable goes out of scope, the original variable becomes accessible againg.
Shadowed variable is pointing to a completely different value.

When you need to work on a variable a lot and you don't care about it in between.
Without shadowing, you would have to think of different names.
When you need to temporarily override the value of a variable.
When you neet to use a variable with the same name in a nested scope.

To use shadowing in Rust, simply declare a new variable with the same name as an existing variable.

When you mutate a variable, you change its value.
When you shadow a variable, you create a new variable with the same name, but the original variable remains unchanged.
