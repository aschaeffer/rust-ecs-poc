# Semantic Reactive Entity Component System (SRECS)

## Motivation

* Highly flexible
* Highly decoupled

## Property

A `Property` is a concrete state of an `Entity Instance` or a `Relation Instance`. A property has a name and a value:

![Property](https://yuml.me/diagram/scruffy/class/[Property|uuid;name;value;...])

[comment]: <> (## Component)

[comment]: <> (What is a `Component`?)

[comment]: <> (A `Component` is a set of `Properties` that:)

[comment]: <> (* fulfills a specific purpose,)

[comment]: <> (* can be applied to different `Entity Types` &#40;is reusable&#41;)

[comment]: <> (* and, optionally, specifies a behaviour)

[comment]: <> (![Component]&#40;https://yuml.me/diagram/scruffy/class/[Component{bg:green}|uuid;name]++1-N>[Property|uuid;name;datatype]&#41;)

[comment]: <> (### Examples)

[comment]: <> (| Component | Purpose | Reusable? | Behaviour |)

[comment]: <> (| --- | --- | --- | --- |)

[comment]: <> (| Named | Gives an `Entity Instance` a name | <ul><li>A `Player` has a name</li><li>A `Particle Emitter` has a name</li></ul> |)

[comment]: <> (| Positionable | The `Entity Instance` has a position in the cartesian coordinate system. | <ul><li>A `Player` has a position</li><li>A `Particle Emitter` has a position</li></ul> |)

[comment]: <> (| Movable | The position of an `Entity Instance` is not static. It moves along a velocity vector. | <ul><li>A `Player` has a position and moves to another position.</li><li>A `Particle Emitter` has a position and may be moving.</li></ul> | The position gets updated by adding the velocity vector to the origin vector. |)

[comment]: <> (![Named]&#40;https://yuml.me/diagram/scruffy/class/[Component%201{bg:green}|name%20=%20Named]++->[Property%201-1|name%20=%20name;datatype%20=%20string],[Component%202{bg:green}|name%20=%20Positionable]++->[Property%202-1|name%20=%20px;datatype%20=%20i32],[Component%202]++->[Property%202-2|name%20=%20py;datatype%20=%20i32],[Component%202]++->[Property%202-3|name%20=%20pz;datatype%20=%20i32],[Component%203{bg:green}|name%20=%20Movable]++->[Property%203-1|name%20=%20vx;datatype%20=%20i32],[Component%203]++->[Property%203-2|name%20=%20vy;datatype%20=%20i32],[Component%203]++->[Property%203-3|name%20=%20vz;datatype%20=%20i32],[Component%203]depends%20on-->[Component%202]&#41;)

[comment]: <> ([comment]: <> &#40;![Positionable]&#40;https://yuml.me/diagram/scruffy/class/[Component%202|name%20=%20Positionable]-->[Property%202-1|name%20=%20px;datatype%20=%20i32],[Component%202]-->[Property%202-2|name%20=%20py;datatype%20=%20i32],[Component%202]-->[Property%202-3|name%20=%20pz;datatype%20=%20i32]&#41;&#41;)

[comment]: <> ([comment]: <> &#40;![Movable]&#40;https://yuml.me/diagram/scruffy/class/[Component%203|name%20=%20Movable]-->[Property%203-1|name%20=%20vx;datatype%20=%20i32],[Component%203]-->[Property%203-2|name%20=%20vy;datatype%20=%20i32],[Component%203]-->[Property%203-3|name%20=%20vz;datatype%20=%20i32]&#41;&#41;)

[comment]: <> (### The behaviour)

[comment]: <> (As you can see, not every `Component` has a behaviour. But here are some examples of behaviours:)

[comment]: <> (| Component | Behaviour |)

[comment]: <> (| --- | --- |)

[comment]: <> (| AND_2 | The output is the result of `bit1 && bit2` |)

[comment]: <> (| INT_STRING | The output is the string representation of the integer input |)


## Entity Instance

What is an `Entity Instance`?

An `Entity Instance` is a particular example or occurrence of something. It could be anything in the game which is like an object: a player, a teleporter, a light,
a particle emitter, ...

### Example: Player named PeterPenacka

The `Entity Instance` has concrete values:

![PeterPenacka](https://yuml.me/diagram/scruffy/class/[Player|uuid;name%20=%20PeterPenacka;px%20=%200;py%20=%2050;pz%20=%2060;...])

The state of one concrete player is represented by one concrete `Entity Instance`.

## Entity Type

An `Entity Type` is an abstract specification of `Entity Instances`.

The specification consists of two parts:

* Which properties are available?
* The behaviour

![Player](https://yuml.me/diagram/scruffy/class/[Entity%20Type|name%20=%20Player]-->[Component%201|name%20=%20Named],[Entity%20Type]-->[Component%202|name%20=%20Positionable],[Entity%20Type]-->[Component%203|name%20=%20Movable]-->[Property%201|name%20=%20vx;datatype%20=%20i32],[Component%203]-->[Property%202|name%20=%20vy;datatype%20=%20i32],[Component%203]-->[Property%203|name%20=%20vz;datatype%20=%20i32])

### Relation

A `Relation` has a connection to a start node and to an end node:

![relation](https://yuml.me/diagram/scruffy/class/[Entity%201]<-start_node[Relation]end_node->[Entity%202])

### Properties

`Entities` and `Relations` can have `Properties`

![properties](https://yuml.me/diagram/scruffy/class/[Relation]1-N[Property|uuid;name;type]1-N[Entity])

### Entity Property

* A `Property` has exactly one (incoming) `Slot`
* A `Property` may have outgoing `Signal`s
* A `Signal` is connected to exactly one `Slot`

![yuml](https://yuml.me/diagram/scruffy/class/[Signal]1->1[Slot],[Property]1->N[Signal],[Property]1<-1[Slot])

This means:

* A `Property` can be activated from outside by it's slot

## Reactive Entity

A reactive entity is an entity which processes it's own state. As soon as (one of) the inputs are changed the output gets calculated by an `EntityProcessor`.

![ReactiveEntity](https://yuml.me/diagram/scruffy/class/[DOOR_1|is_open]bit1-->[AND|bit1;bit2;result],[DOOR_2|is_open]bit2-->[AND]result-->[LIGHT_1|is_on])

### Entity State

The state of an entity is represented by it's properties.

![EntityState](https://yuml.me/diagram/scruffy/class/[Property%201|name%20=%20bit1;type%20=%20input]-->[AND],[Property%202|name%20=%20bit2;type%20=%20input]-->[AND]-->[Property%203|name%20=%20result;type%20=%20output])

Properties can be of two types:

* input
* output

### Entity Processor

A reactive entity is an entity which state
