use std::fmt;

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
    pub fn new(x: u8, y: u8) -> Result<Square, &'static str> {
        if x < 8 && y < 8 {
            Ok(Square { val: (x << 4) | y })
        } else {
            Err("Square (x, y) out of bounds ([0, 7], [0, 7])")
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
    pub fn from_index(index: u8) -> Result<Square, &'static str> {
        if index < 64 {
            Square::new(index % 8, index / 8)
        } else {
            Err("Square index out of bounds [0..64]")
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
    pub fn from_string(s: &str) -> Result<Square, &'static str> {
        if s.len() != 2 {
            return Err("Input string must be 2 characters long");
        }
        
        let file = s
            .chars()
            .nth(0)
            .ok_or("Input string must be 2 characters long")?
            .to_ascii_lowercase();
        let rank = s.chars().nth(1).ok_or("Input string must be 2 characters long")?;
    
        if file < 'a' || file > 'h' {
            return Err("First character must be a letter from 'a' to 'h' or 'A' to 'H'");
        }
    
        if !rank.is_ascii_digit() || rank < '1' || rank > '8' {
            return Err("Second character must be a digit from '1' to '8'");
        }
    
        let file = file as u8 - 'a' as u8;
        let rank = rank.to_digit(10).ok_or("Failed to parse rank")? as u8 - 1;
        
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
        assert!(Square::new(255, 0).is_err());
        assert!(Square::new(0, 255).is_err());
        assert!(Square::new(255, 255).is_err());

        assert!(Square::new(8, 0).is_err());
        assert!(Square::new(0, 8).is_err());
        assert!(Square::new(8, 8).is_err());
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
        assert!(Square::from_index(64).is_err());
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
        assert!(Square::from_string("a9").is_err());
        assert!(Square::from_string("i1").is_err());
        assert!(Square::from_string("a0").is_err());
        assert!(Square::from_string("h9").is_err());
        assert!(Square::from_string("i0").is_err());
        assert!(Square::from_string("i9").is_err());
        assert!(Square::from_string("a").is_err());
        assert!(Square::from_string("1").is_err());
        assert!(Square::from_string("a11").is_err());
        assert!(Square::from_string("i11").is_err());
        assert!(Square::from_string("i11").is_err());
        assert!(Square::from_string("aa").is_err());
        assert!(Square::from_string("11").is_err());
        assert!(Square::from_string("aa11").is_err());
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
