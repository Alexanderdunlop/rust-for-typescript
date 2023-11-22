To throw or not to throw, that is the questions.

- what function throws an error in JS
  A widely unknown common throw is `JSON.parse()`, people forget the try & catch with this.
  This is a common question people can't answer in JS as they don't think about it.
  Reading file can throw errors.
- who handles the error if thrown?
- what can be throw? (look at the promise reject definition (reason: any)) with JS you learn by trail.

How Rust Handles Errors
Errors are values
This means that there is no throwing. You get a value that is either a value or an error.
Notice this is very similar to Option.

- Option is value or undefined
- Result is value or error value

`if err != nil`
yes, the golang meme of error handling doesn't exist in rust. Rust handles errors better than go.

I am going to make some assumptions
You don't need me to show you another example of how to handle enums because that's all we have been doing.
The definition of a result

```rust
enum Result<V, E> {
    Ok(V),
    Err(E),
}
```

Also, rust has `Ok` and `Err` as first class citizens.

```rust
if let Ok(value) = a_function_that_can_error() {
    // something with the value
}

match a_function_that_can_error() {
    Ok(value) => println!("oh yeah, value! {}", value);
    Err(e) => println("oh no... {}", e);
}

// you don't care about the error
_ = a_function_that_can_error();
// Be careful about this espically with futures this will cause an error, as futures require assigment to excute.

// yolo
let foo = a_function_that_can_error().unwrap();

// respectful yolo
let foo = a_function_that_can_error().expect("should never fail");

// defaults
let foo = a_function_that_can_error().unwrap_or(0);

// convert to option
// Ok(V) => Some(V)
// Err(E) => None
// bai felicia
let foo = a_function_that_can_error().ok();

let foo = a_function_that_can_error()
    .map(|value| value + 1);

let foo = a_function_that_can_error()
    .and_then(|value| another_possible_error(value))
    .and_then(|value| again(value));
// and_then only gets excuted on successful step

// If your function returns an error, you can do this!
let foo = a_function_that_can_error()?;
```

Side Note
there are two crates (rust packages) that work very well errors

- thiserror - great for creating your own error, should be used in libraries
- anyhow - great for applications
  We will use anyhow shortly
