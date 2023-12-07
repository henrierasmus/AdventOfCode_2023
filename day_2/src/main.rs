use std::{collections::HashMap, io::{self, BufReader, BufRead}};
use std::fs::File;

struct Game {
    id: u16,
    game_results: Vec<HashMap<String, usize>>
}

fn main() -> io::Result<()> {
    let cubes_set = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let file = File::open("/home/henri/dev/AdventOfCode/2023/AdventOfCode_2023/day_2/data.txt")?;
    let reader = BufReader::new(file);
    let mut games: Vec<Game> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(value) => {
                let parts: Vec<&str> = value.split(":").collect();
                let game_name_id: Vec<&str>= parts[0].split(" ").collect();
                let game_id = game_name_id[1].parse::<u16>().expect("Not valid input to parse");
                let game_results = parts[1];
                let result_instance: Vec<&str> = game_results.split(";").collect();
                let mut result_vec: Vec<HashMap<String, usize>> = Vec::new();

                for result in &result_instance {
                    let mut result_map: HashMap<String, usize> = HashMap::new(); 
                    let result_parts: Vec<&str> = result.split(",").collect();
                    for block in &result_parts {
                        let key_value_pair: Vec<&str> = block.trim().split(" ").collect();
                        let key = &key_value_pair[1];
                        if let Ok(value) = key_value_pair[0].parse::<usize>() {
                            result_map.insert(key.to_string(), value);
                        }
                    }
                    result_vec.push(result_map);
                }

                let game = Game {
                    id: game_id,
                    game_results: result_vec
                };

                games.push(game);
            },
            Err(e) => {
                println!("{}", e);
                break;
            }
        }        
    }

    let mut filtered_result = Vec::new();

    for game in &games {
        let mut valid_game = true;
        for result in &game.game_results {
            if let (Some(value1), Some(value2)) = (cubes_set.get("red"), result.get("red")) {
                if value1 < value2 {
                    valid_game = false;
                }
            }

            if let (Some(value1), Some(value2)) = (cubes_set.get("blue"), result.get("blue")) {
                if value1 < value2 {
                    valid_game = false;
                }
            }

            if let (Some(value1), Some(value2)) = (cubes_set.get("green"), result.get("green")) {
                if value1 < value2 {
                    valid_game = false;
                }
            }

        }   
        if valid_game {
            filtered_result.push(game);
        }
    }

    let mut total_score = 0;
    for game in filtered_result {
        println!("{}", game.id);
        total_score += game.id;
    }

    println!("{}", total_score);

    let mut multiplied_values: Vec<usize> = Vec::new();
    
    for game in &games {
        let mut highest_red: usize = 0;
        let mut highest_blue: usize = 0;
        let mut highest_green: usize = 0;

        for result in &game.game_results {
            if let Some(&value) = result.get("red") {
                if highest_red < value {
                    highest_red = value;
                }
            }    
            
            if let Some(&value) = result.get("blue") {
                if highest_blue < value {
                    highest_blue = value;
                }
            }    
            
            if let Some(&value) = result.get("green") {
                if highest_green < value {
                    highest_green = value;
                }
            }    
        }

        let answer: usize = highest_green * highest_blue * highest_red;
        multiplied_values.push(answer);
    }

    let mut answer = 0;

    for number in multiplied_values {
        answer += number;
    }

    println!("{}", answer);

    Ok(())
} 
