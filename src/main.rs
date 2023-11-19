fn main() {
    let file = std::fs::read_to_string("lines").unwrap();
    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}
