fn not_complete_fn(x: usize) -> bool {
    if x < 10 {
        return true;
    }

    todo!("finish this later");
}

fn only_evens(x: usize) -> bool {
    if x % 2 == 1 {
        unreachable!("this should never happen");
    }

    return true;
}

fn main() {
    let foo = Some(5);
    let bar = foo.unwrap();

    println!("Hello, world!");
}
