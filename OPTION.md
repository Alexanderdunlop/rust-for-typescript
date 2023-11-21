Option
Option is the answer to typescript's undefinded / null problem.
Option is an enum.

The thing about null / undefined is that you get different answers for why you should use what or the other...
null:undefined on purpose
undefined: we have no reasonable guarantee it will be there...

```ts
type Foo = {
  bar?: string;
};

const item3: Foo = { bar: undefined };
```

```rust
enum Option<T> { // yes, generics can be used in enums, again, cool
  None
  Some(T),
}
```

```rust
Some(5);
Option::Some(5);
```

But why?
Why do we need Options in rust? the answer is memory. If you might or might not return an item from a function, rust needs to be able to allocate that memory on the stack. (we will talk about this more shortly)

Working with Option
They are enums, so match/if let pattern matching works, but there is more because there are plenty of convenient methods.

```rust
fn main() {
  let foo: Option<String> = None;

  if let Some(x) = foo {
    // lifts the optional out
  }
}
```

```ts
function multiply(value: number | undefined): number {
  return (value ?? 0) * 5;
}
```

If i was starting out in rust this is what i would probably write:

```rust
fn multiply(value: Option<usize>) -> usize {
    if value.is_none() {
        return 0
    } else {
        return value.unwrap() * 5;
    }
}
```

Then you get better and write it like this:

```rust
fn multiply(value: Option<usize>) -> usize {
    if let Some(x) = value {
        return x * 5;
    }
    return 0;
}
```

But then there is still better like this:

```rust
fn multiply(value: Option<usize>) -> usize {
    return value.unwrap_or(0) * 5;
}
```

```ts
function multiply(value: number | undefined): number | undefined {
  return value === undefined ? undefined : value * 5;
}
```

```rust
fn multiply(value: Option<usize>) -> Option<usize> {
    return value.map(|x| x * 5);
}
```

```rust
fn multiply(value: Option<usize>) -> Option<usize> {
  return Some(value? * 5);
}
```

This will return undefined if undefined is passed.

```rust
let value = value?;
```

what this does under the hood is

```rust
let value = match value {
  Some(x) => x,
  None => return None,
}
```

In go this is the same as `if err != nil`
