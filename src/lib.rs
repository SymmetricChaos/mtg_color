#![no_std]

pub enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

pub struct ColorSet {
    bits: u8,
}

impl ColorSet {
    /// Construct a Colors struct from a string, ignoring symbols other than WUBRG
    /// Using TryFrom<&str> is now preferred as it returns Result when given invalid characters.
    pub fn from_symbols(s: &str) -> ColorSet {
        let mut c = ColorSet { bits: 0 };
        for ch in s.chars() {
            match ch {
                'W' => c.add(Color::White),
                'U' => c.add(Color::Blue),
                'B' => c.add(Color::Black),
                'R' => c.add(Color::Red),
                'G' => c.add(Color::Green),
                _ => (),
            }
        }
        c
    }

    /// Add a color
    pub fn add(&mut self, color: Color) {
        match color {
            Color::White => self.bits |= 1,
            Color::Blue => self.bits |= 2,
            Color::Black => self.bits |= 4,
            Color::Red => self.bits |= 8,
            Color::Green => self.bits |= 16,
        }
    }

    /// Remove a color
    pub fn remove(&mut self, color: Color) {
        match color {
            Color::White => self.bits &= !1,
            Color::Blue => self.bits &= !2,
            Color::Black => self.bits &= !4,
            Color::Red => self.bits &= !8,
            Color::Green => self.bits &= !16,
        }
    }

    /// Check if a color is included
    pub fn is_color(&self, color: Color) -> bool {
        match color {
            Color::White => self.bits & 1 == 1,
            Color::Blue => self.bits & 2 == 2,
            Color::Black => self.bits & 4 == 4,
            Color::Red => self.bits & 8 == 8,
            Color::Green => self.bits & 16 == 16,
        }
    }

    /// Check if the ColorSet is a specific monocolor
    pub fn is_color_mono(&self, color: Color) -> bool {
        if self.is_monocolor() {
            self.is_color(color)
        } else {
            false
        }
    }

    /// Do all the colors in this ColorSet appear in another ColorSet
    pub fn is_subset_of(&self, other: ColorSet) -> bool {
        let other_sym = other.symbols();
        for ch in self.symbols().chars() {
            if !other_sym.contains(ch) {
                return false
            }
        }
        true
    }

    /// Remove all colors
    pub fn set_colorless(&mut self) {
        self.bits = 0
    }

    /// Returns true if ColorSet has no colors
    pub fn is_colorless(&self) -> bool {
        self.bits == 0
    }

    /// Returns true if ColorSet is exactly one color
    pub fn is_monocolor(&self) -> bool {
        self.num_colors() == 1
    }

    /// Returns true if ColorSet is two or more colors
    pub fn is_multicolor(&self) -> bool {
        self.num_colors() > 1
    }

    /// Determine how many colors are in the ColorSet
    pub fn num_colors(&self) -> u32 {
        self.bits.count_ones()
    }

    // Minimal(?) string containing all symbols in canonical order
    const HYPER_PERM: &'static str = "RWBGURWUBRGWUB";

    /// Returns the symbols representing this Colors in canonical order
    /// To avoid panic handling only the lower 5 bits of the representation are considered
    pub fn symbols(&self) -> &'static str {
        match self.bits % 32 {
            0 => &Self::HYPER_PERM[0..0],
            1 => &Self::HYPER_PERM[1..2],
            2 => &Self::HYPER_PERM[4..5],
            3 => &Self::HYPER_PERM[6..8],
            4 => &Self::HYPER_PERM[2..3],
            5 => &Self::HYPER_PERM[1..3],
            6 => &Self::HYPER_PERM[12..14],
            7 => &Self::HYPER_PERM[6..9],
            8 => &Self::HYPER_PERM[0..1],
            9 => &Self::HYPER_PERM[0..2],
            10 => &Self::HYPER_PERM[4..6],
            11 => &Self::HYPER_PERM[4..7],
            12 => &Self::HYPER_PERM[8..10],
            13 => &Self::HYPER_PERM[0..3],
            14 => &Self::HYPER_PERM[7..10],
            15 => &Self::HYPER_PERM[6..10],
            16 => &Self::HYPER_PERM[3..4],
            17 => &Self::HYPER_PERM[10..12],
            18 => &Self::HYPER_PERM[3..5],
            19 => &Self::HYPER_PERM[10..13],
            20 => &Self::HYPER_PERM[2..4],
            21 => &Self::HYPER_PERM[1..4],
            22 => &Self::HYPER_PERM[2..5],
            23 => &Self::HYPER_PERM[10..14],
            24 => &Self::HYPER_PERM[9..11],
            25 => &Self::HYPER_PERM[9..12],
            26 => &Self::HYPER_PERM[3..6],
            27 => &Self::HYPER_PERM[9..13],
            28 => &Self::HYPER_PERM[8..11],
            29 => &Self::HYPER_PERM[8..12],
            30 => &Self::HYPER_PERM[7..11],
            31 => &Self::HYPER_PERM[6..11],
            _ => unreachable!("invalid ColorSet bits"),
        }
    }
}

