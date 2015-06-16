//! Geometric shapes in a 2D cartesian plane.
use std::ops::{
    Add,
    Sub,
    Mul,
};

use std::cmp::{
    PartialOrd,
    Eq,
    PartialEq,
    Ordering,
    Ord,
    min,
    max,
};
use std::fmt::{Display, Debug};
use ::num::traits::{One, Zero};

/// A marker trait for an axis coordinate representation
pub trait Coordinate : Debug + Display + Eq + Ord + PartialOrd + Clone + Copy + One + Zero +
Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + 'static { }

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
#[derive(Debug, Clone, Copy)]
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

    /// Orders two points by their x coordinates
    #[inline]
    pub fn vertical_cmp(self, rhs : Point<Coord>) -> Ordering {
        self.x.cmp(&rhs.x)
    }

    /// Orders two points by their y coordinates
    #[inline]
    pub fn horizontal_cmp(self, rhs : Point<Coord>) -> Ordering {
        self.y.cmp(&rhs.y)
    }

    pub fn get_x(&self) -> Coord { self.x }
    pub fn get_y(&self) -> Coord { self.y }
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

impl<Coord : Coordinate> Sub<Point<Coord>> for Point<Coord> {
    type Output = Point<Coord>;

    /// Vector subtraction. Returns a point translated by the given vector
    fn sub(self, rhs : Point<Coord>) -> Point<Coord> {
        Point {
            x : self.x - rhs.x,
            y : self.y - rhs.y,
        }
    }
}

impl<Coord : Coordinate> Add<Point<Coord>> for Point<Coord> {
    type Output = Point<Coord>;

    /// Vector addition. Returns a point translated by the given vector
    fn add(self, rhs : Point<Coord>) -> Point<Coord> {
        Point {
            x : self.x + rhs.x,
            y : self.y + rhs.y,
        }
    }
}

/// A point is equal to another if it has the same coordinates
/// A point is greater to another if one of its coordinates is greater
/// A point is smaller to another if one of its coordinates is smaller
/// In any other case, the order is undefined
impl<Coord : Coordinate > PartialOrd for Point<Coord> {

    fn partial_cmp(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        if self.x == rhs.x {
            self.y.partial_cmp(&rhs.y)
        } else if self.y == rhs.y {
            self.x.partial_cmp(&rhs.x)
        } else {
            if self.x < rhs.x && self.y < rhs.y {
                Some(Ordering::Less)
            } else if self.x > rhs.x && self.y > rhs.y {
                Some(Ordering::Greater)
            } else {
                None
            }
        }
    }
}

/// A vertical line
#[derive(Debug, Clone, Copy)]
pub struct VerticalLine<Coord : Coordinate> {
    x : Coord,
}

impl<Coord : Coordinate> VerticalLine<Coord> {

    /// Returns the grid line intersecting the axis at x 
    pub fn new(x : Coord) -> VerticalLine<Coord> {
        VerticalLine {
            x : x,

        }
    }

    /// Returns the vertical line passing by the given point
    pub fn at_point(p : Point<Coord>) -> VerticalLine<Coord> {
        VerticalLine {
            x : p.x
        }
    }
}

impl<Coord : Coordinate> Add<Coord> for VerticalLine<Coord> {

    type Output = VerticalLine<Coord>;

    fn add(self, rhs : Coord) -> VerticalLine<Coord> {
        VerticalLine { x : self.x + rhs }
    }
}


impl<Coord : Coordinate> Sub<Coord> for VerticalLine<Coord> {

    type Output = VerticalLine<Coord>;

    fn sub(self, rhs : Coord) -> VerticalLine<Coord> {
        VerticalLine { x : self.x - rhs }
    }
}
/// Returns `Equal` if the line intersects the tile, `Less` if the line is strictly below the
/// tile or `Greater` if the line is strictly after the tile
impl<Coord : Coordinate> PartialOrd<Tile<Coord>> for VerticalLine<Coord> {

    fn partial_cmp(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        let order_left = self.x.cmp(&rhs.bottom.x);
        let order_right = self.x.cmp(&rhs.top.x);

        match (order_left, order_right) {
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Equal),
        }
    }
}

impl<Coord : Coordinate> PartialEq<Tile<Coord>> for VerticalLine<Coord> {

    fn eq(&self, rhs : &Tile<Coord>) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
    }
}

