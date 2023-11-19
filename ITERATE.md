Iterator way of thinking
This is an important concept which isn't in javascript

[Type] -> [Iterator] -> [Type]

This typically gives us code that looks like.

```rust
some_type
    .iter() // creates iterator
    .filter(|x| ...

    ) // A series of combinators

    .collect/sum/count/for_each() // some operation that takes the iterator
```

Split
that takes substring and creates an array.
That means calling split iterates the entire string up front and creates a list

Are they new strings or are they references to the string. I have no idea, it's no easy to know.

What about filter?
Filter takes in a list and produces a new list

forEach goes through each item, it's a great method.

What would the code actually looks like.

```ts
function filter_1(x: number): boolean {
  return x % 2 === 0;
}

function filter_2(x: number): number {
  return x >= 2 && x < 4;
}

// Skipping the split operation
let a = contents.toString().split("\n");
let b = [];
for (let i = 0; i < a.length; i++) {
  if (filter_1(a[i])) {
    b.push(a[i]);
  }
}
let c = [];
for (let i = 0; i < b.length; ++i) {
  if (filter_2(i)) {
    c.push(a[i]);
  }
}
for (let i = 0; i < c.length; ++i) {
  console.log(c[i]);
}
```

v8 may optimize some of this away. To what extent, I don't have the faintest clue and neither do you.

```rust
let mut start = 0;
let mut taken = 0;
let mut skipped = 0;
let mut lines_found = 0;
for (idx, c) in lines.enumerate().chars() {
    if c !== "\n" {
        continue;
    }

    // doesn't copy, just a &str (prt, len)
    let slice = lines[start..idx];
    start = idx + 1;

    lines_found += 1;
    if lines_found % 2 == 0 {
        continue
    }

    if skipped < 2 {
        skipped += 1;
        continue;
    }

    taken += 1;
    println!("{}", slice);

    if taken == 2 {
        break;
    }
}
```

Rust is a pull operation, JavaScript is a push operation.
It pushes you the values, rust you have to pull out the values.

If this file was 10K lines long, in Rust we would process 8 lines. In JavaScript we would process 10K + 10K + 5K + 2 lines.

This is the benifits of having an iterator instead of a mapped method. An iterator is a seperate structure in which either refers to or owns a value, and walks through the values lazyily. When you call next if gives you the next value, it doesn't process anything it does that one atomic operation.

**Zero cost abstractions**
You will see this phrase commonly in the rust community, and this is why. Its able to have these higher order abstractions, just without all the cost of them.
