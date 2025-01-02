use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 5;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let mut afters: HashMap<i32, HashSet<i32>> = HashMap::new();

    let mut lines = reader.lines();
    loop {
        let l = lines.next();
        match l {
            Some(r) => {
                let line = r?;
                if line.trim().is_empty() {
                    break;
                }
                let edge: Vec<&str> = line.split('|').collect();
                let n1 = edge.first().ok_or("WTF no int ?")?.parse::<i32>()?;
                let n2 = edge.get(1).ok_or("WTF no second int ?")?.parse::<i32>()?;
                afters.get_mut(&n1).ok_or("wtf")?.insert(n2);
                let after_of_n1 = afters.entry(n1).or_default();
                after_of_n1.insert(n2);
            }
            None => break,
        }
    }

    let mut res = 0;

    for u in lines {
        let update = u?;
        let mut prev = HashSet::new();
        let mut is_correct = true;
        let pages = update
            .split(',')
            .filter_map(|p| p.parse::<i32>().ok())
            .collect::<Vec<i32>>();
        for &page in &pages {
            if let Some(a) = afters.get(&page) {
                if !a.is_disjoint(&prev) {
                    is_correct = false;
                    break;
                }
            }
            prev.insert(page);
        }

        if is_correct {
            res += pages
                .get(pages.len() / 2)
                .ok_or("what do you mean middle element doesn't exist ?")?;
        }
    }

    println!("{}", res);

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // let current_file_path = Path::new(file!());
    // let year = 2024;
    // let day = 5;
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
