[![Logo](https://gitlab.com/qonfucius/aragog/-/snippets/2090578/raw/master/logo.svg)](http://aragog.rs)

# Aragog CLI

[![pipeline status](https://gitlab.com/qonfucius/aragog/badges/master/pipeline.svg)](https://gitlab.com/qonfucius/aragog/commits/master)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io](https://img.shields.io/crates/v/aragog_cli.svg)](https://crates.io/crates/aragog_cli)
[![dependency status](https://deps.rs/crate/aragog_cli/0.4.0/status.svg)](https://deps.rs/crate/aragog_cli)

[![Discord](https://img.shields.io/discord/763034131335741440.svg?label=&logo=discord&logoColor=ffffff&color=7389D8&labelColor=6A7EC2)](https://discord.gg/Xyx3hUP)
[![Gitter](https://badges.gitter.im/aragog-rs/community.svg)](https://gitter.im/aragog-rs/community)

Migration and schema generation tool for  [aragog](http://aragog.rs) ([crates.io](aragog)).

> Note: Currently, transactional operations are not supported, a failing migrations will not automatically rollback and you may need to handle some errors manually

## Installation

run with cargo: `cargo install aragog_cli`

## Options

- `--db-host <DB_HOST>` Sets the ArangoDB host (by default env var `DB_HOST` is used)
- `--db-name <DB_NAME>` Sets the ArangoDB database name (by default env var `DB_NAME` is used)
- `--db-password <DB_PASSWORD>` Sets the ArangoDB database user password (by default env var `DB_PASSWORD` is used)
- `--db-user <DB_USER>` Sets the ArangoDB database user (by default env var `DB_USER` is used)
- `--folder <PATH>` Sets the path for the migrations and schema (by default env var `SCHEMA_PATH` is used)
- `--aragog-collection <COLLECTION>` Sets the name of the config ArangoDB collection that will be used to synchronize database and schema version (by default "AragogConfiguration" is used)

### Verbose

Add `-v` option for debug log and `-vv` for verbose log

## Commands

### Creating a Migration

Command: `aragog create_migration $MIGRATION_NAME` 

Creates a new migration file in `$SCHEMA_PATH/migrations/`. If the `db` folder is missing it will be created automatically.

### Launching migrations

Command: `aragog migrate`

Will launch every migration in `$SCHEMA_PATH/migrations/` and update the schema according to its current version.
If there is no schema it will be generated.

### Rollbacking migrations

Command: `aragog rollback`

Will rollback 1 migration in `$SCHEMA_PATH/migrations/` and update the schema according to its current version.

Command: `aragog rollback $COUNT`

Will rollback `$COUNT` migrations in `$SCHEMA_PATH/migrations/` and update the schema according to its current version.

### Truncate database

Command: `aragog truncate_database`

Will drop every graph and collection in the database.

### Describe database

Command: `argog describe`

Will render information about the database, schema synced version and render a table describing every collection.

### Discover database

Command: `aragog discover`

Will generate and apply a migration file for every collection, graph and index in the database not referenced in the schema.

This command is useful to force synchronization between your database state and the schema or to initialize your schema and migrations from an existing database.

### Completion scripts (inspired by `rustup` documentation)

Command: `aragog completions`

Enable tab completion for Bash, Fish, Zsh, or PowerShell. The script is output on `stdout`, allowing one to re-direct the
output to the file of their choosing. Where you place the file will depend on which shell, and which operating system you are
using. Your particular configuration may also determine where these scripts need to be placed.

Here are some common set-ups for the three supported shells under Unix and similar operating systems (such as GNU/Linux).

#### BASH:

Completion files are commonly stored in `/etc/bash_completion.d/` for system-wide commands, but can be stored in
`~/.local/share/bash-completion/completions` for user-specific commands.
    
Run the command:

> $ mkdir -p ~/.local/share/bash-completion/completions
> $ aragog completions bash >> ~/.local/share/bash-completion/completions/aragog

This installs the completion script. You may have to log out and log back in to your shell session for the changes to take effect.

#### BASH (macOS/Homebrew):

Homebrew stores bash completion files within the Homebrew directory.
With the `bash-completion` brew formula installed, run the command:

> $ mkdir -p $(brew --prefix)/etc/bash_completion.d
> $ aragog completions bash > $(brew --prefix)/etc/bash_completion.d/aragog.bash-completion

#### FISH:

Fish completion files are commonly stored in `$HOME/.config/fish/completions`. 

Run the command:

> $ mkdir -p ~/.config/fish/completions
> $ aragog completions fish > ~/.config/fish/completions/aragog.fish

This installs the completion script. You may have to log out and log back in to your shell session for the changes to take effect.

#### ZSH:

ZSH completions are commonly stored in any directory listed in your `$fpath` variable. To use these completions, you must either
add the generated script to one of those directories, or add your own to this list.

Adding a custom directory is often the safest bet if you are unsure of which directory to use. First create the directory;
for this example we'll create a hidden directory inside our `$HOME`directory:

> $ mkdir ~/.zfunc

Then add the following lines to your `.zshrc` just before `compinit`:

> fpath+=~/.zfunc

Now you can install the completions script using the following command:

> $ aragog completions zsh > ~/.zfunc/_aragog

You must then either log out and log back in, or simply run

> $ exec zsh

for the new completions to take effect.

#### CUSTOM LOCATIONS:

Alternatively, you could save these files to the place of your choosing, such as a custom directory inside your $HOME.
Doing so will require you to add the proper directives, such as `source`ing inside your login script. Consult your shells documentation for
how to add such directives.

#### POWERSHELL:

The powershell completion scripts require PowerShell v5.0+ (which comes with Windows 10, but can be downloaded separately for windows 7 or 8.1).

First, check if a profile has already been set

> PS C:\> Test-Path $profile

If the above command returns `False` run the following

> PS C:\> New-Item -path $profile -type file -force

Now open the file provided by `$profile` (if you used the `New-Item` command it will be `${env:USERPROFILE}\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1`

Next, we either save the completions file into our profile, or into a separate file and source it inside our profile.
To save the completions into our profile simply use

> PS C:\> aragog completions powershell >> ${env:USERPROFILE}\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1

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
    wait_for_sync: true       # Optional waitForSync attribute
- delete_collection:          # Deletes a collection with
    name: CollectionName      # Mandatory name
- create_edge_collection:     # Creates a edge collection
    name: EdgeCollectionName  # Mandatory name
    wait_for_sync: false      # Optional waitForSync attribute
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
    collection: MyCollection  # Mandatory collection
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

> The expireAfter field is in camelCase instead of snake_case, this will be fixed in future releases

#### GeoSpatial index

```yaml
type: geo
geoJson: false
```

> The geoJson field is in camelCase instead of snake_case, this will be fixed in future releases

#### Fulltext index

```yaml
type: ttl
minLength: 3600
```

> The minLength field is in camelCase instead of snake_case, this will be fixed in future releases

## Todo list

- [ ] Migration commands:
  - [ ] `edit_collection`
  - [ ] check on graph creation the existence of the orphan collections
- [ ] Transaction Support
  - [ ] Wrap migration files on transactions

[aragog]: https://crates.io/crates/aragog