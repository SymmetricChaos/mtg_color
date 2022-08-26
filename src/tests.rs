
#[test]
fn test() {
    use crate::Colors;
    let mut c = Colors::from_symbols("WG");

    assert_eq!("GW",c.symbols());
    assert_eq!(true,c.is_white());
    assert_eq!(false,c.is_blue());
    assert_eq!(2,c.num_colors());
    assert_eq!(true,c.is_multicolor());
    assert_eq!(false,c.is_monocolor());
    assert_eq!(false,c.is_colorless());

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