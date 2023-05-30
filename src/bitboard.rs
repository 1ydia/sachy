use std::convert::{From, TryInto};
use std::fmt;

use crate::square::Square;

struct Bitboard {
    bits: u64,
}

impl Bitboard {
    pub fn new() -> Bitboard {
        Bitboard { bits: 0 }
    }

    pub fn get(&self, sq: Square) -> bool {
        self.bits & (1 << sq.index()) != 0
    }

    pub fn set(&mut self, sq: Square) -> bool {
        let old = self.get(sq);
        self.bits |= 1 << sq.index();
        old
    }

    pub fn clear(&mut self, sq: Square) -> bool {
        let old = self.get(sq);
        self.bits &= !(1 << sq.index());
        old
    }

    pub fn put(&mut self, sq: Square, value: bool) -> bool {
        if value {
            self.set(sq)
        } else {
            self.clear(sq)
        }
    }

    pub fn count(&self) -> u32 {
        self.bits.count_ones()
    }

    pub fn any(&self) -> bool {
        self.bits != 0
    }

    pub fn none(&self) -> bool {
        self.bits == 0
    }
}

impl From<u64> for Bitboard {
    fn from(bits: u64) -> Bitboard {
        Bitboard { bits }
    }
}

impl From<Square> for Bitboard {
    fn from(sq: Square) -> Bitboard {
        Bitboard::from(1 << sq.index())
    }
}

impl TryInto<Square> for Bitboard {
    type Error = &'static str;

    fn try_into(self) -> Result<Square, &'static str> {
        if self.count() == 1 {
            Square::from_index(self.bits.trailing_zeros() as u8)
        } else {
            Err("Bitboard does not contain exactly one square")
        }
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let verbose_mode = f.alternate();
        let mut s = String::new();
        for i in 0..64 {
            if self.bits & (1 << i) != 0 {
                s.push('1');
            } else {
                s.push('0');
            }
            if i % 8 == 7 {
                s.push('\n');
            } else {
                s.push(' ');
            }
        }
        write!(f, "{}", s)
    }
}