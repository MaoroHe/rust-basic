fn main() {
    let file = std::fs::read_to_string("src/lines").unwrap();

    file
        .lines()
        .enumerate()
        .filter(|(idx, _)| idx % 2 == 0)
        .skip(2)
        .take(2)
        .for_each(|line| println!("{}", line.1));
}