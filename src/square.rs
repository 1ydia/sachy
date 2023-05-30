use std::fmt;

/// Represents an error that can occur when creating a `Square`.
/// 
/// The `SquareError` enum is used to represent an error that can occur when
/// creating a `Square` from an x and y coordinate, or from a string.
/// 
/// # Variants
/// 
/// - `XYOutOfBounds` - The x or y coordinate is out of bounds. This means that
/// either the x or y coordinate is greater than 7.
/// - `IndexOutOfBounds` - The index is out of bounds. This means that the index
/// is greater than 63.
/// - `InvalidString` - The string is not a valid square. This means that either
/// the string is not 2 characters long, the first character is not in the range
/// 'a' to 'h' or 'A' to 'H', or the second character is not in the range '1' to
/// '8'.
/// 
/// # Examples
/// 
/// ```rust
/// use sachy::square::{Square, SquareError};
/// 
/// let sq1 = Square::new(8, 0);
/// assert!(sq1.is_err());
/// assert_eq!(sq1.unwrap_err(), SquareError::XYOutOfBounds);
/// 
/// let sq2 = Square::new(0, 8);
/// assert!(sq2.is_err());
/// assert_eq!(sq2.unwrap_err(), SquareError::XYOutOfBounds);
/// 
/// let sq3 = Square::from_index(64);
/// assert!(sq3.is_err());
/// assert_eq!(sq3.unwrap_err(), SquareError::IndexOutOfBounds);
/// 
/// let sq4 = Square::from_string("a");
/// assert!(sq4.is_err());
/// assert_eq!(sq4.unwrap_err(), SquareError::InvalidString);
/// 
/// let sq5 = Square::from_string("a9");
/// assert!(sq5.is_err());
/// assert_eq!(sq5.unwrap_err(), SquareError::InvalidString);
/// 
/// let sq6 = Square::from_string("i1");
/// assert!(sq6.is_err());
/// assert_eq!(sq6.unwrap_err(), SquareError::InvalidString);
/// ```
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
pub enum SquareError {
    XYOutOfBounds,
    IndexOutOfBounds,
    InvalidString,
}

/// Represents a square on the chess board.
/// 
/// The `Square` struct is a 2-dimensional coordinate on the chess board. It
/// consists of an x and y coordinate, both of which are in the range 0 to 7
/// inclusive.
/// 
/// The x coordinate is the column, or 'file', and the y coordinate is the row,
/// or 'rank'.
/// 
/// # Implementation
/// 
/// The `Square` struct is implemented as a single `u8` value, with the x
/// coordinate in the upper 4 bits and the y coordinate in the lower 4 bits. The
/// `Square` struct is therefore 1 byte in size.
/// 
/// The `Square` struct implements the `Copy` trait, so it is cheap to copy.
/// All methods take `self` by value, so they consume the `Square` and return a
/// new one.
#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct Square {
    val: u8,
}

impl Square {
    /// Creates a new `Square` from the given x and y coordinates.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq = Square::new(0, 0).unwrap();
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an Err if the x or y coordinate is out of bounds.
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq1 = Square::new(8, 0);
    /// assert!(sq1.is_err());
    /// 
    /// let sq2 = Square::new(0, 8);
    /// assert!(sq2.is_err());
    /// ```
    /// 
    /// # Implementation
    /// 
    /// This function is implemented using bit shifting and masking. The x
    /// coordinate is shifted left 4 bits and the y coordinate is masked with
    /// 0b0000_1111. The two values are then combined with a bitwise OR to
    /// produce the final value, which is stored in the `Square` struct as two
    /// 'nibbles', in one u8.
    pub fn new(x: u8, y: u8) -> Result<Square, SquareError> {
        if x < 8 && y < 8 {
            Ok(Square { val: (x << 4) | y })
        } else {
            Err(SquareError::XYOutOfBounds)
        }
    }

    /// Returns the index of the `Square`.
    /// 
    /// The index is a number in the range 0 to 63 inclusive. It is the same as
    /// the index of the square in a one-dimensional array of squares, or the
    /// index of the square in a flattened 2d 8x8 array.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq1 = Square::new(0, 0).unwrap();
    /// assert_eq!(sq1.index(), 0);
    /// 
    /// let sq2 = Square::new(7, 7).unwrap();
    /// assert_eq!(sq2.index(), 63);
    /// ```
    /// 
    /// # Implementation
    /// 
    /// The index is calculated as `y * 8 + x`.
    pub fn index(&self) -> u8 { self.y() * 8 + self.x() }

