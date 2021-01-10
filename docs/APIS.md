# APIs

![APIs](https://yuml.me/diagram/scruffy/class/[IndraDB]<--[Entity%20System%20API]<--[JavaScript%20API]<-->[Game%20Logic],[JavaScript%20API]<-->[Game%20Mode],[JavaScript%20API]<-->[Map%20Logic],[JavaScript%20API]<-->[MOD],[Entity%20System%20API]<--[GraphQL%20API]<-->[Game%20UI%20/%20Menus],[GraphQL%20API]<-->[Game%20HUD],[GraphQL%20API]<-->[Server%20UI],[Entity%20System%20API]<--[gRPC%20API]<-->[Inexor%20Server],[gRPC%20API]<-->[Other%20Inexor%20Clients])

| API | Target | Description |
| --- | --- | --- |
| Indra DB | - | **Embedded In-Memory Graph Database**<br><ul><li>Support for directed and typed graphs.</li><li>Support for queries with multiple hops.</li><li>Cross-language support via Cap'n Proto, or direct embedding as a library.</li><li>Support for JSON-based properties tied to vertices and edges.</li><li>Pluggable underlying datastores, with built-in support for in-memory-only, rocksdb and sled. Postgresql is available separately.</li><li>Written in rust! High performance, no GC pauses, and a higher degree of safety.</li></ul> |
| Entity System API | Game-Logic | <ul><li>Small wrapper around the Graph Database</li><li>Manages the datastore and transactions.</li><li>IndraDB has everything needed</li></ul>|
| JavaScript API | Game-Logic, Game-Modes, Mods | <ul><li>Small wrapper around the Rust API</li><li>Run scripts in order to manipulate entities / relations / properties</li></ul>|
| GraphQL API | Web Applications | <ul><li>Small wrapper around the Rust API</li><li>Authentication Layer</li></ul>|
| gRPC API | Server / other clients | <ul><li>Small wrapper around the Rust API</li><li>Authentication Layer</li></ul>|
