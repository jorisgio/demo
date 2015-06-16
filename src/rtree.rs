use std::mem;
use std::cmp::{
    PartialEq,
    PartialOrd,
    Ordering,
};

use std::clone::Clone;

use ::geometry::{
    Coordinate,
    Point,
    Tile,
    Line,
    VerticalLine,
    HorizontalLine,
    bounding_tile,
};

/// A subnode entry is either an internal tree node, or a leaf node with a point and associated
/// data
#[derive(Debug)]
enum Node<Coord : Coordinate, Value> {
    Leaf {
        point : Point<Coord>,
        data : Value,
    },
    Node {
        coverage : Tile<Coord>,
        vector : Vec<Node<Coord, Value>>,
    }
}

impl<Coord : Coordinate, Value, Obj : ?Sized> PartialOrd<Obj> for Node<Coord, Value>
where Obj : PartialOrd<Point<Coord>>, Obj : PartialOrd<Tile<Coord>>, Node<Coord, Value> : PartialEq<Obj> {

    #[inline]
    fn partial_cmp(&self, rhs : &Obj) -> Option<Ordering> {
        match *self {
            Node::Leaf { point : ref p, .. } => rhs.partial_cmp(&Tile::from_point(*p)),
            Node::Node { coverage : ref tile, .. } => rhs.partial_cmp(tile),
        }
        .map(|o| match o { Ordering::Less => Ordering::Greater, Ordering::Greater => Ordering::Less, o => o, })
    }
}

impl<Coord : Coordinate, Value, Obj : ?Sized> PartialEq<Obj> for Node<Coord, Value>
where
Obj : PartialEq<Point<Coord>>, 
Obj : PartialEq<Tile<Coord>>,
{

    fn eq(&self, rhs : &Obj) -> bool {
        match *self {
            Node::Leaf { point : ref p, .. } => rhs.eq(p),
            Node::Node { coverage : ref tile, .. } => rhs.eq(tile),
        }
    }
}

impl<Coord : Coordinate, Value> Node<Coord, Value> {

    /// Returns the smallest covering tile for the subtree
    fn coverage(&self) -> Tile<Coord> {
        match *self {
            Node::Leaf { point : ref p, .. } => Tile::from_point(*p),
            Node::Node { coverage : ref tile, .. } => *tile,
        }
    }

    /// Computes a partition of a tile vector into two subvectors. Put at most `fill_factor` elements into the first
    /// vector and the remaining elements into the second node. Returns the splitting line
    ///
    /// # Panics 
    ///
    /// Panics when the node has less than `fill_factor` elements or if fill_factor is less than 2
    fn sweep(tile_set : Vec<Tile<Coord>>, fill_factor : usize) -> Box<Line<Coord>> {

        // Use a greedy algorithm. Completly fill the first node and put the remaining elements in
        // the second node.
        // The box can be splitted either vertically or horizontally.         
        // Compute the number of tiles to be splited for each case and chose the case which leads to
        // the minimal number of split to minimize the height of the tree

        // Clone the tile sets two times to compute the area for both case without cloning the
        // whole subtrees.
        let mut tile_set_vertical = tile_set; 
        let mut tile_set_horizontal = tile_set_vertical.clone();


        // Sort by x axis starting points
        tile_set_vertical.sort_by(|n1, n2| n1.bottom_left_corner().vertical_cmp(n2.bottom_left_corner()));
        let iter = tile_set_vertical.iter();

        // Compute the vertical splitting line
        let vline =
            VerticalLine::at_point(tile_set_vertical[fill_factor].bottom_left_corner());

        // Compute the number of tiles which have to be splited (overlapping the first box)
        let vertical_cost = iter.filter(|&tile| &vline == tile).count();



        // Sort by y axis starting points
        tile_set_horizontal.sort_by(|n1, n2| n1.bottom_left_corner().horizontal_cmp(n2.bottom_left_corner()));
        let iter = tile_set_horizontal.iter();

        // Compute the spliting line
        let hline = 
            HorizontalLine::at_point(tile_set_horizontal[fill_factor].bottom_left_corner());

        // Compute the number of tiles which have to be splited (overlapping the first box)
        let horizontal_cost = iter.filter(|&tile| &hline == tile).count();



        // Decide wether the bounding box should be splitted vertically or horizontally
        if vertical_cost > horizontal_cost {
            Box::new(hline)
        } else {
            Box::new(vline)
        }
    }

