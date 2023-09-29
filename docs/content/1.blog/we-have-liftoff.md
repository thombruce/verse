---
title: We Have Liftoff
layout: article
date: 2023-09-29T18:49:29Z
image: /uploads/we-have-liftoff.png
---

![Code for a game](/uploads/we-have-liftoff.png)

Space! The... first frontier so far as my game is concerned. In an earlier post, [I'm Making a Game](./making-a-game), I described the steps I was intending on taking as I started my game development journey. Step one was SPACE! And that's roughly what we've got at this point. I've created a simple little simulation in which you control a spaceship. You're able to rotate it around its own centre using the `A` and `D` or `←` and `→` keys, and you can thrust the ship forwards using `W` or `↑`. The ship can reach a speed of around 1,370 m/s before some background mathematics prevents you accelerating any further.

Said background maths is actually a dampening effect, implemented to reduce your speed and rotation when you let go of the movement keys. This is not how space works, I know - an object in a vacuum _should_ maintain a constant rate of speed - but I'm not aiming for a true simulation here. Just imagine the dampeners are built in to the ship to help the pilot maintain their sense of orientation. We'll tweak how this works eventually, because I know 1,370 m/s is pretty slow on a cosmic scale too.

There is also a speedometer shown in the bottom right corner of the screen, part of me figuring out how Bevy UI works, as well as some static text in the top right that just says "In Space". This text will eventually tell you whether you're encountering or in orbit around a celestial body, and what the name of that body is, but we don't yet have any planets or moons so you're just "In Space" for now. We don't even have stars; not really.

Once I had fixed the camera to the player position, I realised I needed a new way to indicate player movement (because their position no longer changes relative to the screen), so I added a tiling background with the help of [bevy_tiling_background](https://github.com/braymatterorg/bevy_tiling_background) and a small starfield image from Kenney's [Space Shooter Redux](https://www.kenney.nl/assets/space-shooter-redux) asset pack (the player ship is also from this pack - I am no artist). I slightly edited the starfield image to increase the contrast between the stars and the black of space, but it's otherwise unmodified.

The result of this work is... you can fly a ship around a virtually infinite space (practically, you probably run into floating point precision errors if you go too far from the origin). The stars aren't real and the background tiling is super noticeable, especially at speed (part of why I currently limit the speed in the way that I do), but you have a sense of movement and a readout of your speed.

It's not a game yet, but it's a start. I'm thinking that we'll add planets next and try to figure out things like gravity and achieving orbit around them. Then we can potentially implement our first gameplay loop, probably consisting of little more than simple courier missions between the planetary bodies.

Planets will also give me an opportunity to think about scale, and those nasty floating point precision errors I briefly mentioned above. There's a real risk that if the player ship gets too far from the coordinate origin that things start to break down and not work at all like expected. There are a few ways to address this problem that I have talked myself through in a GitHub issue here: [Galactic and Planetary Scales](https://github.com/thombruce/verse/issues/12).

Oh yeah! The game is on GitHub. It isn't open source though by the common definition. Officially it is a source available project with a Commons Clause modified GNU General Public License. That means that you can read and even make use of the source code yourself, but you can't sell it in any way without substantial changes. This seemed like the fairest way to protect my work while still being able to make it available for learning and potentially for contributions or modding. You can find it here: [thombruce/verse](https://github.com/thombruce/verse). Check out the releases there too if you're interested in playing with the ship flight system, and keep an eye out for future releases when I hope it becomes something actually worth playing.

Lastly - god I'm so pleased with how this turned out - the game has a logo! I wanted a vibe that said _space adventure_ that wasn't too serious and the [Edge of the Galaxy](https://www.fontspace.com/edge-of-the-galaxy-font-f45748) font by Quinn Davis Type lept out at me; I think it captures exactly that vibe. Say hello to _Verse_.

![Verse Logo](/uploads/verse-galaxy.png){width="600px" class="mx-auto"}
