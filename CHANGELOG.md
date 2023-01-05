# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.9.0] - 2023-01-05

### Added

- View Field for player, he have a sight of 8 tiles
- View Field for monsters, they have a sight of 6 tiles

### Changed

- Monster can pursue the player only he can see him
- The map isn't discovered by default, the player can only know if he has already discovered a title 

## [0.8.1] - 2023-01-04

### Fixed

- Only a single monster spawn at the center of the room
- Screen size is fixed this a lower size
- HUD interface size is fixed, this with the HP bar and the tooltips are now functional

## [0.8.0] - 2023-01-03

### Added

- Game over screen at player death
- Victory screen at player victory
- Amulet of Yala is now spawn on the map
- If the player finds the Amulet of Yala than a won

### Fixed

- Now the game don't crash at player defeat

## [0.7.0] - 2023-01-02

### Added

- Chasing system for monsters. Now the player is hunted be a horde of monsters. Currently, the used algorithm is Dijkstra pathfinder.

### Changed

- Player have now only 10 HPs to increase difficulty

## [0.6.0] - 2023-01-02

### Added

- Combat system for the player. Now when the player moves to a tile and an enemy is here then the player initiates an attack. The damages are fixed to 1 HP
- Combat system is implemented for monster to. Monster cannot attack each over. The damages are fixed to 1 HP

### Changed

- Now Waiting allows the player to heal himself of 1 HP 

## [0.5.0] - 2022-12-31

### Added

- Hud interface for player HP
- With Hud interface the enemies informations can be showed if the player move it's cursor on a monster

### Changed

- Monster spawner rework to add informations to monster and increase readability
- Spawn Rate is now to: 50% for goblins, 30% for Orc, 15% for Ogre and 5% for Ellin

## [0.4.0] - 2022-11-01

### Added

- Enemies random move

### Changed

- The moves are now handled by a message system to decrease code complexity

## [0.3.0] - 2022-10-31

### Added

- 4 kinds of monsters
- A single random monster is created in each rooms
- The collision of a player and a monster resuls in the death of the monster

## [0.2.0] - 2022-10-31

### Changed

- Migrate project architecture into an ECS one to handle the increased of the complexity of the project

## [0.1.0] - 2022-10-29

### Added

- **BTerm** GameState implementation
- **BTerm** Main Loop implementation
- **BTerm** context
- Map composed of wall, floor
- Rooms generated randomly
- Corridors link Rooms
- Player
- Camera which follow the Player
- Graphics to all Tiles

[unreleased]: https://github.com/ulphidius/dungeoncrawl/compare/v0.9.0...master
[0.9.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.8.1...v0.9.0
[0.8.1]: https://github.com/ulphidius/dungeoncrawl/compare/v0.8.0...v0.8.1
[0.8.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ulphidius/dungeoncrawl/releases/tag/v0.1.0
