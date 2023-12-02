use std::{fs::File, io::{self, BufRead, BufReader}};

fn main() -> io::Result<()> {
    let file = File::open("/home/henri/dev/AdventOfCode/2023/AdventOfCode_2023/day_1/data.txt")?;
    let reader = BufReader::new(file);
    let mut numbers: Vec<u8>  = Vec::new();

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
                    numbers.push(n.parse::<u8>().expect("Expected a number"));
                    continue;
                }

                n.push_str(&nums.first().unwrap().to_string());
                n.push_str(&nums.last().unwrap().to_string());
                numbers.push(n.parse::<u8>().expect("Expected a number"));
            },
            Err(_error) => break,
        }
    }

    let mut answer: usize = 0;
    for num in numbers {
        answer += num as usize;
    }
    
    println!("==================================");
    println!("==================================");
    println!("Answer:");
    println!("{}", answer);

    Ok(())
}
