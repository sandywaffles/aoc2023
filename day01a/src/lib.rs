pub fn calc(s: &str) -> i32 {
    let mut nums = Vec::new();
    for line in s.lines() {
        let c1 = line.chars().find(|c| c.is_numeric()).unwrap();
        let c2 = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        nums.push(format!("{c1}{c2}").parse::<i32>().unwrap());
    }
    nums.iter().sum()
}
