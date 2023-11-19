Rust enums are incredible
They are nothing like TypeScripts enums, and a reason why rust, for a static typed language, is so good.

If a match statement using a enum you must use all the enum values, unlike in JavaScript switch statements.

```rust
enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false;
    }
    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}
```

To do this in TypeScript you would have to go through a module, but with Rust its on the enum itself.
