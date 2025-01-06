use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIRS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

pub fn part1() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 10;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example1.txt",
        "../../../questions/{}/{}/input1.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<u8> = line
            .chars()
            .map(|c| { c.to_digit(10).unwrap() } as u8)
            .collect();
        map.push(row); // Add the row to the result
    }

    let mut trailheads = Vec::new();
    for (i, l) in map.iter().enumerate() {
        for (j, num) in l.iter().enumerate() {
            if *num == 0 {
                trailheads.push((j, i));
            }
        }
    }

    let height = map.len();
    let width = map.first().ok_or("wtf width = 0 ???")?.len();
    let mut res = 0;
    for t in trailheads { 
        res += get_score(&map, t, height, width)
    }
    println!("{}", res);

    Ok(())
}

fn get_score(map: &[Vec<u8>], pos: (usize, usize), height: usize, width: usize) -> usize {
    let mut curr_level: VecDeque<(usize, usize)> = VecDeque::from([pos]);
    for i in 0..9 {
        let len = curr_level.len();
        let mut visited = HashSet::new();
        for _ in 0..len {
            let curr_pos = curr_level.pop_front().unwrap();
            for d in DIRS {
                let neighbor_x = curr_pos.0 as i32 + d.0;
                let neighbor_y = curr_pos.1 as i32 + d.1;
                if neighbor_x >= 0
                    && neighbor_x < width as i32
                    && neighbor_y >= 0
                    && neighbor_y < height as i32
                    && map[neighbor_y as usize][neighbor_x as usize] == i + 1
                    && !visited.contains(&(neighbor_x as usize, neighbor_y as usize))
                {
                    curr_level.push_back((neighbor_x as usize, neighbor_y as usize));
                    visited.insert((neighbor_x as usize, neighbor_y as usize));
                }
            }
        }
    }
    curr_level.len()
}

pub fn part2() -> Result<(), Box<dyn std::error::Error>> {
    let current_file_path = Path::new(file!());
    let year = 2024;
    let day = 10;
    let path = current_file_path.parent().unwrap().join(format!(
        // "../../../questions/{}/{}/example2.txt",
        "../../../questions/{}/{}/input2.txt",
        year, day
    ));

    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    // Your solution goes here
    let mut map: Vec<Vec<u8>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let row: Vec<u8> = line
            .chars()
            .map(|c| { c.to_digit(10).unwrap() } as u8)
            .collect();
        map.push(row); // Add the row to the result
    }

    let mut trailheads = Vec::new();
    for (i, l) in map.iter().enumerate() {
        for (j, num) in l.iter().enumerate() {
            if *num == 0 {
                trailheads.push((j, i));
            }
        }
    }

    let height = map.len();
    let width = map.first().ok_or("wtf width = 0 ???")?.len();
    let mut res = 0;
    for t in trailheads { 
        res += get_score_2(&map, t, height, width)
    }
    println!("{}", res);

    Ok(())
}

fn get_score_2(map: &[Vec<u8>], pos: (usize, usize), height: usize, width: usize) -> usize {
    let mut curr_level: VecDeque<(usize, usize)> = VecDeque::from([pos]);
    for i in 0..9 {
        let len = curr_level.len();
        for _ in 0..len {
            let curr_pos = curr_level.pop_front().unwrap();
            for d in DIRS {
                let neighbor_x = curr_pos.0 as i32 + d.0;
                let neighbor_y = curr_pos.1 as i32 + d.1;
                if neighbor_x >= 0
                    && neighbor_x < width as i32
                    && neighbor_y >= 0
                    && neighbor_y < height as i32
                    && map[neighbor_y as usize][neighbor_x as usize] == i + 1
                {
                    curr_level.push_back((neighbor_x as usize, neighbor_y as usize));
                }
            }
        }
    }
    curr_level.len()
}
