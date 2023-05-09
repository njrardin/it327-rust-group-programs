// Program - Intermediate Program
// IT 327 - Group 4
// Names: Quinn Pulley, Cameron Crone, Nate Rardin, Garett Sheley

// These are functions from Rust's standard library that allow for input/output and file reading
use std::env;
use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

// Takes a command line argument that contains student grades out of 100 and prints analytics to the screen
// Main function
fn main() -> Result<(), Error> {
    // Takes a command line argument that is the name of a file containing student grades out of 100
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut grades: Vec<i32> = read(file_path)?;

    // Sort the grades
    //insertion_sort(&mut grades);
    grades.sort();

    // Gather statistics on the grades
    let minimum = get_minimum(&grades);
    let maximum = get_maximum(&grades);
    let mean = compute_mean(&grades);
    let median = find_median(&grades);
    let distribution = compute_grade_distribution(&grades);

    // Print the statistics computed from the student grades
    println!("Analysis of Student Grades:");
    println!("The highest grade in the class is: {}", maximum);
    println!("The lowest grade in the class is: {}", minimum);
    println!("The median grade in the class is: {}", median);
    println!("The class average grade is: {}", mean);
    println!("Here is the distribution of grades in the class:");
    println!("A (90-100%) - {}", distribution[0]);
    println!("B (80-89%)  - {}", distribution[1]);
    println!("C (70-79%)  - {}", distribution[2]);
    println!("D (60-69%)  - {}", distribution[3]);
    println!("F (0-59%)   - {}", distribution[4]);

    Ok(())
}


// Returns the minimum value in v which is presorted
fn get_minimum(v: &Vec<i32>) -> i32 {
    v[0]
}

// Returns the maximum value in v which is presorted
fn get_maximum(v: &Vec<i32>) -> i32 {
    v[v.len() - 1]
}

// Finds and returns the median value in v
fn find_median(v: &Vec<i32>) -> i32 {
    let mut median = v[(v.len() - 1) / 2];
    if v.len() % 2 != 0 {
        median = (median + v[(v.len() - 2) / 2]) / 2;
    }
    median
}

// Computes and returns the means value of v
fn compute_mean(v: &Vec<i32>) -> f32 {
    let mut mean: f32 = 0.0;
    for x in v {
        mean += *x as f32;
    }
    mean / (v.len() as f32)
}

// Computes and returns the distribution of letter grades in v
fn compute_grade_distribution(v: &Vec<i32>) -> Vec<i32> {
    let mut distribution: Vec<i32> = vec![0, 0, 0, 0, 0];// Holds the count of each letter grade
    for &grade in v {
        let letter_grade = get_letter_grade(grade);// Determine the letter grade of the numeric score
        match letter_grade {
            'A' => distribution[0] += 1,
            'B' => distribution[1] += 1,
            'C' => distribution[2] += 1,
            'D' => distribution[3] += 1,
            'F' => distribution[4] += 1,
            _ => (),
        }
    }
    distribution
}


// Finds the letter grade from a numeric score using pattern matching
fn get_letter_grade(grade: i32) -> char {
    match grade {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F', // Anything else
    }
}

// Takes path in the form of string slice and returns enum which contains vector of integers on success or IO error type
fn read(path: &str) -> Result<Vec<i32>, Error> {
    let file = File::open(path)?; // open file by given path
    // wrap file into generic buffered reader
    let br = BufReader::new(file);
    let mut v = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    for line in br.lines() {
        // We check if the IO operation got an error, returning this error as the function result if so
        let line = line?;
        let n = line
            .trim() // trim "whitespaces"
            .parse() // parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error, we convert it to std::io::Error type and return it as function result
        v.push(n);
    }
    Ok(v) // everything is Ok, return vector
}
