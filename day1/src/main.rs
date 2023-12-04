/*
--- Day 1: Trebuchet?! ---

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover.
On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;

fn get_input(path: &Path) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|line| line.unwrap()).collect())
}
fn main() -> Result<()> {
    let digit_maps = HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("0", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
        ("zero", "0"),
    ]);

    let inputs = get_input(Path::new("./day1/input.txt"))?;

    let mut sum: u64 = 0;
    let mut i = 1;
    for input in inputs {
        let mut digits: Vec<(usize, &str)> = vec![];
        for digit_map in &digit_maps {
            let matches: Vec<(usize, &str)> = input.match_indices(digit_map.0).collect();
            digits.extend(matches);
        }
        digits.sort();

        let first: String = digit_maps
            .get(digits.first().unwrap().1)
            .unwrap()
            .to_string();
        let last: String = digit_maps
            .get(digits.last().unwrap().1)
            .unwrap()
            .to_string();
        let number = format!("{first}{last}").parse::<u64>().unwrap();
        println!("{i}: ({first}, {last}) = {number}");
        i = i + 1;
        sum = sum + number;
    }
    dbg!(sum);

    Ok(())
}
