If a type has a trait, it can do things it coudn't do before.

When you define a trait, Rust creates a virtual table (vtable) that contains pointers to the implementaions of the trait's methods for each type that implements it.
Rust uses the vtable to dispatch the correct method implementation for a given type.

allow you to write code that can work with multiple types, making it easier to write reusable and flexible code.
provide a way to abstract away the implementation details of a type, allowing you to focus of the interface and behavior of the type.
help ensure type safety by allowing the compiler to check that a type implements the required methods, preventing type-related errors at runtime.

Rust uses a special syntax called attributes to automatically implement traits like Debug.
#[derive(Debug)]
To make a trait, write trait and then create some methods for it:
struct Dog {
    name: String,
}
struct Parrot {
    name: String,
}
trait DogeLike {
    fn bark(&self) {
        println!("Woof woof!");
    }
    fn run(&self) {
        println!("The dog is runnint!");
    }
}
impl DogLike for Dog {}
impl DogLike for Parrot {}
