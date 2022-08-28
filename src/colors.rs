pub struct Colors {
    color_bits: u8,
}

impl Colors {
    pub fn from_symbols(s: &str) -> Colors {
        let mut c = Colors { color_bits: 0 };
        for ch in s.chars() {
            match ch {
                'W' => c.add_white(),
                'U' => c.add_blue(),
                'B' => c.add_black(),
                'R' => c.add_red(),
                'G' => c.add_green(),
                _ => (),
            }
        }
        c
    }

    // Add a color
    pub fn add_white(&mut self) {
        self.color_bits |= 1
    }

    pub fn add_blue(&mut self) {
        self.color_bits |= 2
    }

    pub fn add_black(&mut self) {
        self.color_bits |= 4
    }

    pub fn add_red(&mut self) {
        self.color_bits |= 8
    }

    pub fn add_green(&mut self) {
        self.color_bits |= 16
    }

    // remove colors
    pub fn del_white(&mut self) {
        self.color_bits &= !1
    }

    pub fn del_blue(&mut self) {
        self.color_bits &= !2
    }

    pub fn del_black(&mut self) {
        self.color_bits &= !4
    }

    pub fn del_red(&mut self) {
        self.color_bits &= !8
    }

    pub fn del_green(&mut self) {
        self.color_bits &= !16
    }

    pub fn set_colorless(&mut self) {
        self.color_bits = 0
    }

    // Predicates
    pub fn is_white(&self) -> bool {
        self.color_bits & 1 == 1
    }

    pub fn is_blue(&self) -> bool {
        self.color_bits & 2 == 2
    }

    pub fn is_black(&self) -> bool {
        self.color_bits & 4 == 4
    }

    pub fn is_red(&self) -> bool {
        self.color_bits & 8 == 8
    }

    pub fn is_green(&self) -> bool {
        self.color_bits & 16 == 16
    }

    pub fn is_colorless(&self) -> bool {
        self.color_bits == 0
    }

    pub fn is_monocolor(&self) -> bool {
        self.num_colors() == 1
    }

    pub fn is_multicolor(&self) -> bool {
        self.num_colors() > 1
    }

    // Descriptors
    pub fn num_colors(&self) -> u32 {
        self.color_bits.count_ones()
    }

    // Minimal(?) string containing all symbols in canonical order
    const HYPER_PERM: &'static str = "RWBGURWUBRGWUB";

    // Symbols in canonical order
    // To avoid panic handling only the lower 5 bits are considered.
    pub fn symbols(&self) -> &'static str {
        match self.color_bits % 32 {
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
            _ => unreachable!("there are only 32 color possibilities"),
        }
    }
}

// It is literally just a byte
impl Into<u8> for Colors {
    fn into(self) -> u8 {
        self.color_bits
    }
}

// Ignore the top three bits
impl From<u8> for Colors {
    fn from(value: u8) -> Colors {
        Colors {
            color_bits: value % 32,
        }
    }
}

#[test]
fn test_changes() {
    let mut c = Colors::from_symbols("WG");

    assert_eq!("GW", c.symbols());
    assert_eq!(true, c.is_white());
    assert_eq!(false, c.is_blue());
    assert_eq!(2, c.num_colors());
    assert_eq!(true, c.is_multicolor());
    assert_eq!(false, c.is_monocolor());
    assert_eq!(false, c.is_colorless());

    // Check double assign.
    c.add_blue();
    c.add_blue();

    assert_eq!("GWU", c.symbols());
    assert_eq!(true, c.is_white());
    assert_eq!(true, c.is_blue());
    assert_eq!(3, c.num_colors());
    assert_eq!(true, c.is_multicolor());
    assert_eq!(false, c.is_monocolor());
    assert_eq!(false, c.is_colorless());

    // Check double delete.
    c.del_white();
    c.del_white();

    assert_eq!("GU", c.symbols());
    assert_eq!(false, c.is_white());
    assert_eq!(true, c.is_blue());
    assert_eq!(2, c.num_colors());
    assert_eq!(true, c.is_multicolor());
    assert_eq!(false, c.is_monocolor());
    assert_eq!(false, c.is_colorless());

    c.del_green();

    assert_eq!("U", c.symbols());
    assert_eq!(false, c.is_white());
    assert_eq!(true, c.is_blue());
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
    for color_bits in 0..32 {
        let c = Colors::from(color_bits);
        assert_eq!(c.symbols(), symbols[color_bits as usize])
    }
}
