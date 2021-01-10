# rust-ecs-poc

## Goals

- [ ] Integrate Libraries
  - [x] Dependency Injection Library
  - [x] Graph Database Library
  - [ ] Reactive Streams
  - [ ] JavaScript Runtime
- [ ] Minimal Entity System API (Rust)
  - [x] Component Manager
    - [x] Has Component by Name
    - [x] Register Component
    - [x] Get All Components
    - [x] Get Component By Name
    - [x] Delete Component By Name
    - [x] Export Component To JSON File
    - [x] Import Component From JSON File
  - [x] Entity Type Manager
    - [x] Has Entity Type by Name
    - [x] Register Entity Type
      - [x] Expand Effective Properties From All Components (merge properties with the properties provided by the components)
    - [x] Create Entity Type
    - [x] Get Entity Type by Name
    - [x] Delete Entity Type By Name
    - [x] Export Entity Type To JSON File
    - [x] Import Entity Type From JSON File
  - [ ] Reactive Entity Instance Manager
    - [x] Hold references of "ReactiveEntityInstance"
      * These are the actually "running" / "living" instances
    - [x] Model "EntityInstance" (serializable)
      - [ ] Create Vertex
        - [ ] Parameter: type, properties
        - [ ] Check if id exists in Datastore (must not exist)
      - [ ] Delete Vertex
        - [ ] Parameter: id
        - [ ] Check if id exists in HashMap (must not exist -> )
      - [ ] Import Vertex from JSON (EntityInstance)
        - [ ] Check if id exists in Datastore (must not exist)
        - [ ] Check if id exists in HashMap (must not exist)
        - [ ] Create Vertex with id
        - [ ] Create Vertex Properties
      - [ ] Export Vertex to JSON (EntityInstance)
        - [ ] Create EntityInstance from Vertex
    - [x] Model "ReactiveEntityInstance" (not serializable!)
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

- [ ] Visual Scripting Base
  - [ ] Timestamp every N millis
  - [ ] System Constant NumCpus + PhysicalNumCpus
- [ ] Minimal JavaScript Runtime Integration
  - [ ] 
- [ ] Minimal Text UI Integration
  - [ ] 
- [ ] Minimal Command System
  - [ ] 
- [ ] Minimal GLFW Integration
  - [ ] 
- [ ] Minimal Entity System API (Rust)

## Project Setup

- [ ] Readme
- [ ] Documentation
- [ ] GitHub Actions
- [ ] Source Code Format Linter
