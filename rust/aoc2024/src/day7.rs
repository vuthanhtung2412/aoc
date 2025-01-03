use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 7;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here

    // How to print out a number bits by bits
    // let value: usize = 2;
    // for i in 0..2 {
    //     println!("{}", value >> i & 1)
    // }

    let lines = reader.lines();
    let mut res: u64 = 0;

    for l in lines {
        let line = l?;
        let (t, a) = line
            .split_once(":")
            .ok_or("input mal mal-formated where is the colon")?;

        let arr: Vec<u64> = a
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        let target = t.parse::<u64>()?;
        if possible(target, &arr) {
            res += target;
        }
    }
    println!("{}", res);
    Ok(())
}

fn possible(target: u64, arr: &[u64]) -> bool {
    if arr.is_empty() {
        return target == 0;
    }
    if arr.len() == 1 {
        return target == *arr.first().unwrap();
    }

    let mut eval = *arr.first().unwrap();
    println!("target {}, arr {:?}", target, arr);
    for i in 0..usize::pow(2, (arr.len() - 1) as u32) {
        for j in 0..arr.len() - 1 {
            if (i >> j) & 1 == 1 {
                eval *= arr[j + 1]
            } else {
                eval += arr[j + 1];
            }
        }
        if eval == target {
            println!("true : {:b}", i);
            return true;
        }
        eval = *arr.first().unwrap();
    }
    false
}

// IDEA: if precedence rule
// use a stack to pop and push whenever i encounter a multiplcation
// at the end we can sum up what is left in the stack
// fn possible(target: i32, arr: &[i32]) -> bool {
//     if arr.is_empty() {
//         return target == 0;
//     }
//     if arr.len() == 1 {
//         return target == *arr.first().unwrap();
//     }
//
//     let mut stack: Vec<i32> = vec![0; arr.len()];
//     let mut len: usize = 1;
//     stack[0] = *arr.first().unwrap();
//
//     println!("target {}, arr {:?}", target, arr);
//     for i in 0..usize::pow(2, (arr.len() - 1) as u32) {
//         for j in 0..arr.len() - 1 {
//             if (i >> j) & 1 == 1 {
//                 // multiply
//                 stack[len - 1] *= arr[j + 1]
//             } else {
//                 stack[len] = arr[j + 1];
//                 len += 1
//             }
//         }
//         let mut sum = 0;
//         (0..len).for_each(|a| {
//             print!("{} ", stack[a]);
//             sum += stack[a]
//         });
//         println!();
//         if sum == target {
//             println!("true : {:b}", i);
//             return true;
//         }
//         len = 1;
//         stack[0] = *arr.first().unwrap();
//     }
//     false
// }

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // let current_file_path = Path::new(file!());
    // let year = 2024;
    // let day = 7;
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
