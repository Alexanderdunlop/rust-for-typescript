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

### Numbers

```js
// We would call this an interger in rust
4 // this... is technically a smi, but for you purpose, its a  number

// interger becomes a float auto_magically_ in js
4 / 3 = 1.3333333333

// this is totally cool
4 * -1 = -4
```

None of that was cool for rust
Rust you have to specify the types <NUMBER> = power of two
i<NUMBER> = an integer that can be negative or positive (signed)
u<NUMBER> = an integer that can be positive only (unsigned)
f<NUMBER> = a number that requires decimal point
usize = a u<NUMBER> where <NUMBER> is your system arch. (64bit = u64)
isize = a i<NUMBER> where <NUMBER> is your system arch. (64bit = i64)

```rust
4 / 3 = 1

// cannont divide {float} by {integer}
// yes... this is an error
4.0 / 3 = Nope

// 4 is an i32
4 * -1 = -4

// 4 is an i32
let foo = 4u23; // saying its a 4 that is a u32 (defining type)
foo * -1 // ERROR
```

If you have ever worked with any static language, this should be pretty straight forward.

**The difference between String and &str**
Yes, you will see there are two types of strings you commonly run into. So what are they?

String

- Well String is a heap allocated (heap may be a foreign word to you)
- String can be mutable

&str

- this points to a sequence of utf-8 characters. Its commonly called a slice. Its a view into a String
- its immutable
- its analogous to &[u8]
  So if i say String, i mean String and if i say stir i mean &str (quick whiteboarding)

The best way to think about this is in JavaScript is you were to pass a string around you don't have to copy all the data. In JavaScript land when you do substring do you get a new string or a reference I honestly have no idea.

You don't have control in JavaScript on how memory works.

**Basics on Rust**

Just some basics so we can understand things going forward

When you are starting out using rust you should see

1. unwraps
2. clonse

That is totally normal, completely fine. understanding, at least for me, comes in waves. The more I understand, the more I realize I understand less.

**Vector**
Vec (rust) and [](typescript) are very similar. Their behaviors are near identical.
These two are functionally equivalent

```js
const a = [1,2,3,4,5] as const;
```

```rust
let a = vec![1,2,3,4,5];
```

Mutation

```js
const a = [1, 2, 3, 4, 5];
a.push(6); // [1,2,3,4,5,6] // returns size
```

```rust
let a = vec![1,2,3,4,5];
a.push(6); // Error: a is not mutable

// but with rust we can shadow
let mut a = a;
a.push(6); // [1,2,3,4,5,6] // does not return size
```

```js
const a = [1, 2, 3, 4, 5];
a.pop(); // [1,2,3,4] undefined or T
```

```rust
let mut a = vec![1,2,3,4,5];
a.pop(); // [1,2,3,4] Option<T>
```

Accessing Data

```js
const a = [1,2,3,4,5] as const;
const item = a[2];
```

```rust
let a = vec![1,2,3,4,5];
let item = a[2]; // does work, but if out of bounds, panic
let item = a.get(2); // better, returns Option<T> where T can be i32
```

An Option<T> is a possible undefined value. All things could be undefined, must be specified with an Option.
We will talk about enums an Options in depth shortly

Tuple
This doesn't really have a similarity in javascript

```rust
let a = (5, String::from("hello")); // this type is (i32, String)
```

it is "near" equivalent to

```js
const a = [5, "hello"];
```

You can pattern match (think destructuring) tuples;

```rust
let a = (5, String::from("hello")); // this type is (i32, String)

// you probably best know thius as destructuring, but we will refer to this
// as pattern matching.
let (my_num, my_str) = a;
```

You can even pattern match in a function

```rust
let a = (5, String::from("hello")); // this type is (i32, String)

fn bar((my_num, my_str): (i32, String)) {

}

bar(a)
```

Structs

```rust
struct MyStruct {
  x: usize,
  y: usize,
  z: usize,
}

fn bar(MyStruct { y, z, .. }: MyStruct) -> bool {
  return y * z < 100;
}

fn main() {

  let foo = MyStruct {
    x: 69,
    y: 420,
    z: 1337
  }

  let MyStruct { x, .. } = foo;
  let MyStruct { x, y, .. } = foo;
  let MyStruct { x, y, z, .. } = foo;

  if let MyStruct { x, .. } = foo {
    println!("things about x {}", x)
  }
}
```

