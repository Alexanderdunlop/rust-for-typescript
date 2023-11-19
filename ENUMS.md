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

One small argument
Most of what rust can do, javascript can do, but differently.
You could imagen that a javascript module exists for Color where the function is_green and is_green_parts are defined and exported. But I would argue that having to peruse through a module to know what operations are supported is not nearly as nice as having them hang off the struct itself. And in this case, the enum.

```ts
type Custom = {
  age: number;
  name: string;
};

type Item = number | string | Custom;

function append(items: Item[]) {
  items.push("Hello Fem!");
}

const items: Item[] = [];

console.log(items);
append(items);
console.log(items);

const numbers: number[] = [];
console.log(numbers);
append(numbers);
console.log(numbers);
```

This prints:

```sh
[]
[ 'Hello Fem!' ]
[]
[ 'Hello Fem!' ]
```

There is a string in the number array.

Rust enums can have subtypes, we are able to add type descrimination. In a way that there is no way to add a type descriminatior.

Under the hood the rust enum is just a union type in C. The number is type 0 and has a usize for its value, the string is type 1 and has a String for its value, the MyCustom is type 2 and has a Custom for its value. An enum should be the size of whatever your largest data type is under the hood in memory.

```rust
struct Custom {
    age: usize,
    name: String
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello Fem".into()));
}

fn main() {
    let mut items: Vec<Item> = vec![];
    append(&mut items);

    let mut items: Vec<usize> = vec![];
    append(&mut items); // Safety error is here
}
```

This means no more typescript:

```ts
if (typeof x === "number") {
  // ...
}

if ("bar" in x) {
  // ...
}
```

So no more "magic" checking for types, you get named types and this works very well with non type discriminated unions (what we made). This is because the discrimination exists at a language level, not a type: string level
its not all magic.
Sometimes code can become a bit more verbose because of this, and that isn't as nice to write. But at the same time, it prevents easy errors where you forgot to handle cases.

### Pattern Matching

```rust
struct Custom {
    name: String,
    age: usize
}

enum Item {
    Number(usize),
    Custom(Custom),
    String(String),
}

fn main() {
    let foo = Item::Number(5);

      match &foo {
        Item::Number(num) => println!("i am a number: {}", num)
        Item::String(str) => println!("i am a string: {}", str)
        Item::Custom(custom) => println!("name: {}, age: {}", custom.name, custom.age),
      }

      match &foo {
        Item::Custom(custom) =>
            println!("name: {}, age: {}", custom.name, custom.age),
        _ => {}
      }

      match &foo {
        Item::Custom(Custom {
            age,
            ..
        }) => println!("age: {}", age),
        _ => {}
      }

      match &foo {
        Item::Custom(custom) if custom.name == "Ricky" =>
            println!("Hi, Ricky"),
        Item::Custom(custom) if custom.age > 33 =>
            println!("N64 was the best console"),
        Item::Custom(custom) if custom.age < 30 =>
            println!("Xbox wa the best console"),
        _ => {}
      }
}
```

There are SO many problems that can be solved by good pattern matchings, its wild.
