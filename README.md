# Inexor Reactive Semantic Entity Component System

[<img src="https://img.shields.io/badge/Language-Rust-brightgreen">]()
[<img src="https://img.shields.io/badge/Platforms-Linux%20%26%20Windows-brightgreen">]()
[<img src="https://img.shields.io/github/workflow/status/aschaeffer/rust-ecs-poc/Inexor%20Reactive%20Semantic%20Entity%20Component%20System">](https://github.com/aschaeffer/rust-ecs-poc/actions?query=workflow%3AInexor%20Reactive%20Semantic%20Entity%20Component%20System)
[<img src="https://img.shields.io/github/license/aschaeffer/rust-ecs-poc">](https://github.com/aschaeffer/rust-ecs-poc/blob/main/LICENSE)
[<img src="https://img.shields.io/discord/698219248954376256?logo=discord">](https://discord.com/invite/acUW8k7)

[<img src="https://img.shields.io/github/contributors/aschaeffer/rust-ecs-poc">]()
[<img src="https://img.shields.io/github/downloads/aschaeffer/rust-ecs-poc/total?color=brightgreen">]()
[<img src="https://img.shields.io/github/last-commit/aschaeffer/rust-ecs-poc">]()
[<img src="https://img.shields.io/github/issues/aschaeffer/rust-ecs-poc">]()
[<img src="https://img.shields.io/github/languages/code-size/aschaeffer/rust-ecs-poc">]()

[<img src="https://raw.githubusercontent.com/aschaeffer/rust-ecs-poc/main/docs/images/inexor_2.png">]()

Inexor is an open-source project which ???.

The main objective of this repository is the development of a entity component system based on Rust, a graph database,

Inexor is licensed under the MIT license.

## About

The `Reactive Semantic Entity Component System` is meant to be the core of the `game engine` which represents the
`game state` as the one and only truth using `entities` and `semantic relations` in a graph that controls and runs
the `game logic` using `reactive technologies`.

## Goals

The minimal valuable product (MVP) contains the following goals:

- [ ] The entity system is a network of entities which are connected with semantic and directional relations in a graph
- [ ] The entity system is a network of properties which are connected with connectors in order to control the data flow
  from one entity to another
- [ ] The underlying data store technology is a graph database which is embedded in the application
- [ ] The entities are reactive. This means the properties of the entity/relation are processed internally using
  reactive technologies (for example an entity of type AND outputs the result of a boolean and operation of the
  two boolean inputs)

## Stories

### Developer

- [x] As a developer I can define components using the API
- [x] As a developer I can create entities (with properties) using the API
- [ ] As a developer I can connect and disconnect two properties which data flows from one to the other using the API
- [ ] As a developer I can create an entity of type AND-operation which outputs the AND-operation of two inputs using the API

### Flow Designer

#### JSON

- [ ] As a flow designer I can define components which can be used as building block for entities using a JSON file
- [ ] As a flow designer I can create entities with properties using a JSON file
- [ ] As a flow designer I can connect and disconnect two properties which data flows from one to the other using a JSON file
  
#### JavaScript

- [ ] As a flow designer I can define components which can be used as building block for entities using JavaScript
- [ ] As a flow designer I can create entities with properties using JavaScript
- [ ] As a flow designer I can connect and disconnect two properties which data flows from one to the other using JavaScript
- [ ] As a flow designer I can create an entity of type boolean constant
- [ ] As a flow designer I can create an entity of type AND-operation which outputs the AND-operation of two inputs

## Library Integrations

The goal is to integrate a set of strategically important libraries and glue them together in order to archive the
goals:

- [x] Dependency Injection (waiter_di)
- [x] Embedded Graph Database (indradb)
- [x] Reactive Streams (bidule)
- [ ] JavaScript Runtime (deno)

Further libraries might be of interest (asset management, remote procedure calls, ...) but these four libraries are the
essential ones.

## Setup

- [x] Readme
- [x] License
- [x] Code of Conduct
- [x] Changelog
- [ ] Code Documentation
- [x] Unit tests
- [ ] Reference Documentation
- [ ] Source Code Format Linter
- [ ] GitHub Actions
  - [ ] Compile
  - [ ] Run tests
  - [ ] Create snap package
  - [ ] Lint Source Code Format

## Implementation

### Models

- [x] `Component`
- [x] `EntityType`
- [x] `EntityInstance`
- [ ] `ReactiveEntityInstance`
  - [x] Not serializable
  - [x] Running / Living instances
  - [ ] Construct ReactiveEntityInstance from Vertex
  - [ ] Check if id exists in Datastore
  - [ ] Check if id exists in HashMap
  - [ ] Get EntityType from Vertex Type
  - [ ] Construct Properties
  - [ ] stream
  - [ ] EntityInstance::new()
  - [ ] id: vertex.id
  - [ ] type_name
  - [ ] components (vec<String>)
  - [ ] properties (vec<ReactivePropertyInstance>)
  - [ ] Add ReactiveEntityInstance to HashMap
  - [ ] Destruct EntityInstance
    - [ ] Destruct All PropertyInstances
    - [ ] Remove Entry from HashMap
- [ ] `ReactivePropertyInstance`
- [ ] `Connector`
- [ ] `Flow`

### APIs

- [x] `Component Manager`
- [x] `EntityTypeManager`
- [x] `EntityInstanceVertexManager`
- [x] `EntityInstanceManager`
- [x] `ReactiveEntityInstanceManager`
- [ ] `RelationTypeManager`
- [ ] `RelationInstanceManager`
- [ ] `ReactiveRelationInstanceManager`

### Service Layer Implementations

- [x] `ComponentManager`
  - [x] Store references of `Component`
  - [x] Has Component by Name
  - [x] Register Component
  - [x] Get All Components
  - [x] Get Component By Name
  - [x] Delete Component By Name
  - [x] Export Component To JSON File
  - [x] Import Component From JSON File
- [x] `EntityTypeManager`
  - [x] Store references of `EntityType`
  - [x] Has Entity Type by Name
  - [x] Register Entity Type
    - [x] Expand Effective Properties From All Components (merge properties with the properties provided by the components)
  - [x] Create Entity Type
  - [x] Get Entity Type by Name
  - [x] Delete Entity Type By Name
  - [x] Export Entity Type To JSON File
  - [x] Import Entity Type From JSON File
- [x] `EntityInstanceVertexManager`
  - [x] Has Vertex by UUID
  - [x] Get Vertex by UUID
  - [x] Get Vertex Properties by UUID
  - [x] Create Vertex
  - [x] Create Vertex with UUID
    - [x] Check if id exists in Datastore (must not exist)
    - [x] Create Vertex Properties
  - [x] Delete Vertex
- [x] `EntityInstanceManager`
  - [x] Has Entity Instance by UUID
  - [x] Get Entity Instance by UUID
  - [x] Create Entity Instance
  - [x] Create Entity Instance with UUID
  - [x] Delete Entity Instance By UUID
  - [x] Import EntityInstance from JSON
    - [x] Check if id exists in Graph Database (must not exist)
    - [x] Create Vertex
  - [x] Export EntityInstance to JSON
    - [x] Create EntityInstance from Vertex
- [ ] Reactive Entity Instance Manager
  - [x] Hold references of "ReactiveEntityInstance"
    * These are the actually "running" / "living" instances
  - [ ] Check if id exists in HashMap (must not exist)
