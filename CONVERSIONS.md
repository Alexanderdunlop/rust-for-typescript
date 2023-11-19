### Map

```ts
const foo = [1, 2, 3].map((x) => x + 1);
console.log(foo);
```

```rust
let foo: Vec<_> = vec![1,2,3]
    .iter()
    .map(|x| x + 1)
    .collect();
println!("{:?}", foo)
```

### Read file & Print each line

```ts
import fs from "fs";

fs.readFileSync("lines")
  .toString()
  .split("\n")
  .forEach((line) => console.log(line));
```

```rust
let file = std::fs::read_to_string("lines").unwrap();
file.lines().for_each(|line| println!("{}", line));
```

### Filtering

```ts
import fs from "fs";

fs.readFileSync("lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .forEach((line) => console.log(line));
```

```rust
let file = std::fs::read_to_string("lines").unwrap();
file
    .lines()
    .enumerate()
    .filter(|(idx, _)| idx % 2 == 0)
    .for_each(|(_, line)| println!("{}", line));
```

### Skip & Take

```ts
import fs from "fs";

fs.readFileSync("lines")
  .toString()
  .split("\n")
  .filter((_, i) => i % 2 === 0)
  .filter((_, i) => i > 1 && i < 4)
  .forEach((line) => console.log(line));
```

```rust
let file = std::fs::read_to_string("lines").unwrap();
file
    .lines()
    .enumerate()
    .filter(|(idx, _)| idx % 2 == 0)
    .skip(2)
    .take(2)
    .for_each(|(_, line)| println!("{}", line));
```

### Enums

```ts
enum Color {
  Red,
  Green,
  Blue,
}

function printColor(color: Color) {
  switch (color) {
    case Color.Red:
      console.log("red");
      break;
    case Color.Green:
      console.log("green");
      break;
    case Color.Blue:
      console.log("blue");
      break;
  }
}

printColor(Color.Red);
```

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    };
}

fn main() {
    print_color(Color::Red)
}
```
