use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    x: u8,
    y: u8,
}

impl Square {
    pub fn new(x: u8, y: u8) -> Result<Square, &'static str> {
        if x < 8 && y < 8 {
            Ok(Square { x, y })
        } else {
            Err("Square out of bounds [0..8]")
        }
    }

    pub fn index(&self) -> u8 { self.y * 8 + self.x }

    pub fn x(&self) -> u8 { self.x }

    pub fn y(&self) -> u8 { self.y }
}

impl Square {
    pub fn from_index(index: u8) -> Result<Square, &'static str> {
        if index < 64 {
            Ok(Square {
                x: index % 8,
                y: index / 8,
            })
        } else {
            Err("Square index out of bounds [0..64]")
        }
    }

    pub fn from_string(s: &str) -> Result<Square, &'static str> {
        if s.len() != 2 {
            return Err("Square must be 2 characters long");
        }
        let mut chars = s.chars();
        let x = chars.next().unwrap() as u8 - 'a' as u8;
        let y = chars.next().unwrap() as u8 - '1' as u8;
        Square::new(x, y)
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
    fn from_string() {
        let (sq1, sq2, sq3, sq4) = setup();

        assert_eq!(Square::from_string("a1").unwrap(), sq1);
        assert_eq!(Square::from_string("a8").unwrap(), sq2);
        assert_eq!(Square::from_string("h1").unwrap(), sq3);
        assert_eq!(Square::from_string("h8").unwrap(), sq4);
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
        assert_eq!(format!("{}", sq2), "a8");
        assert_eq!(format!("{}", sq3), "h1");
        assert_eq!(format!("{}", sq4), "h8");
    }
}
