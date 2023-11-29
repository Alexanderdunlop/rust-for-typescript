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

```rust
#[derive(Debug)]
struct Item {
    count: usize
}

fn add_one(item: &mut Item) {
    item.count += 1;
}

fn print_all(items: &Vec<Item>) {
    for item in items {
        println!("{:?}", item);
    }
}

fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.first_mut();
    println!("{:?}", first);
    print_all(&items);
    println!("{:?}", first); // breaks a rule
}
```

```rust
fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.get_mut(0);
    let second = items.get_mut(1); // breaks a rule
    println!("{:?}", first);
}
```

You can only ever have one out, the best way to think of this is imagine you have an array if you change something in the array, the thing pointing might not be there so it would break.

```rust
fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.get_mut(0);
    let second = items.get_mut(1);
    println!("{:?}", second);
}
```

This is valid as the first flow ends as rust identifies that the first lifetime ends.

```rust
fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.get_mut(0);
    println!("{:?}", first);
    let second = items.get_mut(1);
    println!("{:?}", second);
}
```

This is also valid

```rust
fn main() {
    let mut items = vec![Item { count: 1 }];
    let first = items.get_mut(0);
    println!("{:?}", first);
    let second = items.get_mut(1);
    println!("{:?}", second);
    println!("{:?}", first); // this is now wrong
}
```

```rust
fn add_one(item: Item) {
    item.count += 1;
}

fn add_one(item: &mut Item) {
    item.count += 1;
}
```

This is the most common mistake starting out in Rust, this is done for you in other languages.

```rust
fn main() {
    let items = vec![1,2,3]
        .iter()
        .map(|x| x + 1);

    println!("{:?}", items);
}
```

Why does this error, iter takes a reference to self, then returns a reference to the items its going over.
The vec![1,2,3] is only being held onto temp as no one is pointing at it.

```rust
fn main() {
    let data = vec![1,2,3];
    let items = data
        .iter()
        .map(|x| x + 1);

    println!("{:?}", items);
}
```

If Rust cannot guaranttee it is safe, it will fail it.
Rust does exactly what you tell it to do at all points.

Rust will automatically drop values for, you can manually do this but it is quite advanced `drop()`.

```rust
fn main() {
    let items: i32 = vec![1, 2, 3]
        .iter()
        .map(|x| x + 1)
        .sum();

    println!("{:?}", items);
}
```

```rust
let first = items.get_mut(0);
// same as
let first = items[0];
// expect it is a mut ref instead of just a ref
```