unwrap, todo, u

```rust
// todo is good for throwing an error when you want to implement the function later
fn not_complete_fn(x: usize) -> bool {
    if x < 10 {
        return true;
    }

    todo!("finish this later");
}

// unreachable will throw an error if the code is reached
fn only_evens(x: usize) -> bool {
    if x % 2 == 1 {
        unreachable!("this should never happen");
    }

    return true;
}

// unwrap will force the optional to be not optional but it is dangerous
fn main() {
    let foo = Some(5);
    let bar = foo.unwrap();

    println!("Hello, world!");
}
```

### Value, Mutable, Immutable

One more thing about rust
You will hear me say the following words

- value
- mutable reference or mutable borrow
- immutable reference or immutable borrow

In rust you can have:

```
x = 5
The 5 is a value, it is something that is sitting there in memory. It is the thing.
The owner of said value is x, in rust land you can refer to a value.
y = &x
That means y refers to x, if you were to print out y it would always be the same as x. This is a read only reference.
y = &mut x
This means not only can I look at the data but it means I can also change the data. Not really done in any other languages. For you to be able to do &mut the original variable needs to be mut so:
mut x = 5

fn do_this(&self)
This means its a function that refers to self but it is not self it does not contain the value but it contains a reference.

fn do_this(&mut self)
This means you can only call it on a function on a object that is defined as mutable, that way you can seperate functions that mutate vs functions that don't mutate.
```

- reference: means readonly
- mutable reference: means write & read
- value: means the thing itself

Borrow checker:
In rust when you program there can be only one value owner, you can have as many readonly references as long as there are no write and read references out. You can only have one write reference and no read references.

More technical version of mutable reference or immutable reference is mutable borrow or immutable borrow.
This will be in the complier errors.

() is a unit
() is called "unit". Effectively it is "nothing" value.

```rust
if true {
  println!("foo");
}

// I could write this
// --v this has type ()
let foo = if true {
  println!("foo");
}
```

You will see this often:
Won't make sense right now.

```rust
fn only_evens(x: usize) -> Result<()> {
  return Ok(())
}
```

This means i'm returning the unit of nothing, it doesn't exist but hey we are okay. Everything went Ok in this function, but we used the nothing unit because we have no value. Sometimes you have functions that have side effects that don't actually do anything for you. It's not like undefined.

### Coding an Iterator

To begin with, There are 2 things that have to be understood in Rust.
This is for fundamental understanding of the language.

1. Iterators
2. Enums

Once you get these two, it becomes easier to work with rust initially. As these concepts are a bit wonkey coming from TypeScript.

Often in TypeScript you iterate but you don't use iterators.

Lets start with iterators.

```
// If you have a vector which is just a list, a iterator is a seperate data structure.
// That can walk through that data.
// Doesn't matter what the data structure is, if there is an order an iterator can walk through it.
vec![1,2,3]
{
  prt => "["
  idx:
}
// You would have a reference to this iterator and call something like .next() this will give the next value out.
```

Iterators
I think iterators will make the easiest transition as they have the strongest similarity in javascript and we can start with `.map`

.map is not an `iterator` it `iterates` not the same but quite similar but completely different at the same time.

### Closures

Closures are different in JavaScript, when we refer to closures in JavaScript we are saying the value is closed of the function and can't be accessed outside of the closure.

In rust a closure is: `|x| x + 1`
In JavaScript this is a lambda function i.e. `x => x + 1`

What is collect?
The Iterator is its own data type. So we must convert from an iterator back into the struct we want and in our case its a Vec

### Borrow checker example

This is an error

```rust
let mut foo = vec![1,2,3]
  .iter()
  .map(|x| x + 1);

while let Some(x) = foo.next() {
}
```

This is because this iterator refers to the vec but who owns the vector nobody, how long does this value live it was created temp and then it went away. So now we have a reference to nothing so this needs to be.

```rust
let data = vec![1,2,3];
let mut foo = data
  .iter()
  .map(|x| x + 1);

while let Some(x) = foo.next() {
}
```

Its important to think how things live in memory.
