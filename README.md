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

- [x] The entity system is a network of entities which are connected with semantic and directional
  relations in a graph
- [x] The entity system is a network of properties which are connected with connectors in order to
  control the data flow from one entity to another
- [x] The underlying data store technology is a graph database which is embedded in the application
- [x] The entities/relations are reactive. This means the properties of the entity/relation are
  processed internally using reactive technologies (for example an entity of type AND outputs the
  result of a boolean and operation of the two boolean inputs)

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
- [ ] As a flow designer I can create flows using JavaScript

#### GraphQL

- [ ] As a flow designer I can define components which can be used as building block for entities using a GraphQL interface
- [ ] As a flow designer I can create entities with properties using a GraphQL interface
- [ ] As a flow designer I can connect and disconnect two properties which data flows from one to the other using a GraphQL interface
- [ ] As a flow designer I can create flows using a GraphQL interface

## Library Integrations

The goal is to integrate a set of strategically important libraries and glue them together in order to archive the
goals:

- [x] Dependency Injection (waiter_di)
- [x] Embedded Graph Database (indradb)
- [x] Reactive Streams (bidule)
- [x] Logging
- [ ] Websockets Server
- [ ] HTTP Server
- [ ] GraphQL Server
- [ ] JavaScript Runtime (deno)

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
- [x] GitHub Actions
  - [x] Compile
  - [x] Run tests
  - [x] Generate Code Documentation
  - [x] Create GitHub Release by Tag
- [ ] Packaging
  - [ ] Create snap package

## Implementation

### Models

#### Base Models (Serializable)

- [x] `Component`
- [x] `EntityType`
- [x] `EntityInstance`
  - [x] From Vertex Properties
- [x] `RelationInstance`
  - [x] From Edge Properties
- [x] `Flow`
  - [x] List of entity instances
  - [x] List of relation instances

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
- [x] `ReactiveFlow`
  - [x] List of `ReactiveEntityInstance`
  - [x] List of `ReactiveRelationInstance`

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
- [x] `TrigonometricOperation`
  - [x] SIN
  - [x] COS
- [x] `SimpleClosure`
  - [x] PRINT
  - [x] LOG

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
- [x] `EntityVertexManager`
- [x] `EntityInstanceManager`
- [x] `ReactiveEntityInstanceManager`
- [x] `RelationTypeManager`
- [x] `RelationEdgeManager`
- [x] `RelationInstanceManager`
- [x] `ReactiveRelationInstanceManager`
  - [x] Resolves which behaviour(s) should be applied on an entity
  - [x] Delegation to Registry
- [x] `ReactiveEntityManager` delegates to `EntityBehaviourManager`
- [x] `ReactiveRelationManager` delegates to `RelationBehaviourManager`
- [x] `EntityBehaviourManager` delegates to `EntityBehaviourRegistries`
- [x] `RelationBehaviourManager` delegates to `RelationBehaviourRegistries`
- [x] `EntityBehaviourRegistry`
- [x] `RelationBehaviourRegistry`
- [x] `EntityBehaviourFactory`
- [x] `RelationBehaviourFactory`
- [x] `EntityBehaviour`
- [x] `RelationBehaviour`
- [x] `FlowManager`
- [x] `ReactiveFlowManager`

### Service Layer Implementations

- [x] `ComponentManagerImpl`
  - [x] Store references of `Component`
  - [x] Has Component by Name
  - [x] Register Component
  - [x] Get All Components
  - [x] Get Component By Name
  - [x] Delete Component By Name
  - [x] Export Component To JSON File
  - [x] Import Component From JSON File
  - [x] Unit Tests
- [x] `EntityTypeManagerImpl`
  - [x] Store references of `EntityType`
  - [x] Has Entity Type by Name
  - [x] Register Entity Type
    - [x] Expand Effective Properties From All Components (merge properties with the properties provided by the components)
  - [x] Create Entity Type
  - [x] Get Entity Type by Name
  - [x] Delete Entity Type By Name
  - [x] Export Entity Type To JSON File
  - [x] Import Entity Type From JSON File
  - [x] Unit Tests
- [x] `RelationTypeManagerImpl`
  - [x] Store references of `RelationType`
  - [x] Has Relation Type by Name
  - [x] Register Relation Type
    - [x] Expand Effective Properties From All Components (merge properties with the properties provided by the components)
  - [x] Create Relation Type
  - [x] Get Relation Type by Name
  - [x] Delete Relation Type By Name
  - [x] Export Relation Type To JSON File
  - [x] Import Relation Type From JSON File
  - [x] Unit Tests
- [x] `EntityVertexManagerImpl`
  - [x] Has Vertex by UUID
  - [x] Get Vertex by UUID
  - [x] Get Vertex Properties by UUID
  - [x] Create Vertex
  - [x] Create Vertex with UUID
    - [x] Check if id exists in Datastore (must not exist)
    - [x] Create Vertex Properties
  - [x] Delete Vertex
  - [x] Unit Tests