    /// Returns the x coordinate of the `Square`.
    /// 
    /// The x coordinate is the column, or 'file', and is in the range 0 to 7
    /// inclusive.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq1 = Square::new(0, 0).unwrap();
    /// assert_eq!(sq1.x(), 0);
    /// 
    /// let sq2 = Square::new(7, 0).unwrap();
    /// assert_eq!(sq2.x(), 7);
    /// ```
    /// 
    /// # Implementation
    /// 
    /// The x coordinate is stored in the upper 4 bits of the `Square` value.
    /// This method shifts the value right 4 bits. This discards the lower 4 
    /// bits to get the x coordinate 'nibble'. The x coordinate is then returned
    /// as a u8.
    pub fn x(&self) -> u8 { self.val >> 4 }

    /// Returns the y coordinate of the `Square`.
    /// 
    /// The y coordinate is the row, or 'rank', and is in the range 0 to 7
    /// inclusive.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq1 = Square::new(0, 0).unwrap();
    /// assert_eq!(sq1.y(), 0);
    /// 
    /// let sq2 = Square::new(0, 7).unwrap();
    /// assert_eq!(sq2.y(), 7);
    /// ```
    /// 
    /// # Implementation
    /// 
    /// The y coordinate is stored in the lower 4 bits of the `Square` value.
    /// This method masks the value with 0b0000_1111 to get the y coordinate,
    /// discarding the upper 4 bits. The y coordinate is then returned as a u8.
    pub fn y(&self) -> u8 { self.val & 0b0000_1111 }

    /// Returns a `Square` from the given index.
    /// 
    /// The index is a number in the range 0 to 63 inclusive. It is the same as
    /// the index of the square in a one-dimensional array of squares, or the
    /// index of the square in a flattened 2d 8x8 array.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq1 = Square::from_index(0).unwrap();
    /// assert_eq!(sq1, Square::new(0, 0).unwrap());
    /// 
    /// let sq2 = Square::from_index(63).unwrap();
    /// assert_eq!(sq2, Square::new(7, 7).unwrap());
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an Err if the index is out of bounds.
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let square = Square::from_index(64);
    /// assert!(square.is_err());
    /// ```
    /// 
    /// # Implementation
    /// 
    /// The x coordinate is calculated as `index % 8` and the y coordinate is
    /// calculated as `index / 8`. This is the reverse of the calculation in
    /// `Square::index()`.
    pub fn from_index(index: u8) -> Result<Square, SquareError> {
        if index < 64 {
            Square::new(index % 8, index / 8)
        } else {
            Err(SquareError::IndexOutOfBounds)
        }
    }

    /// Returns a `Square` from a given string of algebraic notation.
    /// 
    /// The string must be 2 characters long. The first character must be a
    /// character from either 'a' to 'h' or 'A' to 'H' inclusive. The second
    /// character must be a digit from '1' to '8' inclusive.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let sq1 = Square::from_string("a1").unwrap();
    /// assert_eq!(sq1, Square::new(0, 0).unwrap());
    /// 
    /// let sq2 = Square::from_string("h8").unwrap();
    /// assert_eq!(sq2, Square::new(7, 7).unwrap());
    /// ```
    /// 
    /// # Errors
    /// 
    /// Returns an Err if either the string is not 2 characters long, if the
    /// first character is not a letter from 'a' to 'h' or 'A' to 'H' inclusive,
    /// or if the second character is not a digit from '1' to '8' inclusive.
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let square = Square::from_string("a");
    /// assert!(square.is_err());
    /// 
    /// let square = Square::from_string("a11");
    /// assert!(square.is_err());
    /// 
    /// let square = Square::from_string("i1");
    /// assert!(square.is_err());
    /// 
    /// let square = Square::from_string("a9");
    /// assert!(square.is_err());
    /// 
    /// let square = Square::from_string("a0");
    /// assert!(square.is_err());
    /// ```
    pub fn from_string(s: &str) -> Result<Square, SquareError> {
        if s.len() != 2 {
            return Err(SquareError::InvalidString);
        }

        let mut chars = s.chars();

        let file = chars.nth(0).ok_or(SquareError::InvalidString)?.to_ascii_lowercase();
        let rank = chars.nth(1).ok_or(SquareError::InvalidString)?;
    
        if file < 'a' || file > 'h' {
            return Err(SquareError::InvalidString);
        }
    
        if !rank.is_ascii_digit() || rank < '1' || rank > '8' {
            return Err(SquareError::InvalidString);
        }
    
        let file = file as u8 - 'a' as u8;
        let rank = rank.to_digit(10).ok_or(SquareError::InvalidString)? as u8 - 1;
        
        Square::new(file, rank)
    }    
}

