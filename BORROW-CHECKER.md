Terminology
Dropped - releasing memory

```rust
{
    // a
}
```

`a` is dropped or released in memory.

There are THREE rules you must have in your head at all times.

1. There can only be `one` value owner
2. There can be `unlimited` immutable borrows (reference) with `no` mutable references
3. There can be only `one` mutable reference and `no` immutable references

One thing you can think about is if you have any moves if something gets moved while you have a reference you would be refering to memory thats no longer valid.

There are all the safe programs and then there are the safe programs Rust allows this is due to Rust not knowing wether they are safe.

There is one rule for

1. A reference cannot outlive its value

Stated differently
One var owns the the data
One var can change the data
Many vars can look at the data
You cannot look and change the data simultaneously
You cannot refer to something that has been dropped (released in memory)

```rust
#[derive(Debug)]
struct Item {
    count: usize
}

fn add_one(mut item: Item) {
    item.count += 1;
}

fn main() {
    let item = Item { count: 1 }; // item owns Item
    println!("{:?}", item);
    add_one(item); // fn add_one now owns the Item
    // add_one consumes the value
    println!("{:?}", item);
}

// borrow of moved value: `item`
// value borrowed here after move
```

fn add_one didn't get passed a reference it got passed the value itself.

add_one `consumes` the value

```rust
#[derive(Debug)]
struct Item {
    count: usize
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn main() {
    let mut item = Item { count: 1 }; // item owns Item
    println!("{:?}", item);
    add_one(&mut item); // item still owns Item
    println!("{:?}", item);
}
```

add_one now `consumes` a reference to the value

What rule are we breaking?

1. There can only be `one` value owner <----
2. There can be `unlimited` immutable borrow (reference) with `no` mutable references
3. There can be only `one` mutable reference an `no` immutable references
