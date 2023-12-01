const INPUT: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

#[test]
fn do_it() {
    assert_eq!(day01a::calc(INPUT), 142);
}