/// Returns `Equal` if the point is on the line, `Less` if it is at the left or
///  `Greater` if the point is right to the line
impl<Coord : Coordinate> PartialOrd<Point<Coord>> for VerticalLine<Coord> {

    fn partial_cmp(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        Some(self.x.cmp(&rhs.x))

    }
}

impl<Coord : Coordinate> PartialEq<Point<Coord>> for VerticalLine<Coord> {

    fn eq(&self, rhs : &Point<Coord>) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
    }
}

/// An horizontal line
#[derive(Debug, Clone, Copy)]
pub struct HorizontalLine<Coord : Coordinate> {
    y : Coord,
}

impl<Coord : Coordinate> HorizontalLine<Coord> {

    /// Returns the grid line intersecting the ayis at y 
    pub fn new(y : Coord) -> HorizontalLine<Coord> {
        HorizontalLine {
            y : y,

        }
    }

    /// Returns the horizontal line passing by the given point
    pub fn at_point(p : Point<Coord>) -> HorizontalLine<Coord> {
        HorizontalLine {
            y : p.y
        }
    }
}

impl<Coord : Coordinate> Add<Coord> for HorizontalLine<Coord> {

    type Output = HorizontalLine<Coord>;

    fn add(self, rhs : Coord) -> HorizontalLine<Coord> {
        HorizontalLine { y : self.y + rhs }
    }
}


impl<Coord : Coordinate> Sub<Coord> for HorizontalLine<Coord> {

    type Output = HorizontalLine<Coord>;

    fn sub(self, rhs : Coord) -> HorizontalLine<Coord> {
        HorizontalLine { y : self.y - rhs }
    }
}

/// Returns `Equal` if the line intersects the tile, `Less` if the line is strictly below the
/// tile or `Greater` if the line is strictly after the tile
impl<Coord : Coordinate> PartialOrd<Tile<Coord>> for HorizontalLine<Coord> {

    fn partial_cmp(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        let order_left = self.y.cmp(&rhs.bottom.y);
        let order_right = self.y.cmp(&rhs.top.y);

        match (order_left, order_right) {
            (Ordering::Less, Ordering::Less) => Some(Ordering::Less),
            (Ordering::Greater, Ordering::Greater) => Some(Ordering::Greater),
            (_, _) => Some(Ordering::Equal),
        }
    }
}

impl<Coord : Coordinate> PartialEq<Tile<Coord>> for HorizontalLine<Coord> {

    fn eq(&self, rhs : &Tile<Coord>) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
    }
}

/// Returns `Equal` if the point is on the line, `Less` if it is below it 
///  `Greater` if the point is on top of the line
impl<Coord : Coordinate> PartialOrd<Point<Coord>> for HorizontalLine<Coord> {

    fn partial_cmp(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        Some(self.y.cmp(&rhs.y))

    }
}

impl<Coord : Coordinate> PartialEq<Point<Coord>> for HorizontalLine<Coord> {

    fn eq(&self, rhs : &Point<Coord>) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
    }
}

pub trait Line<Coord> : Debug {
    fn is_vertical(&self) -> bool {false }
    fn is_horizontal(&self) -> bool { false}

    // XXX work around bug #26339, use explicit methods instead of ordering traits to explicitly
    // dispatch to the right object. ( https://github.com/rust-lang/rust/issues/26339 )
    fn cmp_with_tile(&self, rhs : &Tile<Coord>) -> Option<Ordering>;
    fn cmp_with_point(&self, rhs : &Point<Coord>) -> Option<Ordering>;

}

impl<Coord : Coordinate> Line<Coord> for VerticalLine<Coord> {
    fn is_vertical(&self) -> bool { true }

    fn cmp_with_tile(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        self.partial_cmp(rhs)
    }

    fn cmp_with_point(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        self.partial_cmp(rhs)
    }

}
impl<Coord : Coordinate> Line<Coord> for HorizontalLine<Coord> {
    fn is_horizontal(&self) -> bool { true }

    fn cmp_with_tile(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        self.partial_cmp(rhs)
    }

    fn cmp_with_point(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        self.partial_cmp(rhs)
    }
}

impl<'a, Coord : Coordinate> PartialOrd<Point<Coord>> for (Line<Coord> + 'a) {

    fn partial_cmp(&self, rhs : &Point<Coord>) -> Option<Ordering> {
        self.cmp_with_point(rhs)
    }
}


