//! Geometric shapes in a 2D cartesian plane.
use std::ops::{
    Add,
    Sub,
};
use std::num::Zero;
use std::cmp::{
    PartialOrd,
    Eq,
    PartialEq,
    Ordering,
};
use std::fmt::Debug;

/// A marker trait for an axis coordinate representation
pub trait Coordinate : Debug + Eq + PartialOrd + Zero + Copy + Add<Output = Self> + Sub<Output = Self> { }
impl Coordinate for usize {}
impl Coordinate for u64 {}
impl Coordinate for u32 {}
impl Coordinate for u16 {}
impl Coordinate for u8 {}
impl Coordinate for isize {}
impl Coordinate for i64 {}
impl Coordinate for i32 {}
impl Coordinate for i16 {}
impl Coordinate for i8  {}

/// A point in the cartesian coordinate system in a 2D plane
#[derive(Debug)]
pub struct Point<Coord : Coordinate> {
    x : Coord,
    y : Coord,
}

impl<Coord : Coordinate> Point<Coord> {

    /// Creates a cartesian point from the two coordinates.
    pub fn new(x : Coord, y : Coord) -> Point<Coord> {
        Point {
            x : x,
            y : y,
        }
    }
}

impl<Coord : Coordinate> PartialEq for Point<Coord> {
    /// Compares the coordinates and returns true if the two points represent the same location in
    /// the 2D plane
    fn eq(&self, rhs : &Point<Coord>) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

/// Equality on points is an equivalence relation if the axis coordinates have an equivalent
/// equality
impl<Coord : Coordinate> Eq for Point<Coord> { }

impl<'a, Coord : Coordinate> Sub<&'a Point<Coord>> for &'a Point<Coord> {
    type Output = Point<Coord>;

    /// Vector subtraction. Returns a point translated by the given vector
    fn sub(self, rhs : &'a Point<Coord>) -> Point<Coord> {
        Point {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
        }
    }
}

impl<'a, Coord : Coordinate> Add<&'a Point<Coord>> for &'a Point<Coord> {
    type Output = Point<Coord>;

    /// Vector addition. Returns a point translated by the given vector
    fn add(self, rhs : &'a Point<Coord>) -> Point<Coord> {
        Point {
            x : self.x + rhs.x,
            y : self.y + rhs.y,
        }
    }
}

/// A point is equal to another if it has the same coordinates
/// A point is greater to another if both of its coordinates are greater 
/// A point is smaller to another if both of its coordinates are smaller
/// In any other case, the order is undefined
impl<Coord : Coordinate > PartialOrd for Point<Coord> {

    fn partial_cmp(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        if self.x < rhs.x && self.y < rhs.y {
            Some(Ordering::Less)
        } else if self.x > rhs.x && self.y > rhs.y {
            Some(Ordering::Greater)
        } else if self.x == rhs.x && self.y == rhs.y {
            Some(Ordering::Equal)
        } else {
            None
        }
    }
}

/// A `Tile` represents a bounded rectangular area in a 2D plane.
#[derive(Debug)]
pub struct Tile<Coord : Coordinate> {
    center : Point<Coord>,
    vector : Point<Coord>,
} 

impl<Coord : Coordinate> Tile<Coord> {

    /// Creates a new `Tile` centered at `center` with the given `half_height` and `half_width`
    pub fn new(center : Point<Coord>, half_width : Coord, half_height : Coord) -> Tile<Coord> {
        Tile {
            center : center,
            vector : Point { x : half_width, y : half_height },
        }
    }

}

/// A tile is equal to a point if it is centered on this point and has a null width and height
impl<Coord : Coordinate> PartialEq<Point<Coord>> for Tile<Coord> {

    fn eq(&self, rhs : &Point<Coord>) -> bool {
        &self.center == rhs 
            && self.vector.x == <Coord as Zero>::zero()
            && self.vector.y == <Coord as Zero>::zero()
    }
}

/// Order between points and Tile. 
/// A tile is greater than a point if it contains it. 
/// A tile is smaller than a point if it doesn't contain it.
/// A tile is equal to a point if it's centered to this point and has a null width and height
impl<Coord : Coordinate> PartialOrd<Point<Coord>> for Tile<Coord> 
{

    fn partial_cmp(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        if self == rhs {
            Some(Ordering::Equal)
        } else {
            let bottom_corner = &self.center - &self.vector;
            let top_corner = &self.center + &self.vector;
            if rhs >= &bottom_corner && rhs < &top_corner {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::cmp::Ordering;
    use super::*;

    #[test]
    fn points_equality() {
        let p1 = Point::new(5usize, 4usize);
        let p2 = Point::new(5usize, 4usize);
        let p3 = Point::new(5usize, 3usize);
        let p4 = Point::new(4usize, 5usize);
        assert!(p1 == p2);
        assert!(p2 == p1);
        assert!(p1 != p3);
        assert!(p3 != p1);
        assert!(p1 != p4);
        assert!(p4 != p1);
    }

    #[test]
    fn points_ordering() {
        let p1 = Point::new(5usize, 4usize);
        let p2 = Point::new(5usize, 4usize);
        let p3 = Point::new(5usize, 3usize);
        let p4 = Point::new(4usize, 3usize);
        assert!(p1.partial_cmp(&p2) == Some(Ordering::Equal));
        assert_eq!(p2 >= p3, false);
        assert!(p2 > p4);
        assert!(p4 < p2);
    }

    #[test]
    fn tile_equality() {
        let p1 = Point::new(10, 10);
        let tile = Tile::new(Point::new(10usize, 10usize), 0, 0);
        let tile2 = Tile::new(Point::new(10usize, 10usize), 1, 0);
        let tile3 = Tile::new(Point::new(11usize, 10usize), 0, 0);
        let tile4 = Tile::new(Point::new(11usize, 10usize), 2, 0);

        assert!(tile == p1);
        assert!(tile2 != p1);
        assert!(tile3 != p1);
    }

    #[test]
    fn tile_ordering() {
        let p1 = Point::new(10, 10);

        let tile = Tile::new(Point::new(10usize, 10usize), 0, 0);
        assert!(tile <= p1);
        assert_eq!(tile < p1, false);

        let tile2 = Tile::new(Point::new(10usize, 10usize), 5, 2);
        assert!(tile2 > p1);

        assert!(tile2 > Point::new(5, 8));
        assert!(tile2 < Point::new(15, 10));
        assert!(tile2 < Point::new(15, 12));
        assert!(tile2 < Point::new(7, 22));
        assert!(tile2 > Point::new(7, 11));
    }
}
