## APIs

![APIs](https://yuml.me/diagram/scruffy/class/[IndraDB]<--[Internal%20API]<--[JavaScript%20API],[Internal%20API]<--[GraphQL%20API],[Internal%20API]<--[gRPC%20API])

| | Description | Features |
| --- | --- | --- |
| Indra DB | Embedded In-Memory Graph Database | <ul><li>Support for directed and typed graphs.</li><li>Support for queries with multiple hops.</li><li>Cross-language support via Cap'n Proto, or direct embedding as a library.</li><li>Support for JSON-based properties tied to vertices and edges.</li><li>Pluggable underlying datastores, with built-in support for in-memory-only, rocksdb and sled. Postgresql is available separately.</li><li>Written in rust! High performance, no GC pauses, and a higher degree of safety.</li></ul> |



* GraphDB
  * Internal API
    * Small wrapper around IndraDB
  * JavaScript API
    * Small wrapper around the internal API
  * GraphQL API
    * Small wrapper around the internal API
  * gRPC API
    * Small wrapper around the internal API

# Schnittstellen

* Monitor
* Window
* Octree
* Camera
* Models
* Textures

### Camera

* Die Camera muss im Renderer nur ein Mal existieren
* Im Entity-System können mehrere Cameras existieren
* Nur eine Camera ist die aktuelle Camera

### Reactive + JavaScript

* Properties-Streams nach JavaScript exportieren
* V8 Accessor stream get value
