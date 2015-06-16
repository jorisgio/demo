use std::io::{
    self,
    Read,
    BufRead,
    Write,
};
use std::iter::Peekable;
use std::error::Error;
use std::num;
use std::fmt::{
    self,
    Display,
};

use geometry::{
    Point,
};


/// A move instruction for the rover
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RoverMove {
    North,
    East,
    South,
    West,
}


impl RoverMove {

    /// Parses a rover move instruction from a `char`. 
    ///
    /// Valid instructions are either S, E, W, or N
    fn parse(c : char) -> Option<RoverMove> {
        match c {
            'N' => Some(RoverMove::North),
            'E' => Some(RoverMove::East),
            'S' => Some(RoverMove::South),
            'W' => Some(RoverMove::West),
            _ => None,
        }
    }
}

/// An error occuring while reading the game map
#[derive(Debug)]
pub enum ParseError {
    InvalidRoverPosition,
    InvalidDustPosition,
    InvalidMove,
    InvalidCoordinateFormat,
    InvalidNumber(num::ParseIntError),
    InputError(io::Error),
    UnexpectedEOF,
}

impl Error for ParseError {

    fn description(&self) -> &str {
        match *self {
            ParseError::InvalidRoverPosition => "initial rover position is outside the arena",
            ParseError::InvalidDustPosition => "dust is outside the arena",
            ParseError::InvalidMove => "invalid rover move instruction",
            ParseError::InvalidCoordinateFormat => "invalid coordinate line format",
            ParseError::InvalidNumber(_) => "invalid coordinate",
            ParseError::InputError(_) => "read error",
            ParseError::UnexpectedEOF => "unexpected end of file",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseError::InvalidNumber(ref e) => Some(e),
            ParseError::InputError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, fmt : &mut fmt::Formatter) -> fmt::Result {
        if let Some(cause) = self.cause() {
            write!(fmt, "{} ( {} )", self.description(), cause)
        } else {
            write!(fmt, "{}", self.description())
        }
    }
}


pub struct Parser<R : BufRead> {
    lines : Peekable<io::Lines<R>>,
    pos : usize,
}

impl<R : BufRead> Parser<R> {

    pub fn new(reader : R) -> Parser<R> {
        Parser {
            lines : reader.lines().peekable(),
            pos : 0,
        }
    }

    #[inline]
    pub fn line_number(&self) -> usize {
        self.pos 
    }


    /// Reads a coordinate tuple from the given stream
    fn parse_coordinate(&mut self) -> Result<(i32, i32), ParseError> {
        let line = 
            try!(
                try!(self.lines
                     .next()
                     .ok_or(ParseError::UnexpectedEOF))
                .map_err(|e| ParseError::InputError(e))
                );
        self.pos += 1;

        let words = line.split(|c : char| c.is_whitespace()).collect::<Vec<_>>();

        if words.len() != 2 {
            return Err(ParseError::InvalidCoordinateFormat)
        }

        let x = try!(words[1].parse::<u16>().map_err(|e| ParseError::InvalidNumber(e)));
        let y = try!(words[0].parse::<u16>().map_err(|e| ParseError::InvalidNumber(e)));
        Ok((x as i32 , y as i32))
    }

    /// Parses the dust coordinates from the given reader
    fn parse_dust(&mut self) -> Result<Vec<Point<i32>>, ParseError> {

        let mut vec = Vec::new();

        loop {
            // Grab the next line.
            {
                // XXX Ugliest code ever. TODO find a nicer hack.
                // Dirty hack to get the error instead of a reference to it since it's not
                // clonable (god knows why...). If the peeked value is an error, consume it
                // with .next() instead. This code makes me cry... This is really a bug in the stdlib,
                // there is no reason io::Error shouldn't be Clone
                //
                // We need to check that separatly in its own scope because we cannot call
                // .next() while holding a reference to the lines iterator, because of aliasing.
                let is_error = {
                    let peek_line = 
                        try!(self.lines
                             .peek()
                             .ok_or(ParseError::UnexpectedEOF));

                    match peek_line.as_ref() {
                        Err(_) => true,
                        _ => false
                    }
                };
                // If the next element of the iterator is an error, consumes it and returns the
                // error
                if is_error {
                    return self.lines.next().unwrap().err().map(|e| Err(ParseError::InputError(e))).unwrap()
                }
            }
            // get the first char of the next line. If it is not a digit, try parsing the rover
            // moves instead. Else continue parsing the dust map
            let first_char = 
            {
                // Get a reference to the next line (which is not an error)
                let line = { 
                    let peek_line = 
                        try!(self.lines
                             .peek()
                             .ok_or(ParseError::UnexpectedEOF));

                    match peek_line.as_ref() {
                        Err(_) => unreachable!(), // See comment above
                        Ok(l) => l,
                    }
                };
                try!(line.chars().take(1).next().ok_or(ParseError::InvalidCoordinateFormat))
            };

            // If first char is a digit, try parsing the line as a coordinate tuple
            if first_char.is_digit(10) {
                let (x, y) = try!(self.parse_coordinate());
                vec.push(Point::new(x, y));
            } else {
                // End of the dust data, start of the rover path
                return Ok(vec)
            }
        }
    }

    /// Parses the rover moves 
    fn parse_rover_path(&mut self) -> Result<Vec<RoverMove>, ParseError> { 
        let line = 
            try!(
                try!(self.lines
                     .next()
                     .ok_or(ParseError::UnexpectedEOF))
                .map_err(|e| ParseError::InputError(e))
                );
        self.pos += 1;

        let mut rover_moves_vector = Vec::new();
        for c in line.chars() {
            let rover_move = try!(RoverMove::parse(c).ok_or(ParseError::InvalidMove));
            rover_moves_vector.push(rover_move);
        }
        Ok(rover_moves_vector)
    }

    /// Parses the input data from the parser
    pub fn parse(&mut self) -> Result<(Point<i32>, Point<i32>, Vec<Point<i32>>, Vec<RoverMove>), ParseError> {
        // Read the grid size
        let (x, y) = try!(self.parse_coordinate());
        // Read the rover initial position
        let (rx, ry) = try!(self.parse_coordinate());

        let dust = try!(self.parse_dust());
        let moves = try!(self.parse_rover_path());

        Ok((Point::new(x, y), Point::new(rx, ry), dust, moves))
    }
}