impl fmt::Display for Square {
    /// Formats the `Square` for displaying.
    ///
    /// The display format is a 2-character string, representing the x and y
    /// coordinates of the `Square`. The x coordinate is represented as a letter
    /// from 'a' to 'h' and the y coordinate as a number from '1' to '8'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let square = Square::new(0, 0).unwrap();
    /// assert_eq!("a1", format!("{}", square));
    /// ```
    /// 
    /// ```rust
    /// use sachy::square::Square;
    /// 
    /// let square = Square::new(7, 7).unwrap();
    /// assert_eq!("h8", format!("{}", square));
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.x() + 'a' as u8) as char,
            (self.y() + '1' as u8) as char
        )
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Square at ({}, {})", self.x(), self.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (Square, Square, Square, Square) {
        let sq1 = Square::new(0, 0).unwrap();
        let sq2 = Square::new(7, 0).unwrap();
        let sq3 = Square::new(0, 7).unwrap();
        let sq4 = Square::new(7, 7).unwrap();

        (sq1, sq2, sq3, sq4)
    }

    #[test]
    fn new_ok() {
        assert!(Square::new(0, 0).is_ok());
        assert!(Square::new(7, 0).is_ok());
        assert!(Square::new(0, 7).is_ok());
        assert!(Square::new(7, 7).is_ok());
    }

    #[test]
    fn new_err() {
        let sq1 = Square::new(255, 0);
        assert!(sq1.is_err());
        assert_eq!(sq1.unwrap_err(), SquareError::XYOutOfBounds);

        let sq2 = Square::new(0, 255);
        assert!(sq2.is_err());
        assert_eq!(sq2.unwrap_err(), SquareError::XYOutOfBounds);

        let sq3 = Square::new(255, 255);
        assert!(sq3.is_err());
        assert_eq!(sq3.unwrap_err(), SquareError::XYOutOfBounds);

        let sq4 = Square::new(8, 0);
        assert!(sq4.is_err());
        assert_eq!(sq4.unwrap_err(), SquareError::XYOutOfBounds);

        let sq5 = Square::new(0, 8);
        assert!(sq5.is_err());
        assert_eq!(sq5.unwrap_err(), SquareError::XYOutOfBounds);
        
        let sq6 = Square::new(8, 8);
        assert!(sq6.is_err());
        assert_eq!(sq6.unwrap_err(), SquareError::XYOutOfBounds);
    }

    #[test]
    fn index() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(sq1.index(), 0);
        assert_eq!(sq2.index(), 7);
        assert_eq!(sq3.index(), 56);
        assert_eq!(sq4.index(), 63);
    }

    #[test]
    fn x() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(sq1.x(), 0);
        assert_eq!(sq2.x(), 7);
        assert_eq!(sq3.x(), 0);
        assert_eq!(sq4.x(), 7);
    }

    #[test]
    fn y() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(sq1.y(), 0);
        assert_eq!(sq2.y(), 0);
        assert_eq!(sq3.y(), 7);
        assert_eq!(sq4.y(), 7);
    }

    #[test]
    fn from_index() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(Square::from_index(0).unwrap(), sq1);
        assert_eq!(Square::from_index(7).unwrap(), sq2);
        assert_eq!(Square::from_index(56).unwrap(), sq3);
        assert_eq!(Square::from_index(63).unwrap(), sq4);
    }

    #[test]
    fn from_index_err() {
        let sq1 = Square::from_index(64);
        assert!(sq1.is_err());
        assert_eq!(sq1.unwrap_err(), SquareError::IndexOutOfBounds);
    }

    #[test]
    fn from_string() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(Square::from_string("a1").unwrap(), sq1);
        assert_eq!(Square::from_string("h1").unwrap(), sq2);
        assert_eq!(Square::from_string("a8").unwrap(), sq3);
        assert_eq!(Square::from_string("h8").unwrap(), sq4);

        assert_eq!(Square::from_string("A1").unwrap(), sq1);
        assert_eq!(Square::from_string("H1").unwrap(), sq2);
        assert_eq!(Square::from_string("A8").unwrap(), sq3);
        assert_eq!(Square::from_string("H8").unwrap(), sq4);
    }

    #[test]
    fn from_string_err() {
        let sq1 = Square::from_string("a9");
        assert!(sq1.is_err());
        assert_eq!(sq1.unwrap_err(), SquareError::InvalidString);

        let sq2 = Square::from_string("i1");
        assert!(sq2.is_err());
        assert_eq!(sq2.unwrap_err(), SquareError::InvalidString);

        let sq3 = Square::from_string("a0");
        assert!(sq3.is_err());
        assert_eq!(sq3.unwrap_err(), SquareError::InvalidString);

        let sq4 = Square::from_string("h9");
        assert!(sq4.is_err());
        assert_eq!(sq4.unwrap_err(), SquareError::InvalidString);

        let sq5 = Square::from_string("i0");
        assert!(sq5.is_err());
        assert_eq!(sq5.unwrap_err(), SquareError::InvalidString);

        let sq6 = Square::from_string("i9");
        assert!(sq6.is_err());
        assert_eq!(sq6.unwrap_err(), SquareError::InvalidString);

        let sq7 = Square::from_string("a");
        assert!(sq7.is_err());
        assert_eq!(sq7.unwrap_err(), SquareError::InvalidString);

        let sq8 = Square::from_string("1");
        assert!(sq8.is_err());
        assert_eq!(sq8.unwrap_err(), SquareError::InvalidString);

        let sq9 = Square::from_string("a11");
        assert!(sq9.is_err());
        assert_eq!(sq9.unwrap_err(), SquareError::InvalidString);

        let sq10 = Square::from_string("i11");
        assert!(sq10.is_err());
        assert_eq!(sq10.unwrap_err(), SquareError::InvalidString);

        let sq11 = Square::from_string("i11");
        assert!(sq11.is_err());
        assert_eq!(sq11.unwrap_err(), SquareError::InvalidString);

        let sq12 = Square::from_string("aa");
        assert!(sq12.is_err());
        assert_eq!(sq12.unwrap_err(), SquareError::InvalidString);

        let sq13 = Square::from_string("11");
        assert!(sq13.is_err());
        assert_eq!(sq13.unwrap_err(), SquareError::InvalidString);

        let sq14 = Square::from_string("aa11");
        assert!(sq14.is_err());
        assert_eq!(sq14.unwrap_err(), SquareError::InvalidString);
    }

    #[test]
    fn eq() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(sq1, sq1);
        assert_ne!(sq1, sq2);
        assert_ne!(sq1, sq3);
        assert_ne!(sq1, sq4);
        
        assert_ne!(sq2, sq1);
        assert_eq!(sq2, sq2);
        assert_ne!(sq2, sq3);
        assert_ne!(sq2, sq4);

        assert_ne!(sq3, sq1);
        assert_ne!(sq3, sq2);
        assert_eq!(sq3, sq3);
        assert_ne!(sq3, sq4);

        assert_ne!(sq4, sq1);
        assert_ne!(sq4, sq2);
        assert_ne!(sq4, sq3);
        assert_eq!(sq4, sq4);
    }

    #[test]
    fn display() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(format!("{}", sq1), "a1");
        assert_eq!(format!("{}", sq2), "h1");
        assert_eq!(format!("{}", sq3), "a8");
        assert_eq!(format!("{}", sq4), "h8");
    }

    #[test]
    fn debug() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(format!("{:?}", sq1), "Square at (0, 0)");
        assert_eq!(format!("{:?}", sq2), "Square at (7, 0)");
        assert_eq!(format!("{:?}", sq3), "Square at (0, 7)");
        assert_eq!(format!("{:?}", sq4), "Square at (7, 7)");
    }
}
