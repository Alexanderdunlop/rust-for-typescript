fn main() {
    let items = vec![1,2,3]
        .iter()
        .map(|x| x + 1);

    println!("{:?}", items);
}
