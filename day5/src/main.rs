use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let mut fresh_ingredient_ids: HashSet<u64> = HashSet::new();
    let mut fresh_ingredient_ranges: Vec<Vec<u64>> = Vec::new();
    let mut loading_fresh_ids = true;
    let mut num_ids_fresh = 0;
    for line in reader.lines() 
    {
        let line = line.expect("An error occurred while attempting to read a line.");
        if line == ""
        {
            loading_fresh_ids = false;
            continue;
        }
        if loading_fresh_ids 
        {
            let line: Vec<&str> = line.split("-").collect();
            fresh_ingredient_ranges.push(line.iter().map(|x| x.parse::<u64>().expect("")).collect());
        }
        else {
            let id = line.parse::<u64>().expect("");
            for range in &fresh_ingredient_ranges {
                if id >= range[0] && id <= range[1] {
                    fresh_ingredient_ids.insert(id);
                }
            }
        }
    }

    println!("Part one: {}", fresh_ingredient_ids.len());

    loop {
        let mut target_range = 0;
        let mut matching_range = 0;
        // Find areas of overlap and remove them
        for i in 0..fresh_ingredient_ranges.len() {
            for j in 0..fresh_ingredient_ranges.len() {
                if j == i {
                    continue;
                }
                if fresh_ingredient_ranges[i][0] <= fresh_ingredient_ranges[j][0] && fresh_ingredient_ranges[i][1] >= fresh_ingredient_ranges[j][0] {
                    target_range = i;
                    matching_range = j;
                }
                else if fresh_ingredient_ranges[i][0] <= fresh_ingredient_ranges[j][1] && fresh_ingredient_ranges[i][1] >= fresh_ingredient_ranges[j][1] {
                    target_range = i;
                    matching_range = j;
                }

                if target_range != matching_range {
                    break;
                }
            }
            if target_range != matching_range {
                break;
            }
        }

        if target_range == matching_range {
            break;
        }


        // Handle shortening ranges
        if fresh_ingredient_ranges[target_range][0] <= fresh_ingredient_ranges[matching_range][0] && fresh_ingredient_ranges[target_range][1] >= fresh_ingredient_ranges[matching_range][0] {
            fresh_ingredient_ranges[matching_range][0] = fresh_ingredient_ranges[target_range][1] + 1;
        } else {
            fresh_ingredient_ranges[matching_range][1] = fresh_ingredient_ranges[target_range][0] - 1;
        }
    }


    for range in fresh_ingredient_ranges {
        if range[1] >= range[0] {
            // inclusive range requires adding 1
            num_ids_fresh += range[1] - range[0] + 1;
        }
    }

    println!("Part two: {}", num_ids_fresh);
    
}
