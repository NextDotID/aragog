# Aragog CLI

Migration and schema generation tool for [aragog][aragog].

> Note: Currently, transactional operations are not supported, a failing migrations will not automatically rollback and you may need to handle some errors manually

## Options

- `--db-host <DB_HOST>` Sets the ArangoDB host (by default env var `DB_HOST` is used)
- `--db-name <DB_NAME>` Sets the ArangoDB database name (by default env var `DB_NAME` is used)
- `--db-password <DB_PASSWORD>` Sets the ArangoDB database user password (by default env var `DB_PASSWORD` is used)
- `--db-user <DB_USER>` Sets the ArangoDB database user (by default env var `DB_USER` is used)
- `--folder <PATH>` Sets the path for the migrations and schema (by default env var `SCHEMA_PATH` is used)

## Commands

### Help

Command: `aragog --help`

### Creating a Migration

Command: `aragog create_migration $MIGRATION_NAME` 

Creates a new migration file in `$SCHEMA_PATH/db/`. If the `db` folder is missing it will be created automatically.

### Launching migrations

Command: `aragog migrate`

Will launch every migration in `$SCHEMA_PATH/db/` and update the schema according to its current version.
If there is no schema it will be generated.

### Rollbacking migrations

Command: `aragog rollback`

Will rollback 1 migration in `$SCHEMA_PATH/db/` and update the schema according to its current version.

Command: `aragog rollback $COUNT`

Will rollback `$COUNT` migrations in `$SCHEMA_PATH/db/` and update the schema according to its current version.

### Truncate database

Command: `aragog truncate_database`

Will drop every graph and collection in the database.

## Migration files

Every migration file looks like this:

```yaml
up:     # Mandatory
  - up_command:
      options: Option
down:   # optional
  - down_command:
      options: Option
```

The `up` section launches on `migrate` and `down` on `rollback`.
For perfect migration files, `down` should reverse exactly everything `up` does.

### Commands

There is a list of all available commands for the `up` and `down` section:

### Collection 

```yaml
- create_collection:          # Creates a colllection with
    name: CollectionName      # Mandatory name
- delete_collection:          # Deletes a collection with
    name: CollectionName      # Mandatory name
- create_edge_collection:     # Creates a edge collection
    name: EdgeCollectionName  # Mandatory name
- delete_edge_collection:     # Deletes a edge collection
    name: EdgeCollectionName  # Mandatory name
```

### AQL

```yaml
- aql: FOR i in..   # Runs a AQL command
```

### Graph

Full parameters:
```yaml
- create_graph:                         # Creates a Graph
    name: GraphName                     # Mandatory name
    edge_definitions:                   # Mandatory edge definition list
      - collection: MyEdgeCollection    # Edge Collection Name
        from: ["MyCollection"]          # List of collections for the `from` part
        to: ["MyCollection2"]           # List of collections for the `to` part
    orphan_collections:                 # Optional list of orphan collections
      - MyCollection3
    is_smart: false                     # Optional attribute (enterprise edition only)
    is_disjoint: false                  # Optional attribute (enterprise edition only)
    options:                            # Optional attribute
      smart_graph_attribute: region     # Optional attribute
      number_of_shards: 2               # Optional attribute
      replication_factor: 9             # Optional attribute
      write_concern: 8                  # Optional attribute
- delete_graph:                         # Deletes a graph   
    name: Graph                         # Mandatory name
```

For more information on the graph creation options see the [ArangoDB Documentation](https://www.arangodb.com/docs/stable/http/gharial-management.html#create-a-graph)

You can use it with minimal parameters:
```yaml
- create_graph:                      
    name: GraphName                  
    edge_definitions:                
      - collection: MyEdgeCollection 
        from: ["MyCollection"]       
        to: ["MyCollection2"]        
- delete_graph:                      
    name: Graph                      
```

### Index

```yaml
- create_index:               # Creates an Index
    name: MyIndex             # Mandatory name
    fields: ["name"]          # Mandatory index fields list
    collection: MyCollection  # Mandatory collection name (doesn't work on edge collections)
    settings:                 # Mandatory settings
      type: persistent        # Mandatory index type (hash, persistent, ttl, geospatial, fulltext, skiplist)
      unique: true
      sparse: false
      deduplicate: false
- delete_index:               # Deletes an Index
    name: MyIndex             # Mandatory name
```

You have various parameters on the `settings`according to index type:

#### Persistent index

```yaml
type: persistent
unique: true
sparse: false
deduplicate: false
```

#### Hash index

```yaml
type: hash
unique: true
sparse: false
deduplicate: false
```

#### SkipList index

```yaml
type: skiplist
unique: true
sparse: false
deduplicate: false
```

#### TTL index

```yaml
type: ttl
expireAfter: 3600
```

#### GeoSpatial index

```yaml
type: geo
geoJson: false
```

#### Fulltext index

```yaml
type: ttl
minLength: 3600
```

[aragog]: https://crates.io/crates/aragog