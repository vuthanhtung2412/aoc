use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 8;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    // Your solution goes here

    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // I am so tempted to do this
    // let mut height: usize = lines.count();
    // let mut width: usize = lines.peekable().next()...len();
    // However there is one preblem every time we do .count() it consumes the iterator -> we have
    // to make the syscall to read the file again

    let mut height: usize = 0;
    let mut width: usize = 0;

    for (i, l) in lines.enumerate() {
        let line = l?;
        for (j, c) in line.chars().enumerate() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    let pos = map.entry(c).or_default();
                    pos.push((j, i));
                }
                _ => {}
            }
            if i == 0 {
                width = j
            }
        }
        height = i
    }

    println!("{}", get_antinodes(&map, height + 1, width + 1));

    Ok(())
}

fn get_antinodes(map: &HashMap<char, Vec<(usize, usize)>>, h: usize, w: usize) -> usize {
    let mut res: HashSet<(usize, usize)> = HashSet::new();
    for (_, positions) in map.iter() {
        for i in 1..positions.len() {
            for j in 0..i {
                let (x1, y1) = positions.get(i).unwrap();
                let (x1, y1) = (*x1 as i32, *y1 as i32);
                let (x2, y2) = positions.get(j).unwrap();
                let (x2, y2) = (*x2 as i32, *y2 as i32);

                let x3 = 2 * x2 - x1;
                let y3 = 2 * y2 - y1;
                if x3 >= 0 && x3 < w as i32 && 2 * y3 >= 0 && y3 < h as i32 {
                    res.insert((x3 as usize, y3 as usize));
                }

                let x4 = 2 * x1 - x2;
                let y4 = 2 * y1 - y2;
                if x4 >= 0 && x4 < w as i32 && y4 >= 0 && y4 < h as i32 {
                    res.insert((x4 as usize, y4 as usize));
                }
            }
        }
    }
    res.len()
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 8;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example2.txt",
        "../../../questions/{}/{}/input2.txt",
        year,
        day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);
    let lines = reader.lines();

    // Your solution goes here

    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    // I am so tempted to do this
    // let mut height: usize = lines.count();
    // let mut width: usize = lines.peekable().next()...len();
    // However there is one preblem every time we do .count() it consumes the iterator -> we have
    // to make the syscall to read the file again

    let mut height: usize = 0;
    let mut width: usize = 0;

    for (i, l) in lines.enumerate() {
        let line = l?;
        for (j, c) in line.chars().enumerate() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' => {
                    let pos = map.entry(c).or_default();
                    pos.push((j, i));
                }
                _ => {}
            }
            if i == 0 {
                width = j
            }
        }
        height = i
    }

    println!("{}", get_antinodes2(&map, height + 1, width + 1));

    Ok(())
}

fn get_antinodes2(map: &HashMap<char, Vec<(usize, usize)>>, h: usize, w: usize) -> usize {
    let mut res: HashSet<(usize, usize)> = HashSet::new();
    for (_, positions) in map.iter() {
        for i in 1..positions.len() {
            for j in 0..i {
                let (x1, y1) = positions.get(i).unwrap();
                res.insert((*x1, *y1));
                let (x1, y1) = (*x1 as i32, *y1 as i32);

                let (x2, y2) = positions.get(j).unwrap();
                res.insert((*x2, *y2));
                let (x2, y2) = (*x2 as i32, *y2 as i32);

                let mut curr_x = 2 * x2 - x1;
                let mut curr_y = 2 * y2 - y1;
                let mut prev_x = x2;
                let mut prev_y = y2;
                while curr_x >= 0 && curr_x < w as i32 && 2 * curr_y >= 0 && curr_y < h as i32 {
                    res.insert((curr_x as usize, curr_y as usize));
                    (curr_x, prev_x) = (2 * curr_x - prev_x, curr_x);
                    (curr_y, prev_y) = (2 * curr_y - prev_y, curr_y);
                }

                curr_x = 2 * x1 - x2;
                curr_y = 2 * y1 - y2;
                prev_x = x1;
                prev_y = y1;
                while curr_x >= 0 && curr_x < w as i32 && 2 * curr_y >= 0 && curr_y < h as i32 {
                    res.insert((curr_x as usize, curr_y as usize));
                    (curr_x, prev_x) = (2 * curr_x - prev_x, curr_x);
                    (curr_y, prev_y) = (2 * curr_y - prev_y, curr_y);
                }
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            if res.contains(&(j, i)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
    res.len()
}
