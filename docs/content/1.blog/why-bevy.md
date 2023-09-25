---
title: Why Bevy?
layout: article
date: 2023-09-25T23:14:10Z
image: /uploads/why-bevy.png
---

![Code for a game](/uploads/why-bevy.png)

When you start a project in Unity, Godot or most any other modern general purpose game engine, you're asked immediately to make a decision about what sort of game you're making: is it 2D or 3D; does it target web, desktop or mobile; what kind of renderer will it use; etc., etc. And this is... good, obviously. You can make adjustments to the setup later, swap between and blend 2D and 3D environments, target multiple platforms; you aren't locked into these decisions, but you have a set of guiding principles to get you started. Bevy is like this too... sort of.

In Bevy, you don't even really start with Bevy. You have to start by creating a new Rust project like so:

```sh
cargo new --bin verse
```

This creates a new folder called 'verse' (replace with your own project name) and initialises a Rust project inside. To then add Bevy to this project, you have to `cd` into the folder and run:

```sh
cargo add bevy
```

This adds Bevy to the dependencies list in your project's `Cargo.toml` file. When you next run the project, it will compile with Bevy though you still aren't technically making use of the engine. To do that, you need to modify your `src/main.rs` file to something like:

```rs
use bevy::prelude::*;

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
```

You'll recognise this piece of code from my last blog entry. This is the minimum Bevy setup required to launch the application in a new window and... do nothing else. You still at this point haven't expressed anything to Bevy about the kind of game you're making or what platform you're targeting.

This is clearly a very different approach. Whereas game engines like Unity and Godot come with rich GUI applications for editing your game and will get you started by making some choices about what to render to the screen and how, with Bevy we're more sort of... cobbling the game engine together on the fly too.

Now, this isn't the same as writing your own engine. Bevy **is** a game engine written in Rust, but we're picking precisely the components we need as and when we need them rather than steamrolling through some of those decisions early on. Bevy does have several default features turned on out of the box, but these can also optionally be disabled or replaced with other features. And I'm not saying that this is a better approach - it's not - or even that you can't refine your builds in other game engines the same way - I'm sure you can. But Bevy is, in a way, a lot more upfront about what's actually going on and I feel it's clearer where certain responsibilities are delegated or otherwise left as an exercise for the user.

Like... make no mistake, we're making a Rust application that includes Bevy as a dependency. Other game engines obfuscate the relationship between engine and programming language. In Unity for instance you write code in C# and the engine itself is written in a combination of C# and C++. Unreal uses and was written in C++. Godot was written in C++ and supports writing code in C++, C# or GDScript. While this is all very clear in the documentation, the blurring of the boundaries between the programming languages which underpin these engines' capabilities and the engine frameworks themselves feels off to me. It fuzzies the lines of each part's responsibility and seems to me to be a very modern convenience that adds bloat and confusion. _To be fair, though, this confusion won't necessarily be a problem for 99.9% of games developed; I'm talking more about how it blurs the lines when handling more complex tasks like doing something outside of the scope of the engine itself._

I know, I know: _All general purpose game engines are modern conveniences. Back in the day, developers had to write their own engines from scratch._ True. But I like the approach on offer here of all of the different conveniences on offer. I like the approach of having to cobble together the engine as we go, extending or modifying its feature set as needed, because I feel like it provides the greatest clarity as to what is _actually_ going on.

I also like the Rust programming language more than I like C# or C++. Can't really put my finger on why though; it's just a flavour thing. I feel like I'd rather learn it than either of those. GDScript does sound nice too, with its Python-like syntax, but I'm not keen on the idea of learning a programming language designed to be used exclusively within a game engine.

All of this is why I'm choosing Bevy and Rust as my game engine and programming language of choice. There's every chance I come to regret the decision, but I'm already having fun just trying to learn both, which is what's most important really; that it's fun - without that, projects get abandoned.

_Might I have had an easier time and just as much fun in Unity, Unreal or GameMaker?_ Maybe, but I'm also drawn to favour the more free and open source licensing terms of Godot and Bevy.

Godot has run up second in my considerations for all of the reasons above. I understand that it is a more mature, more production ready game engine than Bevy. I also understand that Godot doesn't even limit you to the choice of three languages, but supports creating bindings to other languages - including Rust - via GDNative. I know, and still I'm for some reason drawn more to Bevy right now.

If it comes to it, Godot is my fallback and I would be so happy to use it. I would even recommend it to others over Bevy who are, like me, trying to make their first game. To be clear, I am making the wrong choice, the bad choice, I am insane; Godot is no doubt easier to learn for beginners with much more support available should you get stuck. But I have explained my reasons and I hope you can see why I've chosen the engine I have, Bevy, instead.

_Yeah, insanity._

Yeah.

We'll see how far insanity gets us.
