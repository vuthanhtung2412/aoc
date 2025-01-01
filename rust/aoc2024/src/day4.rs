use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
// (x,y)
const DIRS: [(i32, i32); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 4;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let mut res = 0;

    // Iterate over each line in the file and convert it into a Vec<char>
    let m: Result<Vec<Vec<char>>, io::Error> = reader
        .lines()
        .map(|line| {
            line.map(|l| l.chars().collect()) // Convert each line to Vec<char>
        })
        .collect();

    let map = m?;

    // map.iter() only borrow the vector while map.into_iter() move the vector and transfer
    // ownership
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if c == &'X' {
                res += check_xmas(&map, i, j)?
            }
        }
    }

    println!("{}", res);

    Ok(())
}

// usize is an unsigned integer type whose size depends on the architecture of the target system

fn check_xmas(map: &[Vec<char>], r: usize, c: usize) -> Result<i32, Box<dyn std::error::Error>> {
    let mut res = 0;

    for d in DIRS {
        // Calculate the last row and column positions before the loop to avoid repetitive code
        let last_r = (r as i32) + (XMAS.len() as i32 - 1) * d.1;
        let last_c = (c as i32) + (XMAS.len() as i32 - 1) * d.0;

        // Handle conversion and bounds checking in a single step
        if let (Ok(last_r_usize), Ok(last_c_usize)) = (
            // Safe Conversions: Prefer `try_from` because it explicitly handles the conversion failure.
            // `as` Keyword: Avoid using as directly for conversions when there’s a possibility of negative values,
            // `as` silently casts and can lead to unexpected behavior.
            usize::try_from(last_r),
            usize::try_from(last_c),
        ) {
            if last_r_usize < map.len() && last_c_usize < map.first().unwrap_or(&vec![]).len() {
                let mut is_valid = true;

                // Check each position in the XMAS sequence
                for i in 1..XMAS.len() {
                    let row_index = (r as i32 + (d.1 * i as i32)) as usize;
                    let col_index = (c as i32 + (d.0 * i as i32)) as usize;

                    // Check bounds before accessing map
                    if let Some(row) = map.get(row_index) {
                        if let Some(&cell) = row.get(col_index) {
                            if cell != XMAS[i] {
                                is_valid = false;
                                break;
                            }
                        } else {
                            is_valid = false;
                            break;
                        }
                    } else {
                        is_valid = false;
                        break;
                    }
                }

                // If the sequence is valid, increment the result
                if is_valid {
                    res += 1;
                }
            }
        }
    }

    Ok(res)
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 4;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example2.txt",
        "../../../questions/{}/{}/input2.txt",
        year,
        day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here

    // Iterate over each line in the file and convert it into a Vec<char>
    let m: Result<Vec<Vec<char>>, io::Error> = reader
        .lines()
        .map(|line| {
            line.map(|l| l.chars().collect()) // Convert each line to Vec<char>
        })
        .collect();

    let map = m?;

    let mut res = 0;
    for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == 'A' && check_x_mas(&map, i, j) {
                res += 1;
            }
        }
    }
    println!("{}", res);
    Ok(())
}

const DIR_P2: [(i32, i32); 4] = [
    // (r,c)
    (-1, -1),
    (-1, 1),
    (1, 1),
    (1, -1),
];
fn check_x_mas(map: &[Vec<char>], r: usize, c: usize) -> bool {
    if r > 0 && c > 0 && r < map.len() - 1 && c < map.first().unwrap().len() - 1 {
        let corners = DIR_P2.map(|d| {
            map.get((r as i32 + d.0) as usize)
                .unwrap()
                .get((c as i32 + d.1) as usize)
                .unwrap()
        });
        if !(corners.iter().filter(|&&&x| x == 'M').count() == 2
            && corners.iter().filter(|&&&x| x == 'S').count() == 2)
        {
            return false;
        }

        for i in 1..DIR_P2.len() {
            if corners.get(i).unwrap() == corners.get(i - 1).unwrap() {
                return true;
            }
        }
    }
    false
}
