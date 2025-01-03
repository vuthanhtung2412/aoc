use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// UP,RIGHT,DOWN,LEFT
// y.x
const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 6;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year,
        day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let m: Result<Vec<Vec<char>>, io::Error> = reader
        .lines()
        // Result.map() will propagate the error if Result yeild an error and do correct mappings
        // other wise
        .map(|line| {
            line.map(|l| l.chars().collect()) // Convert each line to Vec<char>
        })
        .collect();

    let mut map = m?;

    let mut res = 1;
    let mut curr_y = 0;
    let mut curr_x = 0;
    let mut dir = 0;
    let h = map.len() as i32;
    let w = map.first().ok_or("Wtf empty file")?.len() as i32;
    'a: for (i, l) in map.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if ['v', '<', '>', '^'].contains(c) {
                dir = match c {
                    'v' => 2,
                    '<' => 3,
                    '>' => 1,
                    '^' => 0,
                    _ => return Err("if wtf ?".into()),
                };
                map[i][j] = 'X';
                curr_y = i as i32;
                curr_x = j as i32;
                break 'a;
            }
        }
    }

    loop {
        let d = DIRS.get(dir).ok_or("what do you mean ?")?;
        if curr_y + d.0 >= h || curr_y + d.0 < 0 || curr_x + d.1 < 0 || curr_x + d.1 >= w {
            break;
        }
        match map
            .get((curr_y + d.0) as usize)
            .ok_or("wtf get y")?
            .get((curr_x + d.1) as usize)
            .ok_or("wtf get x")?
        {
            '#' => {
                dir = (dir + 1) % 4;
            }
            '.' => {
                res += 1;
                curr_x += d.1;
                curr_y += d.0;
                map[curr_y as usize][curr_x as usize] = 'X'
            }
            'X' => {
                curr_x += d.1;
                curr_y += d.0;
            }
            _ => return Err("unidentified char".into()),
        }
    }

    println!("{}", res);

    Ok(())
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // let current_file_path = Path::new(file!());
    // let year = 2024;
    // let day = 6;
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
