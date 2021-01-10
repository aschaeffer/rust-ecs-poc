## Entity Instance

What is an `Entity Instance`?

An `Entity Instance` is a particular example or occurrence of something. It could be anything in the game which is like an object: a player, a teleporter, a light,
a particle emitter, ...

### Example: Player named PeterPenacka

The `Entity Instance` has concrete values:

![PeterPenacka](https://yuml.me/diagram/scruffy/class/[Player|uuid;name%20=%20PeterPenacka;px%20=%200;py%20=%2050;pz%20=%2060;...])

The state of one concrete player is represented by one concrete `Entity Instance`.

### Lifecycle

* Start:
  * Database: Vertex (with properties)
  * JSON: Load
  * 

Vertex
* offizieller State

EntityInstance:
* type_name (EntityType)
* id (Vertex)
* properties: PropertyInstances

EntityInstanceManager:
* Hold references of "ReactiveEntityInstance"
* Model "EntityInstance" (serializable)
  * Create Vertex
    * Parameter: type, properties
    * Check if id exists in Datastore (must not exist)
  * Delete Vertex
    * Parameter: id
    * Check if id exists in HashMap (must not exist -> )
  * Import Vertex from JSON (EntityInstance)
    * Check if id exists in Datastore (must not exist)
    * Check if id exists in HashMap (must not exist)
    * Create Vertex with id
    * Create Vertex Properties
  * Export Vertex to JSON (EntityInstance)
    * Create EntityInstance from Vertex
    * 
* Model "ReactiveEntityInstance" (not serializable
  * Construct ReactiveEntityInstance from Vertex
    * Check if id exists in Datastore
    * Check if id exists in HashMap
    * Get EntityType from Vertex Type
    * Construct Properties
      * stream
    * EntityInstance::new()
      * id: vertex.id
      * type_name
      * components (vec<String>)
      * properties (vec<ReactivePropertyInstance>)
    * Add ReactiveEntityInstance to HashMap
  * Destruct EntityInstance
    * Destruct All PropertyInstances
    * Remove Entry from HashMap


Remove(type, properties) -> Vertex with id


* Add Components ->
* Remove(id) ->



* Streams
  * Wenn vertex gelöscht wird, müssen für alle properties auch die streams gelöscht werden

![EntityInstanceLifecycle](https://yuml.me/diagram/scruffy/class/[Create%20Vertex]-->[Add_Component]add%20further%20components-->[Add_Component]-->[Create_Property_Instances]-->[Remove_Components],[Create_Property_Instances]-->[Create_Property_Instance]-->[Create_Property_Instances],[Create_Property_Instance]-->[Create_Stream])
