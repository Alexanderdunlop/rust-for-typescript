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
    append(&mut items);
}