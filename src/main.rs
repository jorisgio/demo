#![feature(collections_drain)]
extern crate num;
// These modules could be a library
#[allow(dead_code)]mod geometry;
#[allow(dead_code)]mod rtree;
mod parser;
mod game;

use std::io::{
    self,
    BufReader,
    Write,
};
use std::fmt::{
    self,
    Display,
};

use parser::Parser;
use game::GameMap;

impl<Coord : geometry::Coordinate> Display for geometry::Point<Coord> {

    fn fmt(&self, f : &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.get_x(), self.get_y())
    }
}

fn main() {
    let mut parser = Parser::new(BufReader::new(io::stdin()));

    let (mut game, moves) = 
        match parser.parse() {
            Ok((grid, rover, map, moves)) =>
                match GameMap::new(grid, rover, map) {
                    Ok(map) => (map, moves),
                    Err(e) =>
                    {
                        writeln!(io::stderr(), "Fromat error : {}", e).unwrap();
                    return
                    },
                },
            Err(e) => 
            {
                writeln!(io::stderr(), "Parsing error at line {}: {}", parser.line_number(), e).unwrap();
                return
            },
        };

    let count = game.move_rover_path(&moves);
    println!("{}", game.rover_pos());
    println!("{}", count);
}