impl TryFrom<u8> for ColorSet {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 32 {
            Ok(ColorSet {
                bits: value % 32,
            })
        } else {
            Err("invalid ColorSet bits")
        }
    }
}

impl TryFrom<&str> for ColorSet {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut c = ColorSet { bits: 0 };
        for ch in value.chars() {
            match ch {
                'W' => c.add(Color::White),
                'U' => c.add(Color::Blue),
                'B' => c.add(Color::Black),
                'R' => c.add(Color::Red),
                'G' => c.add(Color::Green),
                _ => return Err("invalid ColorSet symbol"),
            }
        }
        Ok(c)
    }
}



#[test]
fn test_changes() {
    let mut c = ColorSet::try_from("WG").unwrap();

    assert_eq!("GW", c.symbols());
    assert_eq!(true, c.is_color(Color::White));
    assert_eq!(false, c.is_color(Color::Blue));
    assert_eq!(false, c.is_color_mono(Color::Blue));
    assert_eq!(2, c.num_colors());
    assert_eq!(true, c.is_multicolor());
    assert_eq!(false, c.is_monocolor());
    assert_eq!(false, c.is_colorless());

    // Check double assign.
    c.add(Color::Blue);
    c.add(Color::Blue);

    assert_eq!("GWU", c.symbols());
    assert_eq!(true, c.is_color(Color::White));
    assert_eq!(true, c.is_color(Color::Blue));
    assert_eq!(false, c.is_color_mono(Color::Blue));
    assert_eq!(3, c.num_colors());
    assert_eq!(true, c.is_multicolor());
    assert_eq!(false, c.is_monocolor());
    assert_eq!(false, c.is_colorless());

    // Check double delete.
    c.remove(Color::White);
    c.remove(Color::White);

    assert_eq!("GU", c.symbols());
    assert_eq!(false, c.is_color(Color::White));
    assert_eq!(true, c.is_color(Color::Blue));
    assert_eq!(false, c.is_color_mono(Color::Blue));
    assert_eq!(2, c.num_colors());
    assert_eq!(true, c.is_multicolor());
    assert_eq!(false, c.is_monocolor());
    assert_eq!(false, c.is_colorless());

    c.remove(Color::Green);

    assert_eq!("U", c.symbols());
    assert_eq!(false, c.is_color(Color::White));
    assert_eq!(true, c.is_color(Color::Blue));
    assert_eq!(true, c.is_color_mono(Color::Blue));
    assert_eq!(1, c.num_colors());
    assert_eq!(false, c.is_multicolor());
    assert_eq!(true, c.is_monocolor());
    assert_eq!(false, c.is_colorless());
}

#[test]
fn test_symbols() {
    let symbols = [
        "", "W", "U", "WU", "B", "WB", "UB", "WUB", "R", "RW", "UR", "URW", "BR", "RWB", "UBR",
        "WUBR", "G", "GW", "GU", "GWU", "BG", "WBG", "BGU", "GWUB", "RG", "RGW", "GUR", "RGWU",
        "BRG", "BRGW", "UBRG", "WUBRG",
    ];
    for color_bits in 0..32_u8 {
        let c = ColorSet::try_from(color_bits).unwrap();
        assert_eq!(c.symbols(), symbols[color_bits as usize])
    }
}