    /// Recursivly splits a subtree into two. All subsubtree contained by the `left` tile go to the left subtree. 
    /// Subsubtrees overlapping the `left` tile are splitted. Other subtrees go to the right subtree. 
    ///
    /// Returns the right subtree.
    ///
    fn partition<'a>(&mut self, line : &'a (Line<Coord> + 'a), fill_factor : usize) -> Option<Node<Coord, Value>> {
        let (left, right) =
            match *self {
                Node::Leaf { .. } => return None,
                Node::Node { ref mut vector, .. } => 
                {
                    let mut left_vec = Vec::with_capacity(fill_factor);
                    let mut right_vec = Vec::with_capacity(fill_factor);

                    for mut node in (vector.drain(..)) {
                        match node.partial_cmp(line) {
                            Some(Ordering::Less) => left_vec.push(node),
                            // The current node overlaps the split tile. Recursivly split the subtree
                            Some(Ordering::Equal) => 
                            {
                                let right_subnode = node.partition(line, fill_factor);
                                left_vec.push(node);
                                if let Some(subnode) = right_subnode {
                                    right_vec.push(subnode);
                                }
                            },
                            _ => right_vec.push(node),
                        }
                    }
                    let left_box = bounding_tile(left_vec.iter().map(|n| n.coverage())).unwrap();
                    let right_node =
                        if right_vec.len() == 1 {
                            Some(right_vec.remove(0))
                        } else {
                            let right_box = bounding_tile(right_vec.iter().map(|n| n.coverage()));
                            right_box.map(|tile| Node::Node { coverage : tile, vector: right_vec })
                        };
                    (Node::Node { coverage : left_box, vector : left_vec}, right_node)
                }
            };

        *self = left;

        return right
    }


    /// If the node children count is greater than the fill factor, split it and returns the new
    /// node
    fn split_node(&mut self, fill_factor : usize) -> Option<Node<Coord, Value>> {
        let line = 
            match *self {
                Node::Leaf { .. } => return None,
                Node::Node { ref mut vector, .. } => 
                    if { vector.len() <= fill_factor } {
                        return None
                    } else {
                        // The node is full and needs to be splitted
                        let tile_set = vector.iter().map(|node| node.coverage()).collect::<Vec<_>>(); 
                        // Compute the split line
                        Node::<Coord, Value>::sweep(tile_set, fill_factor)
                    }
            };
        // Recursivly split the subtree. This call needs to be moved out because self cannot be
        // borrowed more than once at a time (borrowed at ref mut vector)
        self.partition(&*line, fill_factor)
    }

    /// Recursivly inserts a `Point` into the subtree. If a binding for this point already exists
    /// in the tree, its value is replaced and the old value is returned. Else, `None` is returned.
    ///
    /// The method also returns the overflow subtree which has to be added to the upper level
    fn insert(&mut self, point : Point<Coord>, mut value : Value, fill_factor : usize) -> (Option<Value>, Option<Node<Coord, Value>>) {
        // The call graph of this function is weird, but again it's for lifetime 
        let old_value = match *self {
            // Node is a leaf, swap the value and returns the old one
            Node::Leaf { ref mut data, .. } => { mem::swap(data, &mut value); return (Some(value), None) },
            Node::Node { ref mut vector, .. } =>
            {
                let (old_value, overflow) = 
                {
                    // Node is an internal node, recursivly call insert if a subtree contains the point
                    // to insert.
                    let subnode = 
                        vector
                        .iter_mut()
                        .find(|entry| **entry >= point);
                    if let Some(child) = subnode {
                        let (val, node) = child.insert(point, value, fill_factor);
                        (val, node)
                    } else { 
                        (None, Some(Node::Leaf { data : value, point : point }))
                    }
                };
                if let Some(node) = overflow {
                    // No subtree contains the point, insert it at the current level
                    vector.push(node);
                    old_value
                } else {
                    return (old_value, None)
                }
            }
        };
        (old_value, self.split_node(fill_factor))
    }

