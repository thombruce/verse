# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Separate nav and speedometer UI features into own modules

## [0.0.13] - 2023-10-10

### Added

- Spritesheet for planet with no atmosphere
- Two extra demo planets (using no atmosphere texture)
- KDTree search for nearest neighbour using bevy_spatial
- Working location indicator for nearest planet

### Changed

- Indicators now appear in circle close to player ship for better readability
- Add transparency to indicators
- Moved bulk of planet and star setup to own modules
- Renamed AppState to GameState for better clarity
- Improved readability and versatility of assets with bevy_asset_loader
- Moved menu music loading from main to menu module
- Moved celestial bodies and starfield background to new astronomy module
- Moved assets, camera, game_time and state to resources module
- Moved credits, pause and start_menu to menus module
- Moved blink effect to effects module
- Moved animation to effects module

## [0.0.12] - 2023-10-07

### Fixed

- Implement buffer to prevent crash to desktop when systems are loaded out of order

## [0.0.11] - 2023-10-07

### Added

- Moon orbiting planet, demonstrating new orbital system
- Indicator system added to the HUD
- (Optional) pixelation shader for retro effect
- (Optional) chromatic aberration shader
- Source and license info added to credit attributions

### Changed

- Generalised orbital system for any spatial coordinates and parent bodies
- Planet orbital radius extended to more "realistic" value
- HUD module moved to new hud directory

## [0.0.10] - 2023-10-04

### Added

- Animated star and planet exported as spritesheets from Deep-Fold's Pixel Planet Generator
- Basic orbital system moving planet in circlular orbit around star
- Pausable GameTime resource and tick system to prevent dynamic systems jumping forwards after unpause
- Attribution for Deep-Fold in Credits for Pixel Planets
- WorldInspectorPlugin from bevy-inspector-egui for development/debugging

### Changed

- Adjusted scale of ship smaller by default
- Moved background handling to plugin
- Adjusted movement scale of background for parallax effect
- Moved assets to reusable resources plugin in assets.rs
- Moved controls to table in README

### Removed

- Blog moved to [itch.io](https://thombruce.itch.io/verse)

## [0.0.9] - 2023-10-02

### Added

- Generalised state change handling with ForState component

### Changed

- Play menu music in credits state as well as start menu
- Fade in added to beginning of Space Dust by Kirk Osamayo for smoother transition

## [0.0.8] - 2023-10-01

### Added

- Start menu
- Start menu music: Lightspeed by Beat Mekanik (Free Music Archive, CC BY)
- Ambient music: Space Dust by Kirk Osamayo (Free Music Archive, CC BY)
- In-game credits accessible from the start menu
- Download links and controls in README

### Changed

- Pause system moved to new pause module
- Menu and pause state blinking moved to new effects module

## [0.0.7] - 2023-09-30

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

[unreleased]: https://github.com/thombruce/verse/compare/v0.0.13...HEAD
[0.0.13]: https://github.com/thombruce/verse/compare/v0.0.12...v0.0.13
[0.0.12]: https://github.com/thombruce/verse/compare/v0.0.11...v0.0.12
[0.0.11]: https://github.com/thombruce/verse/compare/v0.0.10...v0.0.11
[0.0.10]: https://github.com/thombruce/verse/compare/v0.0.9...v0.0.10
[0.0.9]: https://github.com/thombruce/verse/compare/v0.0.8...v0.0.9
[0.0.8]: https://github.com/thombruce/verse/compare/v0.0.7...v0.0.8
[0.0.7]: https://github.com/thombruce/verse/compare/v0.0.6...v0.0.7
[0.0.6]: https://github.com/thombruce/verse/compare/v0.0.5...v0.0.6
[0.0.5]: https://github.com/thombruce/verse/compare/v0.0.4...v0.0.5
[0.0.4]: https://github.com/thombruce/verse/compare/v0.0.3...v0.0.4
[0.0.3]: https://github.com/thombruce/verse/compare/v0.0.2...v0.0.3
[0.0.2]: https://github.com/thombruce/verse/compare/v0.0.1...v0.0.2
[0.0.1]: https://github.com/thombruce/verse/releases/tag/v0.0.1
