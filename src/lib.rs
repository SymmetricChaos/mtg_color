mod tests;

pub struct Colors {
    color_bits: u8,
}
 
impl Colors {

    pub fn from_symbols(s: &str) -> Colors {
        let mut c = Colors{ color_bits: 0 };
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
 
    // Symbols in canonical order
    fn symbols(&self) -> &'static str {
        match self.color_bits {
            0 => "",
            1 => "W",
            2 => "U",
            3 => "WU",
            4 => "B",
            5 => "WB",
            6 => "UB",
            7 => "WUB",
            8 => "R",
            9 => "RW",
            10 => "UR",
            11 => "URW",
            12 => "BR",
            13 => "RWB",
            14 => "UBR",
            15 => "WUBR",
            16 => "G",
            17 => "GW",
            18 => "GU",
            19 => "GWU",
            20 => "BG",
            21 => "WBG",
            22 => "BGU",
            23 => "GWUB",
            24 => "RG",
            25 => "RGW",
            26 => "GUR",
            27 => "RGWU",
            28 => "BRG",
            29 => "BRGW",
            30 => "UBRG",
            31 => "WUBRG",
            _ => unreachable!("invalid bits"),
        }
    }
}

// It is literally just a byte
impl Into<u8> for Colors {
    fn into(self) -> u8 {
        self.color_bits
    }
}

// It is literally just a byte
impl TryFrom<u8> for Colors {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 32 {
            Ok(Colors{color_bits:value})
        } else {
            Err("maximum valid value is 31")
        }
    }
}