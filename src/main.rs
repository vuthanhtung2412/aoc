use chrono::{Datelike, Local};
use html2md::parse_html;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cookie = env::var("AOC_COOKIE")?;

    let args: Vec<String> = env::args().collect();

    let year = match args.get(1) {
        Some(s) => match s.parse::<i32>() {
            Ok(y) => y,
            Err(_) => return Err("Invalid year".into()),
        },
        None => Local::now().year(),
    };

    let day = match args.get(2) {
        Some(s) => match s.parse::<u32>() {
            Ok(d) => {
                if d > 25 {
                    return Err("Invalid day".into());
                }
                d
            }
            Err(_) => return Err("Invalid day".into()),
        },
        None => Local::now().day(),
    };

    let client = Client::new();

    // Download the question
    let question_url = format!("https://adventofcode.com/{}/day/{}", year, day);
    let response = client
        .get(&question_url)
        .header("Cookie", format!("session={}", cookie))
        .send()?;

    #[allow(unused_assignments)]
    let mut is_part2 = false;

    if response.status().is_success() {
        let document = Html::parse_document(&response.text()?);
        let question_selector = Selector::parse("article.day-desc").unwrap();
        let mut element_iterator = document.select(&question_selector);

        match element_iterator.next() {
            None => return Err("Question not found".into()),
            Some(element_ref) => {
                write(
                    &format!("questions/{}/{}", year, day),
                    "question1.md",
                    &parse_html(&element_ref.html()),
                )?;
            }
        };

        match element_iterator.next() {
            None => println!("Only first part available !"),
            Some(element_ref) => {
                is_part2 = true;
                write(
                    &format!("questions/{}/{}", year, day),
                    "question2.md",
                    &parse_html(&element_ref.html()),
                )?;
            }
        };
    } else {
        return Err(format!("Failed to download question: {}", response.status()).into());
    }

    // Download the input
    let input_url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let input_response = client
        .get(&input_url)
        .header("Cookie", format!("session={}", cookie))
        .send()?;

    if input_response.status().is_success() {
        let input_content = input_response.text()?;
        write(
            &format!("questions/{}/{}", year, day),
            &format!("input{}.txt", if is_part2 { 2 } else { 1 }),
            &input_content,
        )?;
    } else {
        return Err(format!("Failed to download input: {}", input_response.status()).into());
    }

    Ok(())
}

#[allow(dead_code)]
fn write(path: &str, file_name: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dir_path = Path::new(path);
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    let file_path = dir_path.join(file_name);
    let mut file = fs::File::create(file_path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
