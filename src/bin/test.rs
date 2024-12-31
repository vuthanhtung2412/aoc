use chrono::{Datelike, Local};
use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    clone_iter();
    Ok(())
}

#[allow(dead_code)]
fn clone_iter() {
    use itertools::Itertools;
    let xs = vec![0, 1, 2, 3];

    let (mut t1, mut t2) = xs.into_iter().tee();
    println!("{}",t1.next().unwrap());
    println!("{}",t2.next().unwrap());
} 

#[allow(dead_code)]
fn get_date() -> Result<(), Box<dyn Error>> {
    let year = Local::now().year();
    let day = Local::now().day();
    println!("day : {}, year : {}", day, year);
    Ok(())
}

#[allow(dead_code)]
fn get_args() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <arg1> <arg2> <arg3>", args[0]);
        return Err("Incorrect use of arguments".into());
    }

    let arg1 = &args[1];
    let arg2 = &args[2];
    let arg3 = &args[3];

    println!("Argument 1: {}", arg1);
    println!("Argument 2: {}", arg2);
    println!("Argument 3: {}", arg3);

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

#[allow(dead_code)]
fn get_env_var() -> Result<(), Box<dyn std::error::Error>> {
    let value = env::var("AOC_COOKIE")?;

    // Print the value
    println!("The value of MY_ENV_VAR is: {}", value);

    Ok(())
}
#[allow(dead_code)]
fn curr_dir() -> Result<(), Box<dyn std::error::Error>> {
    // Get the current working directory
    let current_dir: PathBuf = env::current_dir()?;
    println!("Current working directory: {:?}", current_dir);
    Ok(())
}

#[allow(dead_code)]
fn bin_dir() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe: PathBuf = env::current_exe()?;
    println!("Current executable path: {:?}", current_exe);
    Ok(())
}
