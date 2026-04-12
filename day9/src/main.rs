use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
struct Tile {
    x: f64,
    y: f64
}

struct Edge<'a, 'b> {
    tile_1: &'a Tile,
    tile_2: &'b Tile
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);

    let mut red_tiles = Vec::new();
    for line in reader.lines() 
    {
        let line = line.expect("An error occurred while attempting to read a line.");
        let xy: Vec<f64> = line.split(",").map(|coord| coord.parse::<f64>().expect("Couldn't parse coordinate")).collect();
        red_tiles.push(Tile {x: xy[0], y: xy[1]});
    }

    let mut largest_area = 0.0;

    for tile in &red_tiles {
        for tile_2 in &red_tiles {
            // difference + 1 to account for the length of a single tile
            let area = ( (tile.x - tile_2.x).abs() + 1.0 ) * ( ( tile.y - tile_2.y ).abs() + 1.0);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("part one: {}", largest_area);


    // Algorithm:
    // 1. Find all the outer green tiles
    // 2. Fill in the gaps between green tiles (gets the inner tiles)
    // 3. Iterate through all the red tile combinations
    //      a. Determine if the combination's outer edges are comprised of only red or green tiles
    //      b. If so, Calculate the area between red tiles
    //      c. replace the largest_area the new area is larger

    // too slow!
    // use ray casting...

    // let mut green_tiles: HashSet<Tile> = HashSet::new();

    // for i in 0..(red_tiles.len() - 1) {
    //     let tile_1 = &red_tiles[i];
    //     let tile_2 = &red_tiles[i + 1];
    //     if tile_1.x == tile_2.x {
    //         for y in (tile_1.y)..=tile_2.y {
    //             green_tiles.insert(Tile {x: tile_1.x, y: y});
    //         }
    //     } else {
    //         for x in (tile_1.x)..=tile_2.x {
    //             green_tiles.insert(Tile {x: x, y: tile_1.y});
    //         }
    //     }
    // }
    
    // // Dont forget the special case where the list wraps
    // let tile_1 = &red_tiles[0];
    // let tile_2 = &red_tiles[red_tiles.len() - 1];
    // if tile_1.x == tile_2.x {
    //     for y in (tile_1.y)..=tile_2.y {
    //         green_tiles.insert(Tile {x: tile_1.x, y: y});
    //     }
    // } else {
    //     for x in (tile_1.x)..=tile_2.x {
    //         green_tiles.insert(Tile {x: x, y: tile_1.y});
    //     }
    // }

    // // Red tiles might as well be green tiles too...
    // for tile in &red_tiles {
    //     green_tiles.insert(Tile {x: tile.x, y: tile.y});
    // }

    // // Lets fill in any holes...
    // let mut inner_tiles: HashSet<Tile> = HashSet::new();
    // for tile in &green_tiles {
    //     for tile_2 in &green_tiles {
    //         if tile.x == tile_2.x {
    //             if tile_2.y > tile.y {
    //                 for y in tile.y..=tile_2.y {
    //                     inner_tiles.insert(Tile{x: tile.x, y: y});
    //                 }
    //             } else {
    //                 for y in tile_2.y..=tile.y {
    //                     inner_tiles.insert(Tile{x: tile.x, y: y});
    //                 }
    //             }
    //         } else if tile.y == tile_2.y {
    //             if tile_2.x > tile.x {
    //                 for x in tile.x..=tile_2.x {
    //                     inner_tiles.insert(Tile{x: x, y: tile.y});
    //                 }
    //             } else {
    //                 for x in tile_2.x..=tile.x {
    //                     inner_tiles.insert(Tile{x: x, y: tile.y});
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("made it this far");

    // // add tiles
    // green_tiles.extend(inner_tiles);

    // reset largest area for part two
    largest_area = 0.0;

    for tile in &red_tiles {
        for tile_2 in &red_tiles {
 
            let mut cloned_tile_1 = Tile {x: tile.x, y: tile.y};
            let mut cloned_tile_2 = Tile {x: tile_2.x, y: tile_2.y};

            // shrink rectangle slightly
            if cloned_tile_1.x > cloned_tile_2.x {
                cloned_tile_1.x -= 0.0001;
                cloned_tile_2.x += 0.0001;
            } else {
                cloned_tile_2.x -= 0.0001;
                cloned_tile_1.x += 0.0001;
            }

            if cloned_tile_1.y > cloned_tile_2.y {
                cloned_tile_1.y -= 0.0001;
                cloned_tile_2.y += 0.0001;
            } else {
                cloned_tile_2.y -= 0.0001;
                cloned_tile_1.y += 0.0001;
            }

            // opposite corners..
            let opp_tile_1 = Tile {x: cloned_tile_1.x, y: cloned_tile_2.y};
            let opp_tile_2 = Tile {x: cloned_tile_2.x, y: cloned_tile_1.y};

            let edge_1 = Edge {tile_1: &cloned_tile_1, tile_2: &opp_tile_1};
            let edge_2 = Edge {tile_1: &cloned_tile_2, tile_2: &opp_tile_1};
            let edge_3 = Edge {tile_1: &cloned_tile_1, tile_2: &opp_tile_2};
            let edge_4 = Edge {tile_1: &cloned_tile_2, tile_2: &opp_tile_2};

            let mut within = true;
            for i in 0..(red_tiles.len()) {
                let vertex_1 = &red_tiles[i];
                let vertex_2 = &red_tiles[(i + 1) % red_tiles.len()];
               
                within &= !lines_intersect(&edge_1, vertex_1, vertex_2) && !lines_intersect(&edge_2, vertex_1, vertex_2) && !lines_intersect(&edge_3, vertex_1, vertex_2) && !lines_intersect(&edge_4, vertex_1, vertex_2);
            }

            if within {
                // difference + 1 to account for the length of a single tile
                let area = ( (tile.x - tile_2.x).abs() + 1.0 ) * ( ( tile.y - tile_2.y ).abs() + 1.0);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    println!("part two: {}", largest_area);
}


// fn determine_if_length_is_valid(value_is_x: bool, lower_value: u64, greater_value: u64, constant_value: u64, green_tiles: &HashSet<Tile>) -> bool {
//     // assume innocence
//     let mut length_valid = false;

//     for value in lower_value..=greater_value {
//         if value_is_x {
//             length_valid &= green_tiles.contains(&Tile {x: value, y: constant_value});
//         } else {
//             length_valid &= green_tiles.contains(&Tile {x: constant_value, y: value});
//         }
//     }

//     length_valid
// }


// fn ray_intersects_segment(tile: &Tile, vertex_1: &Tile, vertex_2: &Tile) -> bool {
//     let x = tile.x as f64;
//     let mut y = tile.y as f64;

//     let lower_tile =  if vertex_1.y < vertex_2.y { vertex_1 } else { vertex_2 };
//     let upper_tile = if vertex_1.y > vertex_2.y { vertex_1 } else { vertex_2 }; 

//     if y == lower_tile.y as f64 || y == upper_tile.y as f64 {
//         y += 0.00001;
//     }

//     if (y > upper_tile.y as f64 || y < lower_tile.y as f64) || x > lower_tile.x.max(upper_tile.x) as f64 {
//         false
//     } else if x < lower_tile.x.min(upper_tile.x) as f64 {
//         true
//     } else {
//         let m_red = if (lower_tile.x - upper_tile.x).abs() as f64 > f64::MIN_POSITIVE {
//             (upper_tile.y - lower_tile.y) as f64 / (upper_tile.x - lower_tile.x) as f64
//         } else {
//             f64::MAX
//         };
//         let m_blue = if (lower_tile.x as f64 - x).abs() as f64 > f64::MIN_POSITIVE {
//             (y - lower_tile.y as f64) / (x - lower_tile.x as f64)
//         } else {
//             f64::MAX
//         };
//         m_blue >= m_red
//     }

    // if y == lower_tile.y as f64 || y == upper_tile.y as f64 {
    //     y += 0.000000001;
    // }

    // if y < lower_tile.y as f64 || y > upper_tile.y as f64{
    //     return false;
    // }
    // else if x >= max(lower_tile.x, upper_tile.x) as f64 {
    //     return false;
    // }
    // else {
    //     if x < min(lower_tile.x, upper_tile.x) as f64{
    //         return true;
    //     }
    //     else {
    //         let mut red = f64::MAX;
    //         if lower_tile.x != upper_tile.x {
    //             red = (upper_tile.y as f64 - lower_tile.y as f64) / (upper_tile.x as f64 - lower_tile.x as f64);
    //         }

    //         let mut blue = f64::MAX;
    //         if lower_tile.x as f64 != x {
    //             blue = (y - lower_tile.y as f64) / (x - lower_tile.x as f64);
    //         }

    //         if blue >= red {
    //             return true;
    //         }

    // //         return false;
    //     }
    // }
    
// }

fn lines_intersect_helper(a: &Tile, b: &Tile, c: &Tile) -> bool{
    (c.y- a.y) * (b.x-a.x) > (b.y-a.y) * (c.x-a.x)
}

// fn lines_intersect_helper(a_x: f64, a_y: f64, b_x: f64, b_y: f64, c_x: f64, c_y: f64) -> bool {
//     (c_y- a_y) * (b_x-a_x) > (b_y-a_y) * (c_x-a_x)
// }

fn lines_intersect(edge: &Edge, vertex_1: &Tile, vertex_2: &Tile) -> bool {
    lines_intersect_helper(edge.tile_1, vertex_1, vertex_2) != lines_intersect_helper(edge.tile_2, vertex_1, vertex_2) && lines_intersect_helper(edge.tile_1, edge.tile_2, vertex_1) != lines_intersect_helper(edge.tile_1, edge.tile_2, vertex_2)
}
