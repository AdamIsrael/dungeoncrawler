# Dungeon Crawl

This is my implementation of a roguelike dungeon crawl game, written by following the tutorial in [Hands-on Rust: Effective Learning through 2D Game Development and Play](https://pragprog.com/titles/hwrust/hands-on-rust/), written by [Herbert Wolverson](https://github.com/thebracket) and published by [The Pragmatic Programmers](https://pragprog.com).

There are a lot of things I would like to try, or do differently, in making this a more robust game. This README is serving as a quasi-design doc/corkboard for ideas.

## Keybindings

| Key | Action |
| --- | ------ |
| Left | Move Left |
| Right | Move Right |
| Up | Move Up |
| Down | Move Down |
| G | Get an Item |
| 1-9 | Use an Item |
| Space | Wait/skip a turn |

## Limitations

- You can hold an unlimited number of items, but only activate the first 9
- Wieldable/wearable items (weapons, etc) take up an inventory slot
- Items can be limited by game level but not player level
- No experience system, so player can't level up
- Player has hit points but no other attributes (charisma, strength, luck, etc)
- Static lighting - everything in the entities field of view is well-lit.
- There are only three levels to each dungeon. A more challenging mode might include more levels that need to be explored.

## Wishlist

### Graphics

It'd be nice to find a diverse set of character sprites for the player to choose from.

Animated sprites?

### Town

The player starts off in a randomly generated town, a "dungeon" level of its own, based on the prefab builder. We know the features of the town (a graveyard, church, tavern, blacksmith, other shops), but each iteration would be different and the layout could change between games.

### Game State

The game should be pausable, savable, and reloadable.


### Non-Playable Characters (NPCs)

NPCs play a vital role, especially if you have any kind of story you wish to share with the adventurer.

They can also offer goods and services.

In particular, one idea I like is that a character with a high-enough charisma (or enough coin) could recruit an NPC as a follower. Not every NPC, though, because that could affect story. But there could be randomly generated NPCs, in the tavern, which could be hired or persuaded to follow the player on an adventure (and carry more stuff for them).

### Areas of effect

An entity should be able to emit an effect that affects an area around it. A bard, for example, might raise everyone's spirits. A cursed object might reduce the speed of everyone around it.

A light source, like a lantern or torch, would have an area of effect that would increase the light within a certain distance, and cast deeper shadows beyond it's range.

### Player attributes

There should be more attributes about a player other than hit points. These attributes can be used for rolls and savings throws in the game.

- race
    - subrace
    - skin tone
- gender
- luck?
- languages spoken
- languages read
- languages written


If we follow D&D 5e (tbd)...

- strength
- dexterity
- constitution
- intelligence
- wisdom
- charisma

Other interesting ideas:

- mental health
    - phobias that prevent them from performing certain actions, or make them less effective. Disordered eating. Stress.
- vision - how far the character can see without assistance?

### Races

Racial Traits. Every race has it's pros and cons. Modifiers to core attributes and other things. Like Drow would have better eyesight in the dark?

Races:

- Dwarf
    - Mountain Dwarf
- Elf
    - Wood Elf
    - Dark Elf
- Halfling
- Human

Racial Traits:
- attribute modifiers (strength, charisma, etc)
- age
- size
- speed
- languages:
    - spoken
    - written
    - read

### Rareness

Currently implemented as `frequency`, but the scale is wrong? Right now, the three swords all spawn at the same frequency. This should be expressed as rarity, possibly affected by the characters `luck` attribute.

### Items

Items have value (gold). That's the value that they're _bought_ at. Selling to an NPC will have a percentage knocked off the offered price, based on the players attributes/ability to haggle. NPC always gets a fair price, though.

### Cursed Items

Not all items benefit the player. Some items have negative affects and are not so easily removed/unequipped.

### Wearable Items

A class of item that can be worn/wielded by the player.

Items can be worn on:

- head (hat, helmet, tiara)
- face (eye glasses?)
- arms (bracers)
- torso (armor, robe)
- legs (pantaloons?)
- left hand (weapon, relic, torch, etc)
- right hand (weapon, relic, torch, etc)
- feet (shoes)
- back (cape, backpack)

Should each item have hit points, so that it can be damaged or destroyed? If so,
how do you repair items? Take them to a specialist (tailor, blacksmith) for repair?

It would also mean that item _slots_ could be targeted by an attack? Like, a strong attack is going to hit the torso, but attacks that land but swing wild might hit an arm, the head, or even a foot. Could we incur damage to the body part itself? For example, a creature wielding a hammer swings wild and strikes the character in the leg. Would that affect their movement?


Items can be worn, carried, or dropped. When an item is dropped, it's placed back on the map.