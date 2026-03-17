use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
enum TachyonManifoldLocationType {
    EmptySpace,
    Start,
    Splitter(u64),
    Beam(u64)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let mut tachyon_manifold_diagram: Vec<Vec<TachyonManifoldLocationType>> = Vec::new();
    for line in reader.lines() 
    {
        let line = line.expect("An error occurred while attempting to read a line.");
        tachyon_manifold_diagram.push(line.chars().map(|c| 
            match c {
                'S' => TachyonManifoldLocationType::Start,
                '^' => TachyonManifoldLocationType::Splitter(0),
                _ => TachyonManifoldLocationType::EmptySpace
            }
        ).collect());
    }

    let part_one_split_count = part_one(&mut tachyon_manifold_diagram);
    println!("part one split count: {}", part_one_split_count);

    let timeline_count: u64 = part_two(&mut tachyon_manifold_diagram);

    println!("part two timeline count: {}", timeline_count);
}

fn part_one_helper(tachyon_manifold_diagram: &mut Vec<Vec<TachyonManifoldLocationType>>, current_row: usize, current_col: usize) -> u64 {
    let mut split_count = 0;
    // Check for end of diagram
    if current_row >= tachyon_manifold_diagram.len() || current_col >= tachyon_manifold_diagram[0].len() {
        return split_count
    }
    
    match tachyon_manifold_diagram[current_row][current_col] {
        TachyonManifoldLocationType::EmptySpace => {
            tachyon_manifold_diagram[current_row][current_col] = TachyonManifoldLocationType::Beam(1);
            split_count = part_one_helper(tachyon_manifold_diagram, current_row + 1, current_col);
        },
        TachyonManifoldLocationType::Splitter(_) => {
            if current_col > 0 {
                split_count += part_one_helper(tachyon_manifold_diagram, current_row + 1, current_col - 1);
            }

            split_count += part_one_helper(tachyon_manifold_diagram, current_row + 1, current_col + 1);

            // count split
            split_count += 1;
        },
        // Creates timelines for part 2
        // TachyonManifoldLocationType::Beam(timeline_count) => {
        //     tachyon_manifold_diagram[current_row][current_col] = TachyonManifoldLocationType::Beam(timeline_count + 1);
        //     part_one_helper(tachyon_manifold_diagram, current_row + 1, current_col);
        // }
        _ => {}
    }

    split_count
}

fn part_one(tachyon_manifold_diagram: &mut Vec<Vec<TachyonManifoldLocationType>>) -> u64 {
    // Find Starting column
    let starting_column = tachyon_manifold_diagram[0].iter().position(|location| *location == TachyonManifoldLocationType::Start).expect("");
    part_one_helper(tachyon_manifold_diagram, 1, starting_column)
}

fn part_two(tachyon_manifold_diagram: &mut Vec<Vec<TachyonManifoldLocationType>>) -> u64 {
    for row_index in 1..tachyon_manifold_diagram.len() {
        for col_index in 0..tachyon_manifold_diagram[row_index].len() {
            match tachyon_manifold_diagram[row_index][col_index] {
                TachyonManifoldLocationType::Beam(_) => {
                    let mut timeline_count = 0;
                    if col_index < tachyon_manifold_diagram[row_index].len() - 1 {
                        match tachyon_manifold_diagram[row_index - 1][col_index + 1] {
                            TachyonManifoldLocationType::Splitter(count) => {
                                timeline_count += count;
                            },
                            _ => {}
                        }
                    }

                    if col_index > 0 {
                        match tachyon_manifold_diagram[row_index - 1][col_index - 1] {
                            TachyonManifoldLocationType::Splitter(count) => {
                                timeline_count += count;
                            },
                            _ => {}
                        }
                    }

                    match tachyon_manifold_diagram[row_index - 1][col_index] {
                        TachyonManifoldLocationType::Beam(count) => {
                            timeline_count += count;
                        },
                        TachyonManifoldLocationType::Start => {
                            timeline_count += 1;
                        }
                        _ => {}
                    }
                    tachyon_manifold_diagram[row_index][col_index] = TachyonManifoldLocationType::Beam(timeline_count);
                },
                TachyonManifoldLocationType::Splitter(_) => {
                    let mut timeline_count = 0;
                    match tachyon_manifold_diagram[row_index - 1][col_index] {
                        TachyonManifoldLocationType::Beam(count) => {
                            timeline_count += count;
                        },
                        _ => {}
                    }
                    tachyon_manifold_diagram[row_index][col_index] = TachyonManifoldLocationType::Splitter(timeline_count);
                },
                _ => {}
            }
        }
    }

    let count = tachyon_manifold_diagram.last().expect("No rows in diagram").iter().map(|manifold| match manifold {
        TachyonManifoldLocationType::Beam(timeline_count) => *timeline_count,
        _ => 0
    }).sum();

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let file = File::open("example.txt").expect("File not found");
        let reader = BufReader::new(file);

        let mut tachyon_manifold_diagram: Vec<Vec<TachyonManifoldLocationType>> = Vec::new();
        for line in reader.lines() 
        {
            let line = line.expect("An error occurred while attempting to read a line.");
            tachyon_manifold_diagram.push(line.chars().map(|c| 
                match c {
                    'S' => TachyonManifoldLocationType::Start,
                    '^' => TachyonManifoldLocationType::Splitter(0),
                    _ => TachyonManifoldLocationType::EmptySpace
                }
            ).collect());
        }

        let part_one_split_count = part_one(&mut tachyon_manifold_diagram);
        assert_eq!(part_one_split_count, 21);

        let timeline_count: u64 = part_two(&mut tachyon_manifold_diagram);
        assert_eq!(timeline_count, 40);
    }
}
