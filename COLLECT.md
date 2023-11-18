```rust
fn main() {
    let items = vec![1,2,3];
    let mut iter = items
        .iter()
        .map(|x| x + 1);

    let mut collected_items = vec![];

    while let Some(x) = iter.next() {
        collected_items.push(x);
    }

    println!("collected_items: {:?}", collected_items);
}
```

Collect into string! Like string join

```rust
let foo: String = vec!["this", "is", "a", "test"]
    .into_iter()
    .collect();
```

HasSet & HashMap

```rust
let foo: HashSet<isize> = vec![1,2,3]
    .into_iter()
    .collect();

let foo_two: HashMap<&str, usize> = vec!["this", "is", "a", "test"]
    .into_iter()
    .enumerate() // Adds the index to the iterator!
    .map(|(idx, item)| (item, idx)) // reverses the order
    .collect();
```

HashMap is effectively the same thing as a Map in JavaScript

`map(|idx, item|)` is an example of destructuring.

`map(|)`

```rust
let value: usize = vec![1,2,3]
    .iter()
    .sum();

let how_many_items: usize = vec![1,2,3]
    .iter()
    .skip(2)
    .count();
```

```rust
vec![1,2,5,9,4]
    .iter()
    .skip(2)
    .take_while(|&&x| x > 4)
    .for_each(|x| println!("{}", x));
// 5 9
```

```rust
let what_about_this: usize = vec![1,2,3]
    .iter()
    .filter(|x| *x % 2 == 0)
    .count();

// 1
```

```rust
let map = HashMap::from([
    ("foo", 1),
    ("bar", 2),
    ("baz", 3),
]);

map
    .iter()
    .for_each(|k, v| println!("{}: {}", k, v));
```

```rust
let set = HasSet::from([
    "foo",
    "bar",
    "baz",
]);

set
    .iter()
    .for_each(|v| println!("{}", v));
```

```rust
let todos = Todo { ... values ... }

for task in &todos {
    println!("I need to do: {}", task);
}
```

- `iter` doesn't own underlying data
- `into_iter` does own underlying data
