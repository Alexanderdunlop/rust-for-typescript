## Intro

These are my notes from the course on [Rust for Typescript Developers](https://frontendmasters.com/courses/rust-ts-devs/).

### Assumptions

- I think you are a bad programmer (everyone is expect John Carmack).
- You can program Typescript with relative ease.
- You know types, maybe no wizard.

```js
// I assume everyone here can easily read this and understand what is happening.
// here just by type definitions.
type Promiseable<T> = {
  promise: Promise<T>,
  resolve: (value: T) => void,
  reject: (reason: any) => void,
};
type PromiseFactory<T> = () => Promiseable<T>;

function explodePromise<T>(): Promiseable<T> {
  // technically there would be some errors here, but just ignore that.
  let resolve, reject;
  let promise = new Promise((res, rej) => {
    resolve = res;
    reject = rej;
  });

  return {
    promise,
    resolve,
    reject,
  };
}
```

This is the level I would expect, this shouldn't be very hard for you to understand this is just simply a promise that has been exploded and returned. If you can understand how the generic plays into the promise we are looking pretty good.

The only confusion with this code should be why would you do this, this is a good interview question.

### Why Rust?

**Ergonomics**

For me, ergonomics is defined on two axes, one quickly being able to write software with low unexpected behavior, and two maintaining software longer.

This may seem like it would be the counter, as a lot of people say rust is a hard language. Its one of those things that if you can remember your first time programming everything was hard but anytime you have to learn something novel its always super hard. Rust just has more novel things than something like Typescript, Typescript is a fairly easy language you don't have too many hurdles to jump where as Rust has more hurdles but once you have done them there are things that are easy in Rust that are extremely difficult in Typescript.

**Javascripts(TS) vs Rust Design decisions**

- Specify Readonly vs Specify Mutability
- undefined/null vs Option
- errors being thrown vs being returned

**Goals**

You end up with enough knowledge to be able to google your way through a small to mid sized cli application in rust.

**What we wont cover**

- Errors - creating your own types
- Wasm / UI
- async
- smart pointers and interior mutability
- lifetimes - WHY NOT??
- macros, both proc macros and declarative macros
  - thse truly make rust amazing

```rust
// ----- v that is a marco!!!
return view! {
    <div>
        <MyCustomComponent name="hello" />
    </div>
}
```

### Comparisions

Variable

```js
const foo = 5; // sort of constant but not really
let foo = 5; // definitely not a constant
const foo = [] as const; // const pointer to a const..
                          // i understand if you don't c the joke
```

```rust
let foo = 5; // real constant
let mut foo = 5; // mutable
```

Shadowing

```js
// this wouldn't work in typescript
const foo = [...];
const foo = someMethod(foo);
```

```rust
let foo = [...]; // I am of Type A
let foo = someMethod(foo); // I am of Type B - works fine
```

Why though?
One thing that makes shadowing amazing is that you can change types.

```rust
let foo = get_file(args); // FileHandle
let foo = read_file(foo); // String
let foo = tokenize_and_do_things_to_string(foo); // Vec<String>
```

if

```js
if (condition && second || this_one) {
  ...
} else if ...
else
```

```rust
if condition && second || this_one {
  ...
} else if ...
else ...
```

Loops
For

```js
for (let i = 0; i < 10; i++) {
  // ...
}
```

```rust
for i in 0..10 {
}

// inclusive (includes 10)
for i in 0..=10 {
}
```

While

```js
while (true) {
  // ...
}
```

```rust
while true {
}
```

For ever?

```js
for (;;) {
  // while (true) {
  // ...
}
```

```rust
loop {
}
```

Collections?

```js
for (const [key, value] of Object.entries(obj)) {
  // ...
}
for (const value of [1, 2, 3]) {
  // ...
}
for (const idx in [1, 2, 3]) {
  // ...
}

// array#map // copies
// array#filter // copies
// array#forEach // iterates
// array#reduce // always a bad decision
// map#forEach // weirdest interface in the universe
```

Rust

```rust
for x in &some_array {
  // x will be each time of an array
}

vec![1,2,3]
  .iter()
  .map(...)
  // HUGE AMOUNT OF THINGS HERE
  // you can create your own...
  .collect::<Vec<_>>();
```

Functions

```js
function foo() {}
```

```rust
fn foo() {
}
```

Parameters

```js
function foo(arg1: number, arg2: number) {}
```

```rust
fn foo(arg1: f64, arg2: f64) {
  // numbers can be a bit complicated
}
```

Return
This is interesting in typescript. You may have to change some habbits

```js
// The return type is based on the code below
// function foo(): number {
function foo() {
  return 5;
}
```

```rust
fn foo() -> usize {
  return 5;
}
```

Closures

```js
(x) => {
  return x;
};

// or auto return x + 1 like
(x) => x + 1;
```

```rust
|x| {
  return x;
}

|x| x + 1
```

Class and Methods
This one is where the truest magic happens

```js
class Foo {
  properties...

  constructor() { ... }

  methods...

  static methods

  private methods

  protected methods // if you this i'll fire you
}
```

Pay real close attention

```rust
struct Foo {
  properties...
  properties
}

impl Foo {
  // these are both static methods
  fn this() // available usage within the file
  pub fn this() // available usage within the file

  // you should be able to understand this before the end of the day
  // and all of this can add pub
  // these are instance methods
  fn this(&self)...
  fn this(&mut self)...

  // public instance methods
  pub fn this(self)...
}
```

interfaces

```js
interface Foo {
  properties: type; // gross
  method(): retType;
}

interface Foo {
  hey_another_method(); // i feel many things about this
}
```

```rust
trait Foo {
  // no properties
  fn method(&self) -> retType;
}

impl Foo for MyStruct {
  ...
}
```

traits compose this may not sound big, but its AMAZING effectively prevents the need for inheritance.
