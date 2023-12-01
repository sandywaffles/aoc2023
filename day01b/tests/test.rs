const INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn do_it() {
    assert_eq!(day01b::calc(INPUT), 281);
}
