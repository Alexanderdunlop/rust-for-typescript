### Map

```ts
const foo = [1, 2, 3].map((x) => x + 1);
console.log(foo);
```

```rust
fn main() {
    let foo: Vec<_> = vec![1,2,3]
        .iter()
        .map(|x| x + 1)
        .collect();
    println!("{:?}", foo)
}
```
