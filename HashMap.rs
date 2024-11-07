A HashMap is a unsorted collection of key-value pairs.

The keys of a HashMap are not ordered.
If a HashMap already has a key when you try to put it in, using .insert() will overwrite its value.

lookup, insertion, and deletion of key-value pairs.

You use the key to loop up the value that matches the key.
Creating a new HashMap: yo can just use HashMap::new().
After that, you can use the .insert(key, value) method to insert items.
The simplest but least rigorous wayt to get a value in a HashMap is by putting the key in [] square brackets.
But be careful because the program will crash if there is no key, just like when indexing a Vec.
If you are mot sure threr will be a key, you can use .get(), which returns an Option.
