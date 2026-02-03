use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let mut part_one_result: u64 = 0;
    let mut part_two_result: u64 = 0;
    for line in reader.lines() {
        let line = line.expect("Error reading line");
        part_one_result += calculate_bank_max_joltage_part_one(&line) as u64;
        part_two_result += calculate_bank_max_joltage_part_two(&line);
    }

    println!("Part one max joltage: {}", part_one_result);
    println!("Part two max joltage: {}", part_two_result);
}

fn calculate_bank_max_joltage_part_one(line: &String) -> u32 {
    /*
     * Idea behind solution: 
     * Keep track of the two largest numbers as their combination will result in the largest possible two digit number.
     * Keep track of the numbers positions so we know how to arrange them.
     * 
     */
    let mut largest_number: u32 = 0;
    let mut largest_number_pos: u32 = 0;
    let mut second_number = 0;
    let mut largest_number_is_second_digit = false;
    while second_number == 0 {
        let mut current_pos: u32 = 0;
        for character in line.chars() {
            let number = character.to_digit(10).expect("error converting character to digit");
            if number > largest_number {
                largest_number = number;
                largest_number_pos = current_pos;
                // reset second number, new largest double digit found
                second_number = 0;
            }
            else if number > second_number {
                second_number = number;
            }
            // advance the current position
            current_pos += 1;

            // handles edge case where the largest number was the last
            if current_pos == largest_number_pos {
                largest_number_is_second_digit = true;
                break;
            }
        }
    }
    // Putting the number that appears first into the first digit spot
    if largest_number_is_second_digit {
        second_number * 10 + largest_number
    } else {
        largest_number * 10 + second_number
    }
}

fn calculate_bank_max_joltage_part_two(line: &String) -> u64 { 
    let batteries: Vec<u64>  = line.chars().map(|a| a.to_digit(10).expect("error converting character to digit") as u64).collect(); 
    let number_of_digits = 12;
    calculate_bank_max_joltage_part_two_helper(batteries, number_of_digits)
}

fn calculate_bank_max_joltage_part_two_helper(mut batteries: Vec<u64>, number_of_digits: usize) -> u64 {
    // Determine the maximum place to search for the largest digit
    let max_search_index = ( batteries.len() - number_of_digits ) as u32;
    let mut largest_number: u64 = 0;
    let mut largest_number_pos: u32 = 0;
    let mut current_pos: u32 = 0;
    for battery in &batteries { 
        if *battery > largest_number {
            largest_number = *battery;
            largest_number_pos = current_pos;
        }

        if current_pos == max_search_index {
            break;
        }

        current_pos += 1;
    }

    // handle last digit case
    if number_of_digits == 1 {
        return batteries[largest_number_pos as usize];
    }

    // shrink batteries to remaining pool
    let mut batteries = batteries.split_off(largest_number_pos as usize);

    let batteries_len: u32 = batteries.len() as u32;

    // check to see if we can stop recursing
    if batteries_len == number_of_digits as u32 {
        let mut result = 0;
        // used for determining exponent
        let mut current_count: u32 = 1;
        for battery in batteries {
            result += battery * 10_u64.pow(batteries_len - current_count);
            current_count += 1;
        }
        return result;
    }

    batteries.remove(0) * 10_u64.pow(number_of_digits as u32 - 1) + calculate_bank_max_joltage_part_two_helper(batteries, number_of_digits - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        assert_eq!(calculate_bank_max_joltage_part_two(&String::from("987654321111111")), 987654321111);
    }
}