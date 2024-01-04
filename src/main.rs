fn main() {
    let filename = std::env::args().nth(1)
        .expect("please provide a filename");

    let file = std::fs::read_to_string(filename)
        .expect("file doesnt exist.");

    file
        .lines()
        .for_each(|line| {
            if let Ok(x) = line.parse::<usize>() {
                println!("{}", x);
            } else {
                println!("Line not a number")
            }
        });
}
