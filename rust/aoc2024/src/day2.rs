use itertools::Itertools;
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
    for r in reports {
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

/*
* Different way to solve
* 1. Remove 1 and check -> O(1) space complexity + O(n^2) time complexity
* 2. Greedy
*/
pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 2;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example2.txt",
        "../../../questions/{}/{}/input2.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut res = 0;
    for line in reader.lines() {
        // Not happy with this unwrap
        if check_report_skip_unused(
            &line?
                .split_whitespace()
                .map(|w| w.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )? {
            res += 1;
        }
    }

    print!("{}", res);

    Ok(())
}

fn check_report_skip_unused(report: &[i32]) -> Result<bool, Box<dyn std::error::Error>> {
    if report.is_empty() {
        return Err("empty report".into());
    }

    if report.len() == 1 {
        return Ok(true);
    }

    let &l0 = report.first().ok_or("?")?;
    let &l1 = report.get(1).ok_or("?")?;
    if check_report_skip_used(&report[2..], l0, None)?
        || check_report_skip_used(&report[2..], l1, None)?
    {
        return Ok(true);
    }

    if (l1 - l0).abs() > 3 || (l1 - l0).abs() == 0 {
        return Ok(false);
    }

    for i in 2..report.len() {
        let &prev = report.get(i - 1).ok_or("?")?;
        let &curr = report.get(i).ok_or("?")?;
        if (curr - prev).abs() > 3
            || (curr - prev).abs() == 0
            || ((curr - prev) > 0) != ((l1 - l0) > 0)
        {
            return check_report_skip_used(&report[(i + 1)..], prev, Some((l1 - l0) > 0));
        }
    }

    Ok(true)
}

fn check_report_skip_used(
    report: &[i32],
    prev: i32,
    is_incr: Option<bool>,
) -> Result<bool, Box<dyn std::error::Error>> {
    if report.is_empty() {
        return Ok(true);
    }

    let is_increasing = match is_incr {
        Some(b) => b,
        None => (report.first().ok_or("?")? - prev) > 0,
    };

    for i in 0..report.len() {
        let last = match i {
            0 => prev,
            _ => *report.get(i - 1).ok_or("?")?,
        };
        let &curr = report.get(i).ok_or("?")?;
        if (curr - last).abs() > 3
            || (curr - last).abs() == 0
            || ((curr - last) > 0) != is_increasing
        {
            return Ok(false);
        }
    }
    Ok(true)
}

// fn check_report_skip_unused(
//     mut report: impl Iterator<Item = i32>,
// ) -> Result<bool, Box<dyn std::error::Error>> {
//     let mut prev: i32;
//     match report.next() {
//         Some(l) => prev = l,
//         None => return Err("empty report".into()),
//     }
//
//     let is_increasing: bool;
//
//     match report.next() {
//         Some(l) => {
//             if (l - prev).abs() > 3 || (l - prev).abs() == 0 {
//                 let (t1, t2) = report.tee();
//                 return Ok(
//                     check_report_skip_used(t1, prev, None) || check_report_skip_used(t2, l, None)
//                 );
//             }
//             is_increasing = (l - prev) > 0;
//             prev = l
//         }
//         None => return Ok(true),
//     }
//
//     while let Some(l) = report.next() {
//         if (l - prev).abs() > 3 || (l - prev).abs() == 0 || ((l - prev) > 0) != is_increasing {
//             return Ok(check_report_skip_used(
//                 // TODO: I am not happy with the way i copy iterator
//                 report.tee().0,
//                 prev,
//                 Some(is_increasing),
//             ));
//         }
//         prev = l
//     }
//     Ok(true)
// }
//
// fn check_report_skip_used(
//     mut report: impl Iterator<Item = i32>,
//     prev: i32,
//     is_incr: Option<bool>,
// ) -> bool {
//     let mut curr = prev;
//     let is_increasing: bool;
//
//     match report.next() {
//         Some(l) => {
//             if (l - curr).abs() > 3 || (l - curr).abs() == 0 {
//                 return false;
//             }
//             match is_incr {
//                 Some(b) => {
//                     if ((l - curr) > 0) != b {
//                         return false;
//                     } else {
//                         is_increasing = b;
//                     }
//                 }
//                 None => is_increasing = (l - curr) > 0,
//             }
//             curr = l
//         }
//         None => return true,
//     }
//
//     for l in report {
//         if (l - curr).abs() > 3 || (l - curr).abs() == 0 || ((l - curr) > 0) != is_increasing {
//             return false;
//         }
//         curr = l
//     }
//     true
// }
