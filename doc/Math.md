# Mathematics

## Goal

Mathematics is a set of rules for making decisions. For example:

- How much fuel is needed to launch the rocket to the Moon?
- Which drug is more effective against HIV?
- How to build a house that withstands a Category 4 hurricane?

We can't "try and see" because human lives are at stake. It's safer and faster to make these decisions using mathematics.

## Types

Mathematics begins with a simple-yet-deep observation: "some objects are interchangeable".

For example: two chairs are interchangeable if you can sit on any of them.

The chairs are different. _You_ are indifferent towards sitting on any of them.

If you were to describe the situation to another person, you would say: "I see two objects: `A` and `B`. But in my opinion, those objects have the same type: chair".

Short version:

- `A` is a chair.
- `B` is a chair.

## Constructors

Looking at the chairs, you notice the same components: backrest, seat, legs.

But each chair has its own components. Therefore, you describe different chairs with different components:

- Chair `A` contains backrest `AR`, seat `AS`, legs `AL`.
- Chair `B` contains backrest `BR`, seat `BS`, legs `BL`.

The components are different, but the blueprint is the same. There is a single physical instruction (a leaflet) that describes how to assemble these chairs. This instruction was applied twice to different components, which resulted in different chairs.

Let's focus on chair `A`.

You have the following objects in your physical reality:

- Chair `A`
- Backrest `AR`
- Seat `AS`
- Legs `AL`
- Instruction `MakeChair`

How would you express the idea that instruction `MakeChair` was applied to components `AR`, `AS`, `AL` to produce chair `A`?

Here's one way:

```text
A = MakeChair(AR, AS, AL)
```