impl<'a, Coord : Coordinate> PartialEq<Point<Coord>> for (Line<Coord> + 'a) {

    fn eq(&self, rhs : &Point<Coord>) -> bool {
        self.cmp_with_point(rhs) == Some(Ordering::Equal)
    }
}

impl<'a, Coord : Coordinate> PartialOrd<Tile<Coord>> for (Line<Coord> + 'a) {

    fn partial_cmp(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        self.cmp_with_tile(rhs)
    }
}


impl<'a, Coord : Coordinate> PartialEq<Tile<Coord>> for (Line<Coord> + 'a) {

    fn eq(&self, rhs : &Tile<Coord>) -> bool {
        self.cmp_with_tile(rhs) == Some(Ordering::Equal)
    }
}


/// A `Tile` represents a bounded rectangular area in a 2D plane.
#[derive(Debug, Clone, Copy)]
pub struct Tile<Coord : Coordinate> {
    bottom : Point<Coord>,
    top : Point<Coord>,
} 

impl<Coord : Coordinate> Tile<Coord> {

    /// Creates a new `Tile` given its bottom left corner and its top right corner
    ///
    /// # panic
    ///
    /// Panics if `bottom` is upper or right to `top`
    pub fn new(bottom : Point<Coord>, top : Point<Coord>) -> Tile<Coord> {
        assert!(bottom <= top);
        Tile {
            bottom : bottom,
            top : top
        }
    }

    /// Creates a new tile from a point. The tile is reduced to this point
    pub fn from_point(p : Point<Coord>) -> Tile<Coord> {
        Tile {
            top : p,
            bottom : p,
        }
    }

    /// Returns the bottom left corner
    pub fn bottom_left_corner(&self) -> Point<Coord> {
        self.bottom
    }
    

    /// Returns the to right corner
    pub fn top_right_corner(&self) -> Point<Coord> {
        self.top
    }

    /// Orders two tiles by their `x` coordinates.
    ///
    /// Returns :
    ///  - `Equal` if the two tiles `x` intervals are overlapping
    ///  - `Greater` or `Less` if the `x` interval of the first one is either strictly greater or smaller
    pub fn vertical_cmp(self, rhs : Tile<Coord>) -> Ordering {
        let order_left = self.bottom.x.cmp(&rhs.bottom.x);
        let order_right = self.top.x.cmp(&rhs.top.x);

        match (order_left, order_right) {
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            (_, _) => Ordering::Equal,
        }
    }


    /// Orders two tiles by their `y` coordinates.
    ///
    /// Returns :
    ///  - `Equal` if the two tiles `y` intervals are overlapping
    ///  - `Greater` or `Less` if the `y` interval of the first one is either strictly greater or smaller
    pub fn horizontal_cmp(self, rhs : Tile<Coord>) -> Ordering {
        let order_bottom = self.bottom.y.cmp(&rhs.bottom.y);
        let order_top = self.top.y.cmp(&rhs.top.y);

        match (order_bottom, order_top) {
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            (_, _) => Ordering::Equal,
        }
    }

    /// Returns the smallest tile including both tiles
    pub fn union(self, rhs : Tile<Coord>) -> Tile<Coord> {
        Tile {
            top : Point {
                x : max(self.top.x, rhs.top.x),
                y : max(self.top.y, rhs.top.y),
            },
            bottom : Point {
                x : min(self.bottom.x, rhs.bottom.x),
                y : min(self.bottom.y, rhs.bottom.y),
            }
        }
    }
}

/// Returns the smallest tile containing all the tiles from the iterator.
pub fn bounding_tile<Coord, I>(i : I) -> Option<Tile<Coord>> where Coord : Coordinate, I : Iterator<Item = Tile<Coord>> 
{
    i.fold(None, |acc, tile| acc.map(|t| t.union(tile)).or(Some(tile)))
}

/// A tile is equal to a point if it is reduced to this point
impl<Coord : Coordinate> PartialEq<Point<Coord>> for Tile<Coord> {

    fn eq(&self, rhs : &Point<Coord>) -> bool {
        &self.top == &self.bottom && &self.top == rhs 
    }
}

/// A tile is equal to a point if it is reduced to this point
impl<Coord : Coordinate> PartialEq<Tile<Coord>> for Point<Coord> {