    /// Recursivly search for a matching point, returns `None` if no point is found or a
    /// reference to the value associated with the point
    fn find(&self, point : Point<Coord>) -> Option<&Value> {
        match *self {
            Node::Leaf { ref data, .. } => Some(data),
            Node::Node { ref vector, .. } => 
                vector
                .iter()
                .find(|entry| **entry >= point)
                .and_then(|node| node.find(point)),
        }
    }

    /// Recursivly search for a matching point, returns `None` if no point is found or a mutable
    /// reference to the value associated with the point
    fn find_mut(&mut self, point : Point<Coord>) -> Option<&mut Value> {
        match *self {
            Node::Leaf { ref mut data, .. } => Some(data),
            Node::Node { ref mut vector, .. } => 
                vector
                .iter_mut()
                .find(|entry| **entry >= point)
                .and_then(|node| node.find_mut(point)),
        }
    }
}

/// A balanced tree storing points in a 2D plane
#[derive(Debug)]
pub struct RTree<Coord : Coordinate, Data> {
    fill_factor : usize,
    root : Option<Node<Coord, Data>>,
}

impl<Coord : Coordinate, Data> RTree<Coord, Data> {

    /// Creates a new empty `RTree`
    pub fn new() -> RTree<Coord, Data> {
        RTree {
            fill_factor : 4,
            root : None,
        }
    }

    /// Creates a new empty `RTree` covering the given tile with an user defined radix
    pub fn with_radix(radix : usize) -> RTree<Coord, Data> {
        RTree {
            fill_factor : radix,
            root : None,
        }
    }

    /// Inserts a point into the RTree, and returns the old value associated to this point.
    ///
    pub fn insert(&mut self, point : Point<Coord>, data : Data) -> Option<Data> {
        if let Some(mut root) = self.root.take() {
            let (ret_val, overflow) = root.insert(point, data, self.fill_factor);
            if let Some(n) = overflow {
                // Replace the root
                let mut vector = Vec::with_capacity(self.fill_factor);
                let tile = n.coverage().union(root.coverage()); 
                vector.push(n);
                vector.push(root);

                self.root = Some(Node::Node { coverage : tile, vector : vector });
            }
            ret_val
        } else {
            self.root = Some(Node::Leaf { point : point, data : data }); None
        }
    }
    /// Recursivly search for a matching point, returns `None` if no point is found or a mutable
    /// reference to the value associated with the point
    #[inline]
    pub fn find_mut(&mut self, point : Point<Coord>) -> Option<&mut Data> {
        self.root.as_mut().and_then(|r| r.find_mut(point))
    }

    /// Recursivly search for a matching point, returns `None` if no point is found or a
    /// reference to the value associated with the point
    #[inline]
    pub fn find(&self, point : Point<Coord>) -> Option<&Data> {
        self.root.as_ref().and_then(|r| r.find(point))
    }

}

// Tests
#[test]
#[should_panic="index out of bounds"]
fn test_sweep_empty() {
    let vec = Vec::new();
    Node::<u16, u16>::sweep(vec, 0);
}

#[test]
#[should_panic="index out of bounds"]
fn test_sweep_fill_factor_empty() {
    let vec = vec![Tile::new(Point::new(0u16, 0), Point::new(4, 3))];
    Node::<u16, u16>::sweep(vec, 4);
}

#[test]
fn test_sweep_horizontal() {
    let vec =
        vec![Tile::new(Point::new(0, 0), Point::new(2, 2)),
        Tile::new(Point::new(1, 5), Point::new(8, 6)),
        Tile::new(Point::new(4, 10), Point::new(7, 11)),
        ];

    let line = Node::<u16, u16>::sweep(vec.clone(), 2);
    assert!(line.is_horizontal());
    assert!(*line > vec[0].bottom_left_corner());
    assert!(*line > vec[1].bottom_left_corner());
    assert!(*line <= vec[2].bottom_left_corner());
}

