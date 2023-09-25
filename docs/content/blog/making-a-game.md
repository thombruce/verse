---
title: I'm Making a Game
layout: article
date: 2023-09-24T22:46:14Z
image: /uploads/making-a-game.png
---

![Code for a game](/uploads/making-a-game.png)

So, I'm making a game... and yeah, it's definitely way more than I can chew!

I know, I know, everybody thinks they can make a game and everybody has some super-ridiculous way too ambitious project and yup, I'm no different. Verse is... **BIG!** But it's my dream game and it's been circling around in my head for a while. I just need to make a start on it and start making itty-bitty steps towards probable failure, and when I hit failure keep going. That's how this gets done. I'm more than willing to compromise where I need to, to plan and draft everything before I commit a single line of code, and to seek out help when I need it. I just need to start. So with that said, what is Verse?

Verse is a space exploration game in which you can fly between systems spanning an entire galaxy, you can explore your own ship's interior and manage your crew, and you can touch down on any rocky planet to explore their worlds, be they barren, verdant or metropolitan. You'll encounter alien civilisations, engage in ship-to-ship combat and espionage, and you'll advance the power of your very own fleet... _maybe._ Verse is all of that, in concept.

In reality, Verse is right now... four lines of code, and all that they do is load the Bevy game engine and open an application window. This is the default starting script for any Bevy project:

```rs
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

Yeah, that ain't doing anything close to what I suggested Verse might one day do. Obviously I have a lot of challenges ahead of me, but it's worse still. I've never worked with Bevy, never developed a project in Rust, and I've never before made a finished game, not even a simple one.

So, what? I expect to go from nothing to creating an entire universe, complete with planets, cities and alien species to interact with?

Not quite. And Verse might not actually be the first game I publish, it's just the first I'm starting on. I know the systems I have in mind are ludicrously beyond my capabilities but I don't mind taking it slow and getting distracted if it helps.

To my mind, step one is to create a sort of Asteroids-like space flight simulation: you will control a ship and pilot it between planets; the planets will have gravity; you will be able to fire weapons. Basic stuff.

With that done, I don't think we move onto planetary landings straight away. It seems simpler to sort out the spacecraft interior first - same character controls, on a much smaller scale. You'll be able to switch between Asteroids-like flight controls and an interior view on the fly. The interior will reorient to a grid, on which you'll be able to move your character between individual tiles. All sounds simple enough, right? I don't know what the control scheme is like for that, yet. Either it's Rogue-like in its controls or you have a little more freedom, but it's top-down in either case.

Then we can start working on planet generation. The character controls at this point are done, so if we can generate a simple world then we can start exploring it immediately. _Yeah, I know, I know... "a simple world" is easier said than done, but we'll figure it out._ Cities will not be featured at this point, multi-biome planets will likely not be featured at this point. In fact, I'm almost certain planets will initially be entirely pointless flat planes that you can loop around if you're willing to sink the time into walking around them. We'll have to come up with an interesting purpose for them as we get closer to implementation.

All right, so you get the gist. It's a wholly top-down space adventure game. Not too ambitious when you put it like that, and we've broken it down into reasonably manageable steps. First we do Asteroids-like space flight, then we add planets and gravity to that simulation, then we introduce an interior scene where you can explore your spacecraft interior (eventually we'll introduce NPC needs and crew management), then we'll try to implement planetary surfaces.

That's, in a nutshell, my dream game. And I know that it's a game that has been done before, countless times - there's Starfield, No Man's Sky, Elite: Dangerous, X4, Star Citizen and Starbound, to name just a few. But I'm hoping that along the way we can find a way to differentiate Verse. These games all make different compromises in pursuit of simulating space exploration, and we'll make different ones. Like... I'm sure it has been done, but I can't actually think of an example of a top-down space exploration game. Let me know if you know of any.

Anyway, that's Verse - the idea, at least. I've got a lot of work ahead of me...
