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
