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

const EPSILON: f64 = 0.0001;
const TILE_LENGTH: f64 = 1.0;

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
            let area = ( (tile.x - tile_2.x).abs() + TILE_LENGTH ) * ( ( tile.y - tile_2.y ).abs() + TILE_LENGTH);

            if area > largest_area {
                largest_area = area;
            }
        }
    }

    println!("part one: {}", largest_area);

    // reset largest area for part two
    largest_area = 0.0;

    for tile in &red_tiles {
        for tile_2 in &red_tiles {
 
            let mut cloned_tile_1 = Tile {x: tile.x, y: tile.y};
            let mut cloned_tile_2 = Tile {x: tile_2.x, y: tile_2.y};

            // shrink area of rectangle slightly
            if cloned_tile_1.x > cloned_tile_2.x {
                cloned_tile_1.x -= EPSILON;
                cloned_tile_2.x += EPSILON;
            } else {
                cloned_tile_2.x -= EPSILON;
                cloned_tile_1.x += EPSILON;
            }

            // continue to shrink area of rectangle
            if cloned_tile_1.y > cloned_tile_2.y {
                cloned_tile_1.y -= EPSILON;
                cloned_tile_2.y += EPSILON;
            } else {
                cloned_tile_2.y -= EPSILON;
                cloned_tile_1.y += EPSILON;
            }

            // opposite corners..
            let opp_tile_1 = Tile {x: cloned_tile_1.x, y: cloned_tile_2.y};
            let opp_tile_2 = Tile {x: cloned_tile_2.x, y: cloned_tile_1.y};

            let edge_1 = Edge {tile_1: &cloned_tile_1, tile_2: &opp_tile_1};
            let edge_2 = Edge {tile_1: &cloned_tile_2, tile_2: &opp_tile_1};
            let edge_3 = Edge {tile_1: &cloned_tile_1, tile_2: &opp_tile_2};
            let edge_4 = Edge {tile_1: &cloned_tile_2, tile_2: &opp_tile_2};

            // assume rectangle innocence
            let mut within = true;
            for i in 0..(red_tiles.len()) {
                let vertex_1 = &red_tiles[i];
                let vertex_2 = &red_tiles[(i + 1) % red_tiles.len()];
               
                within &= !lines_intersect(&edge_1, vertex_1, vertex_2) && !lines_intersect(&edge_2, vertex_1, vertex_2) && !lines_intersect(&edge_3, vertex_1, vertex_2) && !lines_intersect(&edge_4, vertex_1, vertex_2);
            }

            if within {
                // difference + 1 to account for the length of a single tile
                let area = ( (tile.x - tile_2.x).abs() + TILE_LENGTH ) * ( ( tile.y - tile_2.y ).abs() + TILE_LENGTH );
                if area > largest_area {
                    largest_area = area;
                }
            }
        }
    }

    println!("part two: {}", largest_area);
}

fn lines_intersect_helper(a: &Tile, b: &Tile, c: &Tile) -> bool {
    (c.y- a.y) * (b.x-a.x) > (b.y-a.y) * (c.x-a.x)
}

fn lines_intersect(edge: &Edge, vertex_1: &Tile, vertex_2: &Tile) -> bool {
    lines_intersect_helper(edge.tile_1, vertex_1, vertex_2) != lines_intersect_helper(edge.tile_2, vertex_1, vertex_2) 
    && lines_intersect_helper(edge.tile_1, edge.tile_2, vertex_1) != lines_intersect_helper(edge.tile_1, edge.tile_2, vertex_2)
}
