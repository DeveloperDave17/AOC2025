use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use kdtree::KdTree;
use kdtree::distance::squared_euclidean;


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let mut junction_boxes = Vec::new();
    for line in reader.lines() 
    {
        let line = line.expect("An error occurred while attempting to read a line.");
        let xyz: Vec<u32> = line.split(",").map(|coord| coord.parse::<u32>().expect("Couldn't parse coordinate")).collect();
        junction_boxes.push(JunctionBox {x: xyz[0], y: xyz[1], z: xyz[2]});
    }

    let dimensions = 3;
    let mut junction_boxes_kd_tree = KdTree::new(dimensions);

    for junction_box in &junction_boxes {
        junction_boxes_kd_tree.add([junction_box.x as f64, junction_box.y as f64, junction_box.z as f64], junction_box).unwrap();
    }

    println!("part one: {}", part_one(&junction_boxes, &junction_boxes_kd_tree));
    println!("part two: {}", part_two(&junction_boxes, &junction_boxes_kd_tree));
}

fn part_one(junction_boxes: &Vec<JunctionBox>, junction_boxes_kd_tree: &KdTree<f64, &JunctionBox, [f64;3]>) -> usize {
    let mut closest_pairs: Vec<(&JunctionBox, &JunctionBox)> = Vec::new();

    while closest_pairs.len() < 1000 {
        let mut closest_pair: Option<(&JunctionBox, &JunctionBox)> = Option::None;
        let mut closest_distance: f64 = f64::MAX;
        for junction_box in junction_boxes {
            let mut n_nearest = 2;
            let (mut distance, mut closest_box): (f64, &&JunctionBox) = junction_boxes_kd_tree.nearest(&[junction_box.x as f64, junction_box.y as f64, junction_box.z as f64], n_nearest, &squared_euclidean).unwrap()[n_nearest - 1];
            while closest_pairs.contains(&(junction_box, *closest_box)) || closest_pairs.contains(&(*closest_box, junction_box)) {
                n_nearest += 1;
                (distance, closest_box) = junction_boxes_kd_tree.nearest(&[junction_box.x as f64, junction_box.y as f64, junction_box.z as f64], n_nearest, &squared_euclidean).unwrap()[n_nearest - 1];
            }
            if distance < closest_distance {
                closest_distance = distance;
                closest_pair = Option::Some((junction_box, *closest_box));
            }
        }

        match closest_pair {
            Some(pair) => {
                closest_pairs.push(pair);
            }
            _ => {}
        }
    }

    let mut circuit_helper: HashMap<&JunctionBox, i32> = HashMap::new();
    for (box_1, box_2) in &closest_pairs {
        circuit_helper.insert(*box_1, -1);
        circuit_helper.insert(*box_2, -1);
    }

    let mut circuits: Vec<Vec<&JunctionBox>> = Vec::new();
    for (box_1, box_2) in &closest_pairs {
        let group_1: i32 = *circuit_helper.get(box_1).expect("");
        let group_2: i32 = *circuit_helper.get(box_2).expect("");

        // do nothing
        if group_1 != -1 && group_1 == group_2 {
            continue;
        }

        if group_1 == -1 && group_2 == -1 {
            // create a new circuit
            circuit_helper.insert(*box_1, circuits.len() as i32);
            circuit_helper.insert(*box_2, circuits.len() as i32);
            circuits.push(vec![box_1, box_2]);
        } else if group_1 == -1 {
            circuit_helper.insert(*box_1, group_2);
            circuits[group_2 as usize].push(box_1);
        } else if group_2 == -1 {
            circuit_helper.insert(*box_2, group_1);
            circuits[group_1 as usize].push(box_2);
        } else {
            // merge circuits
            let mut combined_group: Vec<&JunctionBox> = Vec::new();
            combined_group.append(&mut circuits[group_1 as usize]);
            combined_group.append(&mut circuits[group_2 as usize]);
            circuits[group_1 as usize] = combined_group;
            circuits[group_2 as usize] = Vec::new();

            // update boxes groups
            for junc_box in &circuits[group_1 as usize] {
                circuit_helper.insert(*junc_box, group_1);
            }
        }
    }

    let mut longest = 0;
    let mut second_longest = 0;
    let mut third_longest = 0;
    for circuit in &circuits {
        let length = circuit.len();
        if length > longest {
            third_longest = second_longest;
            second_longest = longest;
            longest = length;
        } else if length > second_longest {
            third_longest = second_longest;
            second_longest = length;
        } else if length > third_longest {
            third_longest = length;
        }
    }
    longest * second_longest * third_longest
}

fn part_two(junction_boxes: &Vec<JunctionBox>, junction_boxes_kd_tree: &KdTree<f64, &JunctionBox, [f64;3]>) -> u64 {
    let mut last_x_coords_multiplied: u64 = 0;
    let mut circuit_map: HashMap<&JunctionBox, i32> = HashMap::new();
    let mut circuit_count = 0;

    for junction_box in junction_boxes {
        circuit_map.insert(junction_box, -1);
    }

    let mut completed = false;

    while !completed {
        let mut closest_pair: Option<(&JunctionBox, &JunctionBox)> = Option::None;
        let mut closest_distance: f64 = f64::MAX;
        for junction_box in junction_boxes {
            let point = [junction_box.x as f64, junction_box.y as f64, junction_box.z as f64];
            let mut iter = junction_boxes_kd_tree.iter_nearest(&point, &squared_euclidean).unwrap();
            // advance iterator past matching with self
            iter.next();
            let (mut distance, mut closest_box) : (f64, &&JunctionBox) = iter.next().unwrap();
            let group_1: i32 = *circuit_map.get(junction_box).expect("");
            let mut group_2: i32 = *circuit_map.get(closest_box).expect("");
            let mut no_more_matches = false;
            while  group_1 != -1 && group_2 != -1 && (group_1 == group_2) && !no_more_matches {
                let possible_match = iter.next();
                match possible_match {
                    Some(pair_info) => {
                        (distance, closest_box) = pair_info;
                        group_2 = *circuit_map.get(closest_box).expect("");
                    },
                    None => {
                        no_more_matches = true;
                        distance = f64::MAX;
                    }
                }
            }
            if distance < closest_distance {
                closest_distance = distance;
                closest_pair = Option::Some((junction_box, *closest_box));
            }
        }

        match closest_pair {
            Some(pair) => {
                let group_1: i32 = *circuit_map.get(pair.0).expect("");
                let group_2: i32 = *circuit_map.get(pair.1).expect("");
                if group_1 == -1 && group_2 == -1 {
                    // create a new circuit
                    circuit_map.insert(pair.0, circuit_count);
                    circuit_map.insert(pair.1, circuit_count);
                    circuit_count += 1;
                } else if group_1 == -1 {
                    circuit_map.insert(pair.0, group_2);
                } else if group_2 == -1 {
                    circuit_map.insert(pair.1, group_1);
                } else {
                    let group_to_merge: Vec<&JunctionBox> = circuit_map.iter().filter(|(_, &group)| group == group_2).map(|(&junction, _)| junction).collect();
                    // update boxes groups
                    for junc_box in group_to_merge {
                        circuit_map.insert(junc_box, group_1);
                    }
                }

                last_x_coords_multiplied = pair.0.x as u64 * pair.1.x as u64;
            }
            _ => {
                completed = true;
            }
        }
    }

    last_x_coords_multiplied
}
