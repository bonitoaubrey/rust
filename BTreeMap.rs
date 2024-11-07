A BTreeMap in Rust is a data structure that stores key-value pairs in a sorted, balanced tree, allowing for efficient insertion, deletion, and search operations.

A BTreeMap uses a self-balancing binary search tree (B-tree) to store its elements.
Each node in the tree represents a key-value pair, and the tree is organized in a way that keeps the height of the tree relatively small, ensuring efficient search and insertion operations.
When a new key-value pair is inserted, the tree is rebalanced to maintain is sorted order and balance.

to store a large number of key-value pairs and need to perform frequent search, insertion, and deletion operations.
It's self-balancing property ensures that the tree remains roughly balanced, which leads to efficient operations even with a large number of elemnts.

use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map.get("key1")); // prints Some("value1")
    println!("{:?}", map.get("key3")); // prints false

    for (key, value) in &map {
        println!("{} {}", key, value);
    }
}

BTreeMap maintains its elements in a sorted order, whereas
    HashMap does not.
