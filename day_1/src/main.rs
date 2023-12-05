use std::{fs::File, io::{self, BufRead, BufReader}};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file = File::open("/home/henri/dev/AdventOfCode/2023/AdventOfCode_2023/day_1/data.txt")?;
    let reader = BufReader::new(file);
    let mut numbers_challenge_1: Vec<u8>  = Vec::new();

    for line in reader.lines() {
        let mut nums: Vec<u8> = Vec::new();
        match line {
            Ok(value) => {
                let chars: Vec<char> = value.chars().collect();
                for character in chars {
                    if character.is_digit(10) {
                        nums.push(character as u8 - b'0');
                    }
                }

                let mut n = String::new();
                if nums.len() == 1 {
                    n.push_str(&nums[0].to_string());
                    n.push_str(&nums[0].to_string()); 
                    numbers_challenge_1.push(n.parse::<u8>().expect("Expected a number"));
                    continue;
                }

                n.push_str(&nums.first().unwrap().to_string());
                n.push_str(&nums.last().unwrap().to_string());
                numbers_challenge_1.push(n.parse::<u8>().expect("Expected a number"));
            },
            Err(_error) => break,
        }
    }
   
    let mut answer: usize = 0;
    for num in numbers_challenge_1 {
        answer += num as usize;
    }

    println!("==================================");
    println!("Answer Part 1:");
    println!("{}", answer);
    
    // PART 2
    let mut numbers_challenge_2: Vec<usize> = Vec::new();
    
    let substrings = HashMap::from([
        (1, "one"),
        (2, "two"), 
        (3, "three"), 
        (4, "four"), 
        (5, "five"), 
        (6, "six"), 
        (7, "seven"), 
        (8, "eight"), 
        (9, "nine")
    ]);

    let numbers_in_string = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let file = File::open("/home/henri/dev/AdventOfCode/2023/AdventOfCode_2023/day_1/data.txt")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_result = line.expect("error reading line");
        let mut first_num_index: usize = line_result.len() - 1;
        let mut last_num_index: usize = 0;
        let mut first_num = String::new();
        let mut last_num = String::new();

        for (key, value) in &substrings {
            let substring_indexes = find_all(&line_result, value);
            if substring_indexes.len() > 0 {
                if substring_indexes[0] < first_num_index {
                    first_num_index = substring_indexes[0];
                    first_num = key.to_string();
                } 
               
                if substring_indexes.last().unwrap() > &last_num_index { 
                    last_num_index = *substring_indexes.last().unwrap();
                    last_num = key.to_string();
                }
            }
        }
        
        for &number in &numbers_in_string {
            let substring_indexes = find_all(&line_result, number);
            if substring_indexes.len() > 0 {
                if substring_indexes[0] < first_num_index {
                    first_num_index = substring_indexes[0];
                    first_num = number.to_string();
                } 

                if substring_indexes.last().unwrap() > &last_num_index { 
                    last_num_index = *substring_indexes.last().unwrap();
                    last_num = number.to_string();
                }
            }
       }

        if first_num == "" {
            first_num = last_num.clone();
        }

        if last_num == "" {
            last_num = first_num.clone();
        }
        
        let mut num_to_add = String::new();
        num_to_add.push_str(&first_num);
        num_to_add.push_str(&last_num);

        numbers_challenge_2.push(num_to_add.parse::<usize>().expect("a number must be provided"));
    }

    let mut answer_2: usize = 0;  
    for num in numbers_challenge_2 {
        answer_2 += num;
    }

    println!("==================================");
    println!("Answer Part 2:");
    println!("{}", answer_2);
    Ok(())
}

fn find_all(haystack: &str, needle: &str) -> Vec<usize> {
    let mut start = 0;
    let mut indexes = Vec::new();

    while let Some(index) = haystack[start..].find(needle){
        indexes.push(start + index);
        start += index + 1;
    }

    indexes
}
