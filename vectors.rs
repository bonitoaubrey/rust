Vector in Rust is a growable, dynamically-sized collection of values of the same type, stored in contigous memory, which povides effecient insertion, removal, and indexing operations.

All items in a Vec must all have the same type.
Vec always has something else inside it, and that's what the <> (angle brackets) are for.
Vectors are implemented as a struct that contains a pointer to a dynamically allocated array, a length (the number of elements currently stored), and a capacity (the maximum number of elements the vector can store without deallocation).
When a vector is created, it is initialized with a default capacity, and as elements are added or removed, the vector may reallocate its internal array to accommodate the new size.
Vec<Type>

We need a collection of values that can grow or shrink dynamically.
We need to store a large number of values of the same type.
We need efficient insertion, removal, and indexing operations.

let mut my_vec = Vec::new();
Vectors are created using the Vec type and the vec! macro.
We can access elements of a vector using indexing.
We can iterate over a vector using the iter method.
.push() method for adding element in the vector
.capacity() to look at the capacity of a Vec
Vec::with_capacity(8)
.into() to make an array into a Vec.

The difference between arrays and vectors is similar to the difference between &str and String: arrays are simpler, with less flexibility and functionality, and may be faster, while vectors are easier to work with because you can change their size.
Vectors are similar to arrays, but unlike arrays, vectors can grow or shrink dynamically, and they provide more flexibility and convenience methods.
Vectors are also similar to slice, but unlike slice, vectors own their data and can be resized.
