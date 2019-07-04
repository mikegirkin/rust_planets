use super::*;
use super::super::util::*;

#[test]
fn test_reading_ships() {
    let path = path_in_test_directory("pleiades10/001/player2.rst");
    let rst = read_rst(&path);

    println!("{:?}", rst.ships);

    assert_eq!(rst.ships.len(), 2);
    assert_eq!(rst.ships[0].name, "HANSA CLASS LARGE TR");
    assert_eq!(rst.ships[0].id, 106);
    assert_eq!(rst.ships[0].engine_id, 9);
    assert_eq!(rst.ships[0].money, 0);

    assert_eq!(rst.ships[1].name, "HANSA CLASS LARGE TR");
    assert_eq!(rst.ships[1].id, 407);
    assert_eq!(rst.ships[1].engine_id, 9);
    assert_eq!(rst.ships[1].money, 0);
}