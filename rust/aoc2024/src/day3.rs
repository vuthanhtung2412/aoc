use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 3;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;
    let mut res = 0;
    for l in reader.lines() {
        let line = l?;
        for (full_match, [e1, e2]) in re.captures_iter(&line).map(|c| c.extract()) {
            println!("{:?} : {:?},{:?}", full_match, e1, e2);
            res += e1.parse::<i32>()? * e2.parse::<i32>()?;
        }
    }
    println!("{}", res);

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 3;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example2.txt",
        "../../../questions/{}/{}/input2.txt",
        year,
        day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(don't\(\))|(do\(\))").unwrap();

    let mut res = 0;
    let mut _do = 1;
    for l in reader.lines() {
        let line = l?;
        for c in re.captures_iter(&line) {
            match c.get(1) {
                Some(e1) => {
                    res += _do
                        * e1.as_str().parse::<i32>()?
                        * c.get(2)
                            .ok_or("2nd element not found")?
                            .as_str()
                            .parse::<i32>()?
                }
                None => match c.get(0).ok_or("WTF")?.as_str() {
                    "do()" => {_do = 1},
                    "don't()" => {_do = 0},
                    _ => return Err("WTF".into())
                },
            }
        }
    }
    println!("{}", res);

    Ok(())
}
