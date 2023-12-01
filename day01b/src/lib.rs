pub fn calc(s: &str) -> i32 {
    let mut nums = vec![];
    for line in s.lines() {
        let c1 = get_num(line);
        let c2 = get_num(&line.chars().rev().collect::<String>());
        nums.push(format!("{c1}{c2}").parse::<i32>().unwrap());
    }
    nums.iter().sum()
}

fn get_num(line: &str) -> i32 {
    let len = line.len();
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            return c.to_digit(10).unwrap() as i32;
        }
        if len > i + 5 {
            match &line[i..][..5] {
                "three" | "eerht" => return 3,
                "seven" | "neves" => return 7,
                "eight" | "thgie" => return 8,
                _ => (),
            }
        }
        if len > i + 4 {
            match &line[i..][..4] {
                "four" | "ruof" => return 4,
                "five" | "evif" => return 5,
                "nine" | "enin" => return 9,
                "zero" | "orez" => return 0,
                _ => (),
            }
        }
        if len > i + 3 {
            match &line[i..][..3] {
                "one" | "eno" => return 1,
                "two" | "owt" => return 2,
                "six" | "xis" => return 6,
                _ => (),
            }
        }
    }
    println!("{}", line);
    panic!("no numbers!");
}
