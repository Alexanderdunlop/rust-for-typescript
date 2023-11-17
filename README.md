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
