fn main() {
    let og_items = vec![1, 2, 3];
    let mut arr = og_items
        .iter()
        .map(|x| x+1);

    let mut items = vec![];

    while let Some(x) = arr.next() {
        items.push(x);
    }

    println!("{:?}", items);
}
