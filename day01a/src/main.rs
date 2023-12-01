fn main() {
    let s = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", day01a::calc(&s));
}
