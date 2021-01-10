# Component

What is a `Component`?

A `Component` is a set of `Properties` that:

* fulfills a specific purpose,
* can be applied to different `Entity Types` and/or `Relation Types`
  * (A `Component` is reusable by and useful for multiple `Entity Types` and/or `Relation Types`)
* and, optionally, specifies a behaviour

![Component](https://yuml.me/diagram/scruffy/class/[Component{bg:green}|uuid;name]++1-N>[Property%20Type|uuid;name;datatype])

## Examples

| Component | Purpose | Reusable? | Behaviour |
| --- | --- | --- | --- |
| Named | Gives an `Entity Instance` a name | <ul><li>A `Player` has a name</li><li>A `Particle Emitter` has a name</li></ul> |
| Positionable | The `Entity Instance` has a position in the cartesian coordinate system. | <ul><li>A `Player` has a position</li><li>A `Particle Emitter` has a position</li></ul> |
| Movable | The position of an `Entity Instance` is not static. It moves along a velocity vector. | <ul><li>A `Player` has a position and moves to another position.</li><li>A `Particle Emitter` has a position and may be moving.</li></ul> | The position gets updated by adding the velocity vector to the origin vector. |

![Named](https://yuml.me/diagram/scruffy/class/[Component%201{bg:green}|name%20=%20Named]++->[Property%201-1|name%20=%20name;datatype%20=%20string],[Component%202{bg:green}|name%20=%20Positionable]++->[Property%202-1|name%20=%20px;datatype%20=%20i32],[Component%202]++->[Property%202-2|name%20=%20py;datatype%20=%20i32],[Component%202]++->[Property%202-3|name%20=%20pz;datatype%20=%20i32],[Component%203{bg:green}|name%20=%20Movable]++->[Property%203-1|name%20=%20vx;datatype%20=%20i32],[Component%203]++->[Property%203-2|name%20=%20vy;datatype%20=%20i32],[Component%203]++->[Property%203-3|name%20=%20vz;datatype%20=%20i32],[Component%203]depends%20on-->[Component%202])

## The behaviour

As you can see, not every `Component` has a behaviour. But here are some examples of behaviours:

| Component | Behaviour |
| --- | --- |
| AND_2 | The output is the result of `bit1 && bit2` |
| INT_STRING | The output is the string representation of the integer input |
