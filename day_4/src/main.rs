use std::{fs::File, io::{ BufReader, self, BufRead }, usize};

fn main() -> io::Result<()> {
    let file = File::open("C:/Users/HenriErasmus/Runninghill/AdventOfCode/AdventOfCode_2023/day_4/data.txt")?;
    let reader = BufReader::new(file);
    let mut total_score = 0;

    for line in reader.lines() {
        match line {
            Ok(value) => {
                let mut correlating_nums: Vec<&str> = Vec::new();
                let card: Vec<&str> = value.split(":").collect(); 
                let all_numbers: Vec<&str> = card[1].split("|").collect();
                let winning_numbers: Vec<&str> = all_numbers[0].split_whitespace().collect();
                let my_nums: Vec<&str> = all_numbers[1].split_whitespace().collect();
                let mut line_score: usize = 0;

                for number in &winning_numbers {
                    for my_number in &my_nums {
                        if my_number == number && !correlating_nums.contains(number) {
                            correlating_nums.push(number);
                            if line_score == 0 {
                                line_score = 1;
                            }  else {
                                line_score = line_score * 2;
                            }
                        }
                    }
                }
                total_score += line_score;
//                println!("Card score: {}", line_score);
            },
            Err(error) => println!("{}", error)
        }
    }

    println!("Final score: {}", total_score);
    
    Ok(())
}