    fn eq(&self, rhs : &Tile<Coord>) -> bool {
        &rhs.top == &rhs.bottom && &rhs.top == self 
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
            if rhs >= &self.bottom && rhs <= &self.top {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

impl<Coord : Coordinate> PartialOrd<Tile<Coord>> for Point<Coord> {

    fn partial_cmp(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        match rhs.partial_cmp(self) {
            Some(Ordering::Greater) => Some(Ordering::Less),
            Some(Ordering::Less) => Some(Ordering::Greater),
            ord => ord,
        }
    }
}


/// Order between tiles
/// A tile is greater than another if it contains it
/// A tile is smaller than another if it is contained by it
/// If tiles overlap, result is undefined
impl<Coord : Coordinate> PartialOrd for Tile<Coord> {

    fn partial_cmp(&self, rhs : &Tile<Coord>) -> Option<Ordering> {
        if self.bottom == rhs.bottom && self.top == rhs.top {
            Some(Ordering::Equal)
        } else if self.bottom >= rhs.bottom && self.top <= rhs.top {
            Some(Ordering::Less)
        } else if self.bottom <= rhs.bottom && self.top >= rhs.top {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

impl<Coord : Coordinate> PartialEq for Tile<Coord> {

    fn eq(&self, rhs : &Tile<Coord>) -> bool {
        self.partial_cmp(rhs) == Some(Ordering::Equal)
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
        assert!(p2 > p3);
        assert!(p2 > p4);
        assert!(p4 < p2);

        let p5 = Point::new(5, 2usize);
        assert!(p5.partial_cmp(&p4) == None);
    }

    #[test]
    fn tile_equality() {
        let p1 = Point::new(10, 10);
        let tile = Tile::new(Point::new(10usize, 10usize), Point::new(10, 10));
        let tile2 = Tile::new(Point::new(10usize, 10usize), Point::new(10, 11));
        let tile3 = Tile::new(Point::new(11usize, 10usize), Point::new(11, 10));

        assert!(tile == p1);
        assert!(tile2 != p1);
        assert!(tile3 != p1);
    }

    #[test]
    fn tile_ordering() {
        let p1 = Point::new(10, 10);

        let tile = Tile::new(Point::new(10usize, 10usize), Point::new(10, 10));
        assert!(tile <= p1);
        assert_eq!(tile < p1, false);

        let tile2 = Tile::new(Point::new(10usize, 10usize), Point::new(11, 11));
        assert!(tile2 >= p1);
        assert!(tile2 > Point::new(11, 11));

        let tile2 = Tile::new(Point::new(5usize, 5usize), Point::new(15, 15));
        assert!(tile2 > Point::new(5, 8));
        assert!(tile2 > Point::new(15, 10));
        assert!(tile2 > Point::new(15, 15));
        assert!(tile2 > Point::new(5, 5));
        assert!(tile2 < Point::new(7, 22));
        assert!(tile2 > Point::new(7, 11));
    }

    #[test]
    fn tile_vertical_cmp() {
        let t1 = Tile::new(Point::new(4usize, 5), Point::new(7, 8));

        let t2 = Tile::new(Point::new(7usize, 8), Point::new(9, 9));
        assert!(t1.vertical_cmp(t2) == Ordering::Less);
        assert!(t2.vertical_cmp(t1) == Ordering::Greater);

        let t2 = Tile::new(Point::new(4usize, 5), Point::new(6, 6));
        assert!(t1.vertical_cmp(t2) == Ordering::Equal);
        assert!(t2.vertical_cmp(t1) == Ordering::Equal);

        let t2 = Tile::new(Point::new(3usize, 5), Point::new(9, 6));
        assert!(t1.vertical_cmp(t2) == Ordering::Equal);
        assert!(t2.vertical_cmp(t1) == Ordering::Equal);

    }

    #[test]
    fn tile_union() {
        let t1 = Tile::new(Point::new(4usize, 5), Point::new(7, 8));
        let t2 = Tile::new(Point::new(4usize, 5), Point::new(6, 6));

        let t3 = t1.union(t2);
        assert!(t3 >= t1);
        assert!(t3 >= t2);
    }

    #[test]
    fn test_line_point_cmp(){
        let line : Box<Line<u16>> = Box::new(HorizontalLine::new(4) );
        assert!(&*line > &Point::new(0, 0));
    }
    

}

