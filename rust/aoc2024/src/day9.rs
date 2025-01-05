use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 9;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year,
        day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let input = reader.lines().next().ok_or("wtf no input")??;

    // Your solution goes here
    let mut content: Vec<usize> = Vec::new();
    let mut blank: Vec<usize> = Vec::new();
    for (i, n) in input
        .chars()
        .map(|c| c.to_digit(10).ok_or("char is non digit"))
        .enumerate()
    {
        let num = n? as usize;
        match i % 2 == 0 {
            true => content.push(num),
            false => blank.push(num),
        }
    }
    println!("{:?}", content);
    println!("{:?}", blank);

    let mut counter: usize = 0;
    let mut is_content: bool = true;
    let mut blank_p: usize = 0;
    let mut blank_consumed: usize = 0;
    let mut head_p: usize = 0;
    let mut head_consumed: usize = 0;
    let mut tail_p: usize = content.len() - 1;
    let mut tail_consumed: usize = 0;
    let mut res = 0;

    while !(blank_p == blank.len() - 1 && blank_consumed == *blank.last().unwrap())
        && !(head_p == tail_p && head_consumed + tail_consumed >= content[head_p])
    {
        if is_content {
            if head_consumed == content[head_p] {
                is_content = false;
                head_p += 1;
                head_consumed = 0;
                continue;
            }
            res += counter * head_p;
            head_consumed += 1;
            counter += 1;
        } else {
            if blank_consumed == blank[blank_p] {
                is_content = true;
                blank_p += 1;
                blank_consumed = 0;
                continue;
            }
            if tail_consumed == content[tail_p] {
                tail_p -= 1;
                tail_consumed = 0;
            }
            res += counter * tail_p;
            blank_consumed += 1;
            tail_consumed += 1;
            counter += 1
        }
    }

    println!(
        "hp : {}, hc : {}, tp : {}, tc : {}, bp : {}, bc : {}",
        head_p, head_consumed, tail_p, tail_consumed, blank_p, blank_consumed
    );

    while !(head_p == tail_p && head_consumed + tail_consumed >= content[head_p]) {
        if head_consumed == content[head_p] {
            head_p += 1;
            head_consumed = 0;
            continue;
        }
        res += counter * head_p;
        head_consumed += 1;
        counter += 1;
    }

    println!("{}", res);
    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // let current_file_path = Path::new(file!());
    // let year = 2024;
    // let day = 9;
    // let path = current_file_path
    //     .parent()
    //     .unwrap()
    //     .join(format!(
    //         // "../../../questions/{}/{}/example2.txt",
    //         "../../../questions/{}/{}/input2.txt",
    //         year, day
    //     ));
    //
    // let file = File::open(path)?;
    // let reader = io::BufReader::new(file);

    // Your solution goes here

    Ok(())
}
