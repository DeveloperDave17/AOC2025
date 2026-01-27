use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.expect("error reading line");
        let id_ranges: Vec<&str> = line.split(",").collect();
        find_invalid_ids_part_one(&id_ranges);
        find_invalid_ids_part_two(&id_ranges);
    }
}

fn find_invalid_ids_part_one(id_ranges: &Vec<&str>) {
    let mut invalid_id_sum = 0;
    for id_range in id_ranges {
        let ids: Vec<&str> = id_range.split("-").collect();
        let lower = ids[0].parse::<u64>().expect("Couldn't parse lower bound");
        let upper = ids[1].parse::<u64>().expect("Couldn't parse upper bound");
        invalid_id_sum += find_invalid_ids_in_range(lower, upper);
    }
    println!("part one, invalid id count: {}", invalid_id_sum);
}

fn find_invalid_ids_in_range(lower: u64, upper: u64) -> u64 {
    let mut invalid_id_sum = 0;
    for id in lower..=upper {
        let id_string = id.to_string();
        let midpoint = id_string.chars().count() / 2;
        let first_half: String = id_string.chars().take(midpoint).collect();
        let second_half: String = id_string.chars().skip(midpoint).collect();
        if first_half == second_half {
            invalid_id_sum += id;
        }
    }
    invalid_id_sum
}

fn find_invalid_ids_part_two(id_ranges: &Vec<&str>) {
    let mut invalid_id_sum = 0;
    for id_range in id_ranges {
        let ids: Vec<&str> = id_range.split("-").collect();
        let lower = ids[0].parse::<u64>().expect("Couldn't parse lower bound");
        let upper = ids[1].parse::<u64>().expect("Couldn't parse upper bound");
        invalid_id_sum += find_invalid_ids_in_range_rec(lower, upper);
    }
    println!("part two, invalid id count: {}", invalid_id_sum);
}

fn find_invalid_ids_in_range_rec(lower: u64, upper: u64) -> u64 {
    let mut invalid_id_sum = 0;
    for id in lower..=upper {
        let midpoint = id.to_string().len() / 2;
        invalid_id_sum += find_invalid_ids_in_range_rec_helper(id, midpoint);
    }
    invalid_id_sum
}

fn find_invalid_ids_in_range_rec_helper(id: u64, repeating_size: usize) -> u64 {
    // base case
    if repeating_size == 0 {
        return 0;
    }

    let id_string = id.to_string();
    let id_chars: Vec<char> = id_string.chars().collect();
    if id_chars.len() % repeating_size == 0 {
        let id_chunks: Vec<String> = id_chars.chunks(repeating_size).map(|chunk| chunk.iter().collect()).collect();
        let id_set: HashSet<String> = id_chunks.into_iter().collect();
        if id_set.len() == 1 {
            return id;
        }
    }

    find_invalid_ids_in_range_rec_helper(id, repeating_size - 1)
}