# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Pause state
- Control mapping using leafwing-input-manager
- Gamepad support

### Changed

- Move ship spawning and system registration from main to plugin
- Move camera spawning and follow_player system from main to plugin

## [0.0.6] - 2023-09-29

### Changed

- Replaced Commons Clause incompatible license with MIT

## [0.0.5] - 2023-09-29

### Added

- HUD speedometer
- HUD location indicator as static "In Space" text
- Window title set to "Verse"
- kenvector_future font from Kenney's Assets
- Linux and MacOS builds to Release workflow

### Changed

- Set ship mass directly rather than deriving from density
- Move camera's follow player system to new camera module
- Decrease linear velocity dampening (faster max speed, slower natural deceleration)

## [0.0.4] - 2023-09-28

### Added

- Follow player for camera component
- Endless repeatable background

## [0.0.3] - 2023-09-27

### Added

- Workflow to build versioned game releases

## [0.0.2] - 2023-09-27

### Added

- Spawn camera and spaceship player sprite
- Basic player controls
- Rapier physics engine for ship controls
- Game assets from kenney.nl
- Git LFS config for image storage
- Installation and Credits sections in README

## [0.0.1] - 2023-09-25

### Added

- Bevy game engine and initial config
- Docs site for documenting Verse development
- "Commons Clause" prepended GNU GPLv3 license
- CHANGELOG.md matching Keep a Changelog formatting
- README with link to license doc and changelog

[unreleased]: https://github.com/thombruce/verse/compare/v0.0.6...HEAD
[0.0.6]: https://github.com/thombruce/verse/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/thombruce/verse/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/thombruce/verse/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/thombruce/verse/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/thombruce/verse/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/thombruce/verse/releases/tag/v0.0.1
