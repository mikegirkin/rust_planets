use crate::util::path_in_test_directory;
use crate::model::game::{Game, Coords};

#[test]
pub fn test_game_read() {
    let path = path_in_test_directory("pleiades10");

    let game = Game::read(path);

    assert_eq!(game.planets[0].name.0, "Ceti Alpha one");
    assert_eq!(game.planets[0].position, Coords{ x: 1167, y: 1766 } );
    assert_eq!(game.planets[499].name.0, "Fantasia");
    assert_eq!(game.planets[499].position, Coords{  x: 2623, y: 2579 });
    assert_eq!(game.rules.engines[8].name, "Bistromatic Drive");

}