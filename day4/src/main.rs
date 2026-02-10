use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let mut floor: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let line = line.expect("An error occurred while attempting to read a line.");
        floor.push(line.chars().collect());
    }

    let num_rolls = count_accessible_rolls_of_paper_part_one(&floor);

    println!("Part one: {}", num_rolls);

    let num_rolls_2 = count_removed_rolls_of_paper_part_two(&mut floor);

    println!("Part two: {}", num_rolls_2);
}

fn check_if_roll_is_removable(current_col: usize, current_row: usize, floor: &Vec<Vec<char>>) -> bool {
    let left_index: i32 = current_row as i32 - 1;
    let right_index = current_row as i32 + 1;
    let up_index = current_col as i32 - 1;
    let down_index = current_col as i32 + 1;

    let left = left_index >= 0;
    let right = right_index < floor[current_col].len() as i32;
    let up = up_index >= 0;
    let down = down_index < floor.len() as i32;

    let mut num_rolls_in_grid = 0;

    if left {
        if floor[current_col][left_index as usize] == '@' || floor[current_col][left_index as usize] == 'x' {
            num_rolls_in_grid += 1;
        }

        if up {
            if floor[up_index as usize][left_index as usize] == '@' || floor[up_index as usize][left_index as usize] == 'x' {
                num_rolls_in_grid += 1;
            }
        }

        if down {
            if floor[down_index as usize][left_index as usize] == '@' || floor[down_index as usize][left_index as usize] == 'x' {
                num_rolls_in_grid += 1;
            }
        }
    }

    if right {
        if floor[current_col][right_index as usize] == '@' || floor[current_col][right_index as usize] == 'x' {
            num_rolls_in_grid += 1;
        }

        if up {
            if floor[up_index as usize][right_index as usize] == '@' || floor[up_index as usize][right_index as usize] == 'x' {
                num_rolls_in_grid += 1;
            }
        }

        if down {
            if floor[down_index as usize][right_index as usize] == '@' || floor[down_index as usize][right_index as usize] == 'x'{
                num_rolls_in_grid += 1;
            }
        }
    }

    if up {
        if floor[up_index as usize][current_row] == '@' || floor[up_index as usize][current_row] == 'x' {
            num_rolls_in_grid += 1;
        }
    }

    if down {
        if floor[down_index as usize][current_row] == '@' || floor[down_index as usize][current_row] == 'x' {
            num_rolls_in_grid += 1;
        }
    }

    if num_rolls_in_grid < 4 {
        true
    } else {
        false
    }
}

fn count_accessible_rolls_of_paper_part_one(floor: &Vec<Vec<char>>) -> u32 {
    let mut floor_string: Vec<Vec<char>> = Vec::new();
    let mut num_accessible_rolls_of_papers: u32 = 0;
    for i in 0..floor.len() {
        floor_string.push(Vec::new());
        for j in 0..floor[i].len() {
            if floor[i][j] != '@' {
                floor_string[i].push('.');
                continue;
            }

            if check_if_roll_is_removable(i, j, &floor) {
                num_accessible_rolls_of_papers += 1;
                floor_string[i].push('x');
            } else {
                floor_string[i].push('@');
            }
        }
    }
    // let debug = floor_string.iter()
    //     .map(|row| 
    //         row.iter()
    //            .map(|&item| item.to_string())
    //            .collect::<Vec<String>>()
    //            .join("")
    //     )
    //     .collect::<Vec<String>>()
    //     .join("\n");
    // println!("{}", debug);
    num_accessible_rolls_of_papers
}

fn count_removed_rolls_of_paper_part_two(floor: & mut Vec<Vec<char>>) -> u32 {
    let mut num_removed_rolls_of_papers: u32 = 0;
    // 
    let mut num_cached_removed;
    loop {
        num_cached_removed = num_removed_rolls_of_papers;
        for i in 0..floor.len() {
            for j in 0..floor[i].len() {
                if floor[i][j] == '@' && check_if_roll_is_removable(i, j, &floor) {
                    floor[i][j] = 'x';
                }

            }
        }

        for i in 0..floor.len() {
            for j in 0..floor[i].len() {
                // Marked for removal, clean up
                if floor[i][j] == 'x' {
                    floor[i][j] = '.';
                    num_removed_rolls_of_papers += 1;
                }

            }
        }

        // let debug = floor.iter()
        // .map(|row| 
        //     row.iter()
        //        .map(|&item| item.to_string())
        //        .collect::<Vec<String>>()
        //        .join("")
        // )
        // .collect::<Vec<String>>()
        // .join("\n");

        // Cleaned up all the possible rolls
        if num_cached_removed == num_removed_rolls_of_papers {
            break;
        }
    }
    // let debug = floor.iter()
    //     .map(|row| 
    //         row.iter()
    //            .map(|&item| item.to_string())
    //            .collect::<Vec<String>>()
    //            .join("")
    //     )
    //     .collect::<Vec<String>>()
    //     .join("\n");
    // println!("{}", debug);  

    num_removed_rolls_of_papers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two() {
        let file = File::open("testinput.txt").expect("File not found");
        let reader = BufReader::new(file);

        let mut floor: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let line = line.expect("An error occurred while attempting to read a line.");
            floor.push(line.chars().collect());
        }

        assert_eq!(43, count_removed_rolls_of_paper_part_two(&mut floor));
    }
}