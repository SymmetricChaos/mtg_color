#![no_std]

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MtgColor {
    White = 1,
    Blue = 2,
    Black = 4,
    Red = 8,
    Green = 16,
}

impl TryFrom<char> for MtgColor {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'W' => Ok(MtgColor::White),
            'U' => Ok(MtgColor::Blue),
            'B' => Ok(MtgColor::Black),
            'R' => Ok(MtgColor::Red),
            'G' => Ok(MtgColor::Green),
            _ => Err("invalid MtgColor symbol"),
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorSet {
    bits: u8,
}

impl ColorSet {
    /// Add a color
    pub fn add(&mut self, color: MtgColor) {
        self.bits |= color as u8;
    }

    /// Remove a color
    pub fn remove(&mut self, color: MtgColor) {
        self.bits &= !(color as u8);
    }

    /// Remove all colors
    pub fn set_colorless(&mut self) {
        self.bits = 0
    }

    /// Check if a color is included
    pub fn is_color(&self, color: MtgColor) -> bool {
        self.bits & (color as u8) == (color as u8)
    }

    /// Check if the ColorSet is a specific monocolor
    pub fn is_color_mono(&self, color: MtgColor) -> bool {
        self.is_monocolor() && self.is_color(color)
    }

    /// Do all the colors in this ColorSet appear in another ColorSet
    pub fn is_subset_of(&self, other: ColorSet) -> bool {
        let other_sym = other.symbols();
        for ch in self.symbols().chars() {
            if !other_sym.contains(ch) {
                return false;
            }
        }
        true
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

    // Minimal super-string containing all symbols in canonical order
    const SUPERSTRING: &'static str = "RWBGURWUBRGWUB";

    /// Returns the symbols representing this ColorSet in canonical order
    /// Only the lower 5 bits are considered in order to avoid panic handling.
    pub fn symbols(&self) -> &'static str {
        match self.bits & 0b11111 {
            0 => &Self::SUPERSTRING[0..0],
            1 => &Self::SUPERSTRING[1..2],
            2 => &Self::SUPERSTRING[4..5],
            3 => &Self::SUPERSTRING[6..8],
            4 => &Self::SUPERSTRING[2..3],
            5 => &Self::SUPERSTRING[1..3],
            6 => &Self::SUPERSTRING[12..14],
            7 => &Self::SUPERSTRING[6..9],
            8 => &Self::SUPERSTRING[0..1],
            9 => &Self::SUPERSTRING[0..2],
            10 => &Self::SUPERSTRING[4..6],
            11 => &Self::SUPERSTRING[4..7],
            12 => &Self::SUPERSTRING[8..10],
            13 => &Self::SUPERSTRING[0..3],
            14 => &Self::SUPERSTRING[7..10],
            15 => &Self::SUPERSTRING[6..10],
            16 => &Self::SUPERSTRING[3..4],
            17 => &Self::SUPERSTRING[10..12],
            18 => &Self::SUPERSTRING[3..5],
            19 => &Self::SUPERSTRING[10..13],
            20 => &Self::SUPERSTRING[2..4],
            21 => &Self::SUPERSTRING[1..4],
            22 => &Self::SUPERSTRING[2..5],
            23 => &Self::SUPERSTRING[10..14],
            24 => &Self::SUPERSTRING[9..11],
            25 => &Self::SUPERSTRING[9..12],
            26 => &Self::SUPERSTRING[3..6],
            27 => &Self::SUPERSTRING[9..13],
            28 => &Self::SUPERSTRING[8..11],
            29 => &Self::SUPERSTRING[8..12],
            30 => &Self::SUPERSTRING[7..11],
            31 => &Self::SUPERSTRING[6..11],
            _ => unreachable!("invalid ColorSet bits"),
        }
    }
}

impl FromIterator<char> for ColorSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut c = Self { bits: 0 };
        for i in iter {
            if let Ok(color) = i.try_into() {
                c.add(color);
            }
        }
        c
    }
}

impl FromIterator<MtgColor> for ColorSet {
    fn from_iter<T: IntoIterator<Item = MtgColor>>(iter: T) -> Self {
        let mut c = Self { bits: 0 };
        for i in iter {
            c.add(i);
        }
        c
    }
}

impl TryFrom<u8> for ColorSet {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 32 {
            Ok(ColorSet { bits: value })
        } else {
            Err("invalid ColorSet bits")
        }
    }
}

impl TryFrom<&str> for ColorSet {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.chars().map(MtgColor::try_from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_changes() {
        let mut c = ColorSet::try_from("WG").unwrap();

        assert_eq!("GW", c.symbols());
        assert_eq!(true, c.is_color(MtgColor::White));
        assert_eq!(false, c.is_color(MtgColor::Blue));
        assert_eq!(false, c.is_color_mono(MtgColor::Blue));
        assert_eq!(2, c.num_colors());
        assert_eq!(true, c.is_multicolor());
        assert_eq!(false, c.is_monocolor());
        assert_eq!(false, c.is_colorless());

        // Check double assign.
        c.add(MtgColor::Blue);
        c.add(MtgColor::Blue);

        assert_eq!("GWU", c.symbols());
        assert_eq!(true, c.is_color(MtgColor::White));
        assert_eq!(true, c.is_color(MtgColor::Blue));
        assert_eq!(false, c.is_color_mono(MtgColor::Blue));
        assert_eq!(3, c.num_colors());
        assert_eq!(true, c.is_multicolor());
        assert_eq!(false, c.is_monocolor());
        assert_eq!(false, c.is_colorless());

        // Check double delete.
        c.remove(MtgColor::White);
        c.remove(MtgColor::White);

        assert_eq!("GU", c.symbols());
        assert_eq!(false, c.is_color(MtgColor::White));
        assert_eq!(true, c.is_color(MtgColor::Blue));
        assert_eq!(false, c.is_color_mono(MtgColor::Blue));
        assert_eq!(2, c.num_colors());
        assert_eq!(true, c.is_multicolor());
        assert_eq!(false, c.is_monocolor());
        assert_eq!(false, c.is_colorless());

        c.remove(MtgColor::Green);

        assert_eq!("U", c.symbols());
        assert_eq!(false, c.is_color(MtgColor::White));
        assert_eq!(true, c.is_color(MtgColor::Blue));
        assert_eq!(true, c.is_color_mono(MtgColor::Blue));
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

    #[test]
    fn test_constructors() {
        let c0 = ColorSet::try_from("BUW").unwrap();
        let c1 = ColorSet::try_from(7).unwrap();
        assert_eq!(c0,c1);
        assert_eq!(c0.symbols(),"WUB");
    }


    #[test]
    fn test_constructor_failure() {
        let c0 = ColorSet::try_from("WURGBJ").is_err();
        let c1 = ColorSet::try_from(51).is_err();
        assert!(c0);
        assert!(c1);
    }

    #[test]
    fn test_set_colorless() {
        let mut blue: ColorSet = "B".try_into().unwrap();
        blue.set_colorless();
        assert_eq!(blue.symbols(), "");
        assert_eq!(blue.bits, 0);
    }

    #[test]
    fn test_from_iter() {
        let c: ColorSet = "GRBUW".chars().collect();
        assert_eq!(c.symbols(), "WUBRG");
    }
}