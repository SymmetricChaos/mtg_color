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
 
    const SYMBOLS: [&'static str; 32] = ["","W","U","WU","B","WB","UB","WUB","R","RW","UR","URW","BR","RWB","UBR","WUBR","G","GW","GU","GWU","BG","WBG","BGU","GWUB","RG","RGW","GUR","RGWU","BRG","BRGW","UBRG","WUBRG"];
    // Minimal(?) string containing all symbols in canonical order
    //                                01234567890123
    // const HYPER_PERM: &'static str = "RWBGURWUBRGWUB";
    // ""
    // 1..2   W
    // 4..5   U
    // 6..8   WU
    // 2..3   B
    // 1..3   WB
    // 12..14 UB
    // 6..9   WUB
    //        R
    //        RW
    //        UR
    //        URW
    //        BR
    //        RWB
    //        UBR
    //        WUBR
    //        G
    //        GW
    //        GU
    //        GWU
    //        BG
    //        WBG
    //        BGU
    //        GWUB
    //        RG
    //        RGW
    //        GUR
    //        RGWU
    //        BRG
    //        BRGW
    //        UBRG
    //        WUBRG

    /// Symbols in canonical order
    /// To avoid panic handling only the lower 5 bits are considered.
    pub fn symbols(&self) -> &'static str {
        Self::SYMBOLS[self.color_bits as usize]
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
        Colors{ color_bits: value % 32 }
    }
}


#[test]
fn test_changes() {
    let mut c = Colors::from_symbols("WG");

    assert_eq!("GW",c.symbols());
    assert_eq!(true,c.is_white());
    assert_eq!(false,c.is_blue());
    assert_eq!(2,c.num_colors());
    assert_eq!(true,c.is_multicolor());
    assert_eq!(false,c.is_monocolor());
    assert_eq!(false,c.is_colorless());

    // Check double assign.
    c.add_blue();
    c.add_blue();

    assert_eq!("GWU",c.symbols());
    assert_eq!(true,c.is_white());
    assert_eq!(true,c.is_blue());
    assert_eq!(3,c.num_colors());
    assert_eq!(true,c.is_multicolor());
    assert_eq!(false,c.is_monocolor());
    assert_eq!(false,c.is_colorless());

    c.del_white();

    assert_eq!("GU",c.symbols());
    assert_eq!(false,c.is_white());
    assert_eq!(true,c.is_blue());
    assert_eq!(2,c.num_colors());
    assert_eq!(true,c.is_multicolor());
    assert_eq!(false,c.is_monocolor());
    assert_eq!(false,c.is_colorless());

    c.del_green();

    assert_eq!("U",c.symbols());
    assert_eq!(false,c.is_white());
    assert_eq!(true,c.is_blue());
    assert_eq!(1,c.num_colors());
    assert_eq!(false,c.is_multicolor());
    assert_eq!(true,c.is_monocolor());
    assert_eq!(false,c.is_colorless());
}