- [x] `RelationEdgeManagerImpl`
  - [x] Has Edge by Outbound-UUID, type-name and Inbound-UUID
  - [x] Get Edge by Outbound-UUID, type-name and Inbound-UUID
  - [x] Get Edge Properties by Outbound-UUID, type-name and Inbound-UUID
  - [x] Create Edge
  - [x] Delete Edge By Outbound-UUID, type-name and Inbound-UUID
  - [x] Unit Tests
- [x] `EntityInstanceManagerImpl`
  - [x] Has Entity Instance by UUID
  - [x] Get Entity Instance by UUID
  - [x] Create Entity Instance
  - [x] Create Entity Instance with UUID
  - [x] Delete Entity Instance By UUID
  - [x] Import EntityInstance from JSON
  - [x] Export EntityInstance to JSON
    - [x] Create EntityInstance from Vertex
  - [x] Unit Tests
- [x] `RelationInstanceManagerImpl`
  - [x] Has Relation Instance by Outbound-UUID, type-name and Inbound-UUID
  - [x] Get Relation Instance by Outbound-UUID, type-name and Inbound-UUID
  - [x] Create Relation Instance
  - [x] Delete Relation Instance By Outbound-UUID, type-name and Inbound-UUID
  - [x] Import Relation Instance from JSON
  - [x] Export Relation Instance to JSON
  - [x] Unit Tests
- [x] `ReactiveEntityInstanceManagerImpl`
  - [x] Central registry of all `ReactiveEntityInstance`s
  - [x] Create `ReactiveEntityInstance` by UUID
  - [x] On Instantiation: Instantiate `EntityBehaviour`
  - [x] Check if id exists in HashMap (must not exist)
  - [x] Check if id exists in Datastore -> Manager
  - [x] Unit Tests
- [x] `ReactiveRelationInstanceManagerImpl`
  - [x] Central registry of all `ReactiveRelationInstance`s
    * These are the actually "running" / "living" instances
  - [x] Create `ReactiveRelationInstance` by UUID
    - [x] Get Relation Instance by EdgeKey from `RelationInstanceManager`
    - [ ] On Instantiation: Instantiate `ReactiveRelationInstanceBehaviour` by TYPE
      - [ ] Connector
  - [x] Unit Tests
- [x] `EntityInstanceBehaviourManager`
  - [x] Instantiate Behaviour
  - [x] Remove Behaviour
- [x] `RelationInstanceBehaviourManager`
  - [x] Instantiate Behaviour
  - [x] Remove Behaviour
- [x] `FlowManagerImpl`
  - [x] Create Flow: Creates entity and relation instances contained in the flow
  - [x] Delete Flow: Deletes entity and relation instances contained in the flow
  - [x] Import, Export
  - [ ] Unit Tests
- [x] `ReactiveFlowManagerImpl`
  - [x] Map<FlowId, List<FlowId>>
  - [x] Has
  - [x] Get
  - [x] Create
  - [x] Commit
  - [x] Delete
  - [x] Export
  - [x] Import
  - [ ] Unit Tests

### Behaviours

- [ ] Implement Numeric Comparison
- [ ] Implement String Operations
- [ ] Implement String Gates
- [ ] Implement String Comparison
- [ ] Implement Timers / Cron Jobs
- [ ] Implement Entity with Numeric Constant by name
  - Math Constants https://doc.rust-lang.org/std/f64/consts/index.html
- [ ] Implement Noise Generation (Procedural Textures + Terrain Generation + Particles)
  - [ ] https://docs.rs/noise/0.7.0/noise/
  - [ ] https://github.com/Razaekel/noise-rs

### WebSocket

- [ ] CR: entity types / relation types
- [ ] CRUD: entity instances / relation instances / flows
- [ ] Notification entity / relation / flow / property
- [ ] https://github.com/housleyjk/ws-rs
- [ ] https://github.com/housleyjk/ws-rs/blob/master/examples/threaded.rs

### GraphQL

- [ ] https://github.com/graphql-rust/juniper
- [ ] Queries for entity instances / relation instances / flows
- [ ] Mutations for entity instances / relation instances / flows

### Command System Flow

- [ ] Websocket endpoint for command input
- [ ] Command parsing
- [ ] Rename behaviour simple_closure (log, print, ...) to command
- [ ] Make sure a simple_closure can have access to managers in order to get/create/update entities/relations
- [ ] Implement a terminal in the flow editor

### System Environment Flow

- [ ] Implement Flow for the Environment Variables
- [ ] Implement Flow for the Command Line Arguments
- [ ] Implement Flow for the System / OS Constants

### Beyond the Core

- [ ] rust bindgen C++
  - [ ] Bindings for Vulkan Renderer
- [ ] models
  - [ ] rust + glTF: https://github.com/gltf-rs/gltf
- [ ] procedural textures
  - [ ] Generate
- [ ] particles
- [ ] sound

### Permission Model

- [ ] Game API (JavaScript Access)
  - [ ] Map = Flow
  - [ ] Game Mode = Flow
  - [ ] Game State = Flow
  - [ ] Player = Flow
- [ ] System = Flow
