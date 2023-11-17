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
