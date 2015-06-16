use std::mem;

use ::rtree::RTree;
use ::parser::{
    ParseError,
    RoverMove,
};
use ::geometry::{
    Tile,
    Point,
};

pub struct GameMap {
    rover : Point<i32>,
    dust_map : RTree<i32, Entity>,
    grid_top : Point<i32>,
}

impl GameMap {

    pub fn rover_pos(&self) -> Point<i32> {
        self.rover
    }

    pub fn new(grid_top : Point<i32>, rover : Point<i32>, dust : Vec<Point<i32>>) -> Result<GameMap, ParseError>
    {
        let arena = Tile::new(Point::new(0, 0), grid_top); 

        // Checks that the rover is on the map
        if rover <= arena {
            let mut rtree = RTree::<i32, Entity>::new();

            for p in dust {
                if p <= arena { 
                    rtree.insert(p, Entity::dust());
                } else {
                    return Err(ParseError::InvalidDustPosition);
                }
            }
            Ok(GameMap {
                rover : rover,
                dust_map : rtree,
                grid_top : grid_top,
            })
        } else {
            Err(ParseError::InvalidRoverPosition)
        }
    }

    /// Moves the rover into the given direction
    #[inline]
    fn move_rover(&mut self, dir : RoverMove) -> Point<i32> {
        let vector = dir.as_vector();
        let new_pos = self.rover + vector;
        if new_pos <= self.grid_top && new_pos >= Point::new(0, 0) { 
            self.rover = self.rover + vector;
        } 
        self.rover
    }

    /// Moves the rover along the given path and returns the cleaned dust tiles
    pub fn move_rover_path(&mut self, moves : &[RoverMove]) -> usize {
        let mut map = RTree::<i32, Entity>::new();
        mem::swap(&mut map, &mut self.dust_map);
        let count = 
        moves.iter()
            .cloned()
            .map(|d| self.move_rover(d))
            // Uncomment to print the rover path
            // .inspect(|pos| println!("{:?}", pos))
            .filter(|pos| map.find_mut(*pos).map(|d| d.clean_dust()).unwrap_or(false))
            .count();
        mem::swap(&mut map, &mut self.dust_map);
        count
    }
}

impl RoverMove {
    pub fn as_vector(self) -> Point<i32> {
        match self {
            RoverMove::North => Point::new(0, 1),
            RoverMove::South => Point::new(0, -1),
            RoverMove::East => Point::new(1, 0),
            RoverMove::West => Point::new(-1, 0),
        }
    }
}

/// A game tile
#[derive(Debug)]
struct Entity {
    dust : bool,
}

impl Entity {

    /// Creates an entity with dust 
    fn dust() -> Entity {
        Entity {
            dust : true,
        }
    }

    fn clean_dust(&mut self) -> bool  {
        let is_dust = self.dust;
        self.dust = false;
        is_dust
    }
}

