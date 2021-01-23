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
- [ ] The entities/relations are reactive. This means the properties of the entity/relation are processed internally using
  reactive technologies (for example an entity of type AND outputs the result of a boolean and operation of the
  two boolean inputs)

## Stories

### Developer

- [x] As a developer I can define components using the API
- [x] As a developer I can create entities (with properties) using the API
- [x] As a developer I can create relations (with properties) using the API
- [x] As a developer I can connect and disconnect two properties which data flows from one to the other using the API
- [x] As a developer I can create an entity of type AND-operation which outputs the AND-operation of two inputs using the API

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
- [ ] Logging

Further libraries might be of interest (asset management, remote procedure calls, ...) but these four libraries are the
essential ones.

## Setup

- [x] Readme
- [x] License
- [x] Code of Conduct
- [x] Changelog
- [x] Code Documentation (`cargo doc`)
- [x] Unit tests (`cargo test --package rust-ecs-poc --bin rust-ecs-poc  -- --nocapture --exact -Z unstable-options --show-output`)
- [x] Source Code Format (`cargo fmt`)
- [ ] Reference Documentation (rst, wiki)
- [x] Logging Framework
- [ ] GitHub Actions
  - [x] Compile
  - [x] Run tests
  - [x] Generate Code Documentation
  - [x] Create GitHub Release
  - [ ] Create snap package

## Implementation

### Models

#### Base Models (Serializable)

- [x] `Component`
  - [x] Serializable
- [x] `EntityType`
  - [x] Serializable
- [x] `EntityInstance`
  - [x] Serializable
  - [x] From Vertex Properties
- [x] `RelationInstance`
  - [x] Serializable
  - [x] From Edge Properties
- [ ] `Flow`
  - [ ] Serializable
  - [ ] List of entity_

#### Reactive Models (Non-Serializable, Managed by a Registry)

- [x] `ReactivePropertyInstance`
  - [x] Not serializable
  - [x] Getter
  - [x] Typed Getters
  - [x] Setter
  - [x] Send (Send but not set)
  - [x] Tick (Resend)
- [x] `ReactiveEntityInstance`
  - [x] Not serializable
  - [x] Construct ReactiveEntityInstance from Vertex
  - [x] Construct Properties
  - [x] Typed Getters
  - [x] Setter
- [x] `ReactiveRelationInstance`
  - [x] Not serializable
  - [x] Construct ReactiveRelationInstance from Edge
  - [x] Construct Properties
  - [x] Typed Getters
  - [x] Setter

### Behaviours

The idea is to wrap functionality around `ReactiveEntityInstance`s and `ReactiveRelationInstance`s.

A `ReactiveEntityInstance` has properties with streams but doesn't do anything yet.

The reactive behaviour implements the behaviour of a type. For example the AND

#### `EntityInstanceBehaviour`s

- [x] `ConstValue`
  - [x] Unit-Test: Fill `ConstValue` with external data
  - [x] Drop
- [x] `LogicalGate`
  - [x] AND
    - [x] Unit-Test: One AND-Gate R = (B1 && B2)
    - [x] Unit-Test: Three AND-Gates R = ((B1 && B2) && (B3 && B4)) -> Using Connectors
  - [x] OR
    - [x] Unit-Test: One OR-Gate R = (B1 || B2)
    - [x] Unit-Test: Three OR-Gates R = ((B1 || B2) || (B3 || B4)) -> Using Connectors
- [x] `ArithmeticGate`
  - [x] ADD
    - [x] Unit-Test: One ADD-Gate R = (N1 + N2)
    - [x] Unit-Test: Three AND-Gates R = ((N1 + N2) + (N3 + N4)) -> Using Connectors
  - [x] SUB
    - [x] Unit-Test: One SUB-Gate R = (N1 - N2)
    - [x] Unit-Test: Three SUB-Gates R = ((N1 - N2) - (N3 - N4)) -> Using Connectors
- [ ] `TrigonometricOperation`
  - [ ] SIN
  - [ ] COS
- [x] `SimpleClosure`
  - [x] PRINT

#### `RelationInstanceBehaviour`s

- [x] `Connector`
  - [x] `Connector::from_relation(ReactiveRelationInstance)`
  - [x] `Connector::new(OutboundEntity, OutboundPropName, InboundEntity, InboundPropName)`
  - [x] `Connector::connect`
  - [x] `Connector::disconnect`
  - [ ] Optionally: Initially send value down the stream
  - [ ] Optionally: Pause + Resume

### APIs

- [x] `Component Manager`
- [x] `EntityTypeManager`
- [x] `EntityInstanceVertexManager`
- [x] `EntityInstanceManager`
- [x] `ReactiveEntityInstanceManager`
- [x] `RelationTypeManager`
- [ ] `RelationInstanceManager`
- [ ] `ReactiveRelationInstanceManager`
- [ ] `FlowManager`
  - [ ] List of entity_ids
  - [ ] List of relation_ids
  - [ ] Register Entity Instance for Flow
  - [ ] Register Relation Instance for Flow
  - [ ] Import / Export
    - [ ] Full exports of all entity instances and relation instances

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
- [ ] `ReactiveEntityInstanceManager`
  - [x] Hold references of `ReactiveEntityInstance`
    * These are the actually "running" / "living" instances
  - [ ] Create `ReactiveEntityInstance` by UUID
    - [ ] Get Entity Instance by UUID from `EntityInstanceManager`
    - [ ] On Instantiation: Instantiate `ReactiveEntityInstanceBehaviour` by TYPE
      - [ ] ConstValue
      - [ ] Logical Gate (AND)
      - [ ] Arithmetic Gate (AND)
      - [ ] Print
  - [ ] Check if id exists in HashMap (must not exist)
  - [ ] Check if id exists in Datastore -> Manager
- [ ] `ReactiveRelationInstanceManager`
  - [ ] Central registry of all `ReactiveRelationInstance`s
    * These are the actually "running" / "living" instances
  - [ ] Create `ReactiveRelationInstance` by UUID
    - [ ] Get Relation Instance by UUID from `RelationInstanceManager`
    - [ ] On Instantiation: Instantiate `ReactiveRelationInstanceBehaviour` by TYPE
      - [ ] Connector
- [ ] `ConnectorManager`
  - [ ] Central registry of all `Connectors`s
  - [ ] (handle_id) = connect(uuid, prop_name, uuid, prop_name)
  - [ ] connect(`ReactiveEntityInstance`, prop_name, `ReactiveEntityInstance`, prop_name)
  - [ ] disconnect(uuid, prop_name, uuid, prop_name)
  - [ ] connect(`ReactiveEntityInstance`, prop_name, `ReactiveEntityInstance`, prop_name)
  
- [ ] `FlowManager`
  - [ ] Map of `Flows`
    - [ ] List of entity_ids
    - [ ] List of relation_ids
  - [ ] Register Entity Instance for Flow (= register UUID)
  - [ ] Register Relation Instance for Flow (= register UUID)
  - [ ] Export
    - [ ] For each registered EntityInstance UUID: Load the `EntityInstance` via `EntityInstanceManager`
    - [ ] For each registered RelationInstance UUID: Load the `RelationInstance` via `RelationInstanceManager`
    - [ ] Construct one big JSON
  - [ ] Import
    - [ ] Read one big JSON
    - [ ] For each `EntityInstance`: Create EntityInstance via `EntityInstanceManager`
      - [ ] For each `RelationInstance`: Create EntityInstance via `RelationInstanceManager`
