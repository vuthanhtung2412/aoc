use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current file's path
    let current_file_path = Path::new(file!());
    let path = current_file_path
        .parent()
        .unwrap()
        .join("../../../questions/2024/1/input1.txt");

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut hl: BinaryHeap<i32> = BinaryHeap::new();
    let mut hr: BinaryHeap<i32> = BinaryHeap::new();
    let mut res = 0;

    for line in reader.lines() {
        let line = line?;
        for (i, w) in line.split_whitespace().enumerate() {
            match i {
                0 => hl.push(w.parse::<i32>()?),
                1 => hr.push(w.parse::<i32>()?),
                _ => break,
            }
        }
    }

    while let (Some(l), Some(r)) = (hl.pop(), hr.pop()) {
        res += (r - l).abs();
    }

    println!("{}", res);

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let path = current_file_path
        .parent()
        .unwrap()
        .join("../../../questions/2024/1/input2.txt");
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut m: HashMap<u32, (u32, u32)> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for (i, w) in line.split_whitespace().enumerate() {
            match i {
                0 => {
                    let num = w.parse::<u32>()?;
                    match m.get(&num) {
                        Some((l, r)) => {
                            m.insert(num, (*l + 1, *r));
                        }
                        None => {
                            m.insert(num, (1, 0));
                        }
                    }
                }
                1 => {
                    let num = w.parse::<u32>()?;
                    match m.get(&num) {
                        Some((l, r)) => {
                            m.insert(num, (*l, *r + 1));
                        }
                        None => {
                            m.insert(num, (0, 1));
                        }
                    }
                }
                _ => break,
            }
        }
    }

    let mut res: u32 = 0;
    for (k,v) in m.iter(){
        res += k*v.0*v.1;
    }
    println!("{}", res);
    Ok(())
}
