Hopefully you "understand" rust now
The selling point of rust should become more clear now. Why people love it. Option, Result, and Iterators are a foundation of the items you work with and allow for highly optimized code with great readability.

But there is one more concept that we need to go over. The Borrow Check, This is the item that everyone always talks about when it comes to Rust and difficulty. I would argue its not that difficult.
What you just got done learning is more difficult.

But first
These next parts are teaching you some of the rust fundamentals that just don't have a typescript side by side. To be able to use rust effectively, we need to cover these.

Stack vs Heap?
You have probably heard this term throughout your career. It may confuse you what they are. To understand rust, it is required you have a vary simple understanding of the slack.

- first stack overflow
- struct
- struct with Vec
- storing struct in Vec
- storing struct with Vec inV Vec
- (whiteboard everything)
- now talk about Option
  for every heap allocation there is a stack allocation

```rust
fn foo() {
    foo();
}

fn main() {
    foo();
}
```

This will call a stack overflow, when ever you call a function something has to be loaded into memory. If you think about it you have a pointer somewhere in your program walking through your code, its going line by line excuting every last instruction. But at some point we need to jump into a function.
You have two memory regions:

- A stack (a smaller memory place, but super fast)
- The Heap (a gaint memory place)

What happens is you have something called the stack pointer, when we call a function we need to know what the return address is where do go back after we finish the function, this happens it happens in javascript but we just don't experience it. There are also parameters, they maybe refs or the literally values. Then you have some sort of return value.
When we did the foo infinite function, it went on until we ran out of memory on the stack (stack overflow).

```rust
struct MyStruct {
    age: usize
}

fn main() {
    let foo = MyStruct { age: 0 };
}
```

What happens here is we assign the Struct to a position in the Stack which is going to be 8 bytes, what ever the address is eg `0x....69420`.
Its easy to do that on the stack, because you take your stack order and add 8, you just got yourself 8 bytes of memory region. If you need to refer to your stack it can just jump right there very fast.

```rust
struct MyStruct {
    age: Vec<usize>
}

fn main() {
    let foo = MyStruct { age: vec![] };
}
```

What happens here:
Instead of being 8 bytes of age we need to store something else, you could imagine it stores a pointer. Which points to somewhere over into the Heap, this is the place I can grow my memory, this is where I can store all my usizes. Then it will store a length on the stack, that way I know how many items I have. Then it can store some kind of capacity, so that way it knows when it needs to resize, when you keep adding items its going to run out of memory that it has allocated, it normally starts of around 5 which is the default one, it has some low amount then it has to grow in memory. 