#[test]
fn test_sweep_vertical() {
    let vec =
        vec![Tile::new(Point::new(0, 0), Point::new(2, 2)),
        Tile::new(Point::new(1, 5), Point::new(3, 6)),
        Tile::new(Point::new(4, 4), Point::new(7, 11)),
        ];

    let line = Node::<u16, u16>::sweep(vec.clone(), 2);
    assert!(line.is_vertical());
    println!("{:?}, {:?}", line, vec[0].bottom_left_corner());
    assert!(*line > vec[0].bottom_left_corner());
    assert!(*line > vec[1].bottom_left_corner());
    assert!(*line <= vec[2].bottom_left_corner());
}

#[test]
fn test_partition_depth1_vertical() {

    let mut vector = Vec::with_capacity(5);

    vector.push(Node::Leaf{ point : Point::new(0, 0),data : ()});
    vector.push(Node::Leaf{ point : Point::new(5, 9),data : ()});
    vector.push(Node::Leaf{ point : Point::new(6, 0),data : ()});
    vector.push(Node::Leaf{ point : Point::new(3, 10), data : ()});
    vector.push(Node::Leaf{ point : Point::new(5, 5), data : ()});

    let mut node = Node::Node {
        coverage : Tile::new(Point::new(0, 0), Point::new(10, 10)),
        vector : vector,
    };

    let line = Box::new(VerticalLine::new(6u16));

    let ret = node.partition(&*line as &Line<u16>, 4);

    assert_eq!(ret, Some(Node::Leaf{ point : Point::new(6, 0), data : () }));

    match node { 
        Node::Node { 
            ref coverage,
            ref vector,
        } =>
        {
            assert!(vector.len() == 4);
            assert!(vector.iter().all(|n| n < &*line))
        }

        _ => panic!("Node became a leaf"),
    }

}

#[test]
fn test_partition_depth1_horizontal() {

    let mut vector = Vec::with_capacity(5);

    vector.push(Node::Leaf{ point : Point::new(0, 0),data : ()});
    vector.push(Node::Leaf{ point : Point::new(5, 9),data : ()});
    vector.push(Node::Leaf{ point : Point::new(6, 0),data : ()});
    vector.push(Node::Leaf{ point : Point::new(3, 10), data : ()});
    vector.push(Node::Leaf{ point : Point::new(5, 5), data : ()});

    let mut node = Node::Node {
        coverage : Tile::new(Point::new(0, 0), Point::new(10, 10)),
        vector : vector,
    };

    let line = Box::new(HorizontalLine::new(10u16));

    let ret = node.partition(&*line as &Line<u16>, 4);

    assert_eq!(ret, Some(Node::Leaf{ point : Point::new(3, 10), data : () }));

    match node { 
        Node::Node { 
            ref coverage,
            ref vector,
        } =>
        {
            assert!(vector.len() == 4);
            assert!(vector.iter().all(|n| n < &*line))
        }

        _ => panic!("Node became a leaf"),
    }

}

#[test]
fn insert() {

    let mut rtree = RTree::<u16, ()>::new();

    rtree.insert(Point::new(1, 1), ());
    rtree.insert(Point::new(1, 2), ());
    rtree.insert(Point::new(1, 4), ());
    rtree.insert(Point::new(3, 4), ());
    rtree.insert(Point::new(4, 4), ());
    rtree.insert(Point::new(10, 10), ());
    rtree.insert(Point::new(9, 10), ());
    rtree.insert(Point::new(1, 10), ());
    rtree.insert(Point::new(8, 6), ());
    rtree.insert(Point::new(0, 10), ());
    rtree.insert(Point::new(3, 7), ());

    assert!(rtree.find(Point::new(1, 1)).is_some());
    assert!(rtree.find(Point::new(1, 2)).is_some());
    assert!(rtree.find(Point::new(1, 4)).is_some());
    assert!(rtree.find(Point::new(3, 4)).is_some());
    assert!(rtree.find(Point::new(4, 4)).is_some());
    assert!(rtree.find(Point::new(10, 10)).is_some());
    assert!(rtree.find(Point::new(9, 10)).is_some());
    assert!(rtree.find(Point::new(1, 10)).is_some());
    assert!(rtree.find(Point::new(8, 6)).is_some());
    assert!(rtree.find(Point::new(0, 10)).is_some());
    assert!(rtree.find(Point::new(3, 7)).is_some());

}
