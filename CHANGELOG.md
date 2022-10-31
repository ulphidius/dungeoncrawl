# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[0.3.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/ulphidius/dungeoncrawl/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/ulphidius/dungeoncrawl/releases/tag/v0.1.0
