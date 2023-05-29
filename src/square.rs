use std::ops::{PartialEq};
use std::fmt;

pub struct Square {
    x: u8,
    y: u8,
}

impl Square {
    pub fn new(x: u8, y: u8) -> Result<Square, String> {
        if x < 8 && y < 8 {
            Ok(Square { x, y })
        } else {
            Err(format!("Square ({}, {}) out of bounds ([0..8], [0..8])", x, y))
        }
    }

    pub fn index(&self) -> u8 { self.y * 8 + self.x }

    pub fn x(&self) -> u8 { self.x }

    pub fn y(&self) -> u8 { self.y }
}

impl Square {
    pub fn from_index(index: u8) -> Result<Square, String> {
        if index < 64 {
            Ok(Square {
                x: index % 8,
                y: index / 8,
            })
        } else {
            Err(
                format!(
                    "Invalid square index: {} out of bounds [0..64]",
                    index
                )
            )
        }
    }

    pub fn from_string(s: &str) -> Result<Square, String> {
        if s.len() != 2 {
            return Err(format!("Square string '{}' not of length 2", s));
        }
        let mut chars = s.chars();
        let x = chars.next().unwrap() as u8 - 'a' as u8;
        let y = chars.next().unwrap() as u8 - '1' as u8;
        Square::new(x, y)
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Square) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (self.x + 'a' as u8) as char,
            (self.y + '1' as u8) as char
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn setup() {
        let sq1 = Square::new(0, 0);
        let sq2 = Square::new(7, 0);
        let sq3 = Square::new(0, 7);
        let sq4 = Square::new(7, 7);
    }

    #[test]
    fn new() {
        assert!(sq1.is_ok());
        assert!(sq2.is_ok());
        assert!(sq3.is_ok());
        assert!(sq4.is_ok());

        assert!(Square::new(255, 0).is_err());
        assert!(Square::new(0, 255).is_err());
        assert!(Square::new(255, 255).is_err());

        assert!(Square::new(8, 0).is_err());
        assert!(Square::new(0, 8).is_err());
        assert!(Square::new(8, 8).is_err());
    }

    #[test]
    fn index() {
        assert_eq!(sq1.unwrap().index(), 0);
        assert_eq!(sq2.unwrap().index(), 7);
        assert_eq!(sq3.unwrap().index(), 56);
        assert_eq!(sq4.unwrap().index(), 63);
    }

    #[test]
    fn x() {
        assert_eq!(sq1.unwrap().x(), 0);
        assert_eq!(sq2.unwrap().x(), 7);
        assert_eq!(sq3.unwrap().x(), 0);
        assert_eq!(sq4.unwrap().x(), 7);
    }

    #[test]
    fn y() {
        assert_eq!(sq1.unwrap().y(), 0);
        assert_eq!(sq2.unwrap().y(), 0);
        assert_eq!(sq3.unwrap().y(), 7);
        assert_eq!(sq4.unwrap().y(), 7);
    }

    #[test]
    fn from_index() {
        assert_eq!(Square::from_index(0), sq1);
        assert_eq!(Square::from_index(7), sq2);
        assert_eq!(Square::from_index(56), sq3);
        assert_eq!(Square::from_index(63), sq4);
    }

    #[test]
    fn from_string() {
        assert_eq!(Square::from_string("a1"), sq1);
        assert_eq!(Square::from_string("a8"), sq2);
        assert_eq!(Square::from_string("h1"), sq3);
        assert_eq!(Square::from_string("h8"), sq4);
    }

    #[test]
    fn eq() {
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
        assert_eq!(format!("{}", sq1), "a1");
        assert_eq!(format!("{}", sq2), "a8");
        assert_eq!(format!("{}", sq3), "h1");
        assert_eq!(format!("{}", sq4), "h8");
    }
}
