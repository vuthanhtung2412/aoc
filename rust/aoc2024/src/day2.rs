use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 2;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let reports = reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res = 0;
    for r in reports{
        if check_report(r.as_slice())? {
            println!("true");
            res += 1
        } else {
            println!("false");
        }
    }

    println!("{}", res);
    Ok(())
}

fn check_report(report: &[i32]) -> Result<bool, Box<dyn std::error::Error>> {
    if report.is_empty() {
        return Err("Report of length 0".into());
    }

    let mut is_increasing = true;
    let mut prev = 0;
    for (i, level) in report.iter().enumerate() {
        if i == 0 {
            prev = *level;
            continue;
        }
        if i == 1 {
            is_increasing = *level > prev;
        }
        if is_increasing {
            if *level <= prev || *level - prev > 3 {
                return Ok(false);
            }
        } else if *level >= prev || prev - *level > 3 {
            return Ok(false);
        }
        prev = *level
    }
    Ok(true)
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // let current_file_path = Path::new(file!());
    // let year = 0;
    // let day = 0;
    // let path = current_file_path
    //     .parent()
    //     .unwrap()
    //     .join(format!(
    //         // "../../../questions/{}/{}/example2.txt"),
    //         "../../../questions/{}/{}/input2.txt",
    //         year, day
    //     ));
    //
    // let file = File::open(path)?;
    // let reader = io::BufReader::new(file);

    Ok(())
}
