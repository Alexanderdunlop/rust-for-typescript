```ts
import fs from "fs";

const fileName = process.argv[2];

if (fileName) {
  fs.readFileSync(fileName)
    .toString()
    .split("\n")
    .forEach((line) => console.log(line));
}
```

```rust
fn main() {
    let file_name = std::env::args().nth(1)
        .expect("the file name to be passed in");

    let file = std::fs::read_to_string(file_name)
        .expect("unable to read the file to string");

    file.lines().for_each(|line| println!("{}", line));
}
```

Compare TypeScript w/ Rust
what makes rust better or worse in this example?

You find errors at complie time instead of runtime.

TypeScript is super fast but if something goes wrong the slow down is more.

```ts
import fs from "fs";

const fileName = process.argv[2];

if (fileName) {
  fs.readFileSync(fileName)
    .toString()
    .split("\n")
    .forEach((line) => {
      const print = parseInt(line);
      if (isNaN(print)) {
        console.log("Line not a number");
      } else {
        console.log(print);
      }
    });
}

// Prints
// 1
// 5
// 9
// 33
// Line not a number
```

```rust
fn main() {
    let file_name = std::env::args().nth(1)
        .expect("the file name to be passed in");

    let file = std::fs::read_to_string(file_name)
        .expect("unable to read the file to string");

    file.lines().for_each(|line| {
        if let Ok(value) = line.parse::<usize>() {
            println!("{}", value)
        } else {
            println!("Line not a number")
        }
    });
}

// Prints
// 1
// 5
// 9
// 33
```

A Case for rust
In the simplest sense, you always know where your errors happen, you always know when undefineds can happen.

- Result saves you from errors you should be able to prevent
- Option saves you from undefined is not a function
- Rust doesn't save you from bad logic, we are all programmers

```rust
file
  .lines()
  .filter_map(|line| line.parse::<usize>().ok())
  .for_each(|line| {
    println!("{}", value);
  })
```

This will filter out any not numbers.
