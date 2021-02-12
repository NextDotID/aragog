# The Query system

You can retrieve document from the database two ways:
- from the unique ArangoDB `_key` (see the [record](./record.md) section)
- from an [AQL](https://www.arangodb.com/docs/stable/aql/index.html) query

`aragog` provides an AQL query builder system, allowing safer queries than direct string litterals.

The example below show different ways to retrieve records, look at each function documentation for more exhaustive explanations.

**Example**
 ```rust
 // User creation
 let record = DatabaseRecord::create(user, &database_pool).await.unwrap();

 // Find with the primary key or..
 let user_record = User::find(&record.key, &database_pool).await.unwrap();
 // .. Generate a query and..
 let query = User::query().filter(Filter::new(Comparison::field("last_name").equals_str("Surcouf")).and(Comparison::field("age").greater_than(15)));
 // get the only record (fails if no or multiple records)
 let user_record = User::get(query, &database_pool).await.unwrap().uniq().unwrap();

 // Find all users with multiple conditions
 let query = User::query().filter(Filter::new(Comparison::field("last_name").like("%Surc%")).and(Comparison::field("age").in_array(&[15,16,17,18])));
 let clone_query = query.clone(); // we clone the query

 // This syntax is valid...
 let user_records = User::get(query, &database_pool).await.unwrap();
 // ... This one too
 let user_records = clone_query.call(&database_pool).await.unwrap().get_records::<User>();
 ```
You can simplify the previous queries with some tweaks and macros:
 ```rust
 #[macro_use]
 extern crate aragog;

 let record = DatabaseRecord::create(user, &database_pool).await.unwrap();

 // Find a user with a query
 let query = User::query().filter(compare!(field "last_name").equals_str("Surcouf").and(compare!(field "age").greater_than(15)));

 // get the only record (fails if no or multiple records)
 let user_record = User::get(query, &database_pool).await.unwrap().uniq().unwrap();

 // Find all users with multiple conditions
 let query = User::query().filter(compare!(field "last_name").like("%Surc%").and(compare!(field "age").in_array(&[15,16,17,18])));
 let clone_query = query.clone();
 // This syntax is valid...
 let user_records = User::get(query, &database_pool).await.unwrap();
 // ... This one too
 let user_records = clone_query.call(&database_pool).await.unwrap().get_records::<User>();
 ```

## Query Object

You can intialize a query in the following ways:
- The recommended way:
    * `Object::query()` (only works if `Object` implements `Record`)
- Unsafe ways:
    * `Query::new("CollectionName")`
    * `query!("CollectionName")`

You can customize the query with the following methods:
* `filter()` you can specify AQL comparisons
* `prune()` you can specify blocking AQL comparisons for traversal queries
* `sort()` you can specify fields to sort with
* `limit()` you can skip and limit the query results
* `distinct()` you can skip duplicate documents
> The order of operations will be respected in the rendered AQL query (except for `distinct`)

you can then call a query in the following ways:
* `query.call::<Object>(&database_connection_pool)`
* `Object::get(query, &database_connection_pool`

Which will return a `JsonQueryResult` containing a `Vec` of `serde_json::Value`.
`JsonQueryResult` can return deserialized models as `DatabaseRecord` by calling `.get_records::<T>()`

### Filter

You can initialize a `Filter` with `Filter::new(comparison)`

Each comparison is a `Comparison` struct built via `ComparisonBuilder`:
 ```rust
 // for a simple field comparison

 // Explicit
 Comparison::field("some_field").some_comparison("compared_value");
 // Macro
 compare!(field "some_field").some_comparison("compared_value");

 // for field arrays (see ArangoDB operators)

 // Explicit
 Comparison::all("some_field_array").some_comparison("compared_value");
 // Macro
 compare!(all "some_field_array").some_comparison("compared_value");

 // Explicit
 Comparison::any("some_field_array").some_comparison("compared_value");
 // Macro
 compare!(any "some_field_array").some_comparison("compared_value");

 // Explicit
 Comparison::none("some_field_array").some_comparison("compared_value");
 // Macro
 compare!(none "some_field_array").some_comparison("compared_value");
 ```
All the currently implemented comparison methods are listed under [ComparisonBuilder][ComparisonBuilder] documentation page.

Filters can be defined explicitely like this:

 ```rust
 let filter = Filter::new(Comparison::field("name").equals_str("felix"));
 ```

or

 ```rust
 let filter :Filter = Comparison::field("name").equals_str("felix").into();
 ```

### Traversal Querying

You can use graph features with sub-queries with different ways:

#### Straightforward Traversal query

* Explicit way
 ```rust
 let query = Query::outbound(1, 2, "edgeCollection", "User/123");
 let query = Query::inbound(1, 2, "edgeCollection", "User/123");
 let query = Query::any(1, 2, "edgeCollection", "User/123");
 // Named graph
 let query = Query::outbound_graph(1, 2, "NamedGraph", "User/123");
 let query = Query::inbound_graph(1, 2, "NamedGraph", "User/123");
 let query = Query::any_graph(1, 2, "NamedGraph", "User/123");
 ```

* Implicit way from a `DatabaseRecord<T>`

 ```rust
 let query = user_record.outbound_query(1, 2, "edgeCollection");
 let query = user_record.inbound_query(1, 2, "edgeCollection");
 // Named graph
 let query = user_record.outbound_graph(1, 2, "NamedGraph");
 let query = user_record.inbound_graph(1, 2, "NamedGraph");
 ```

#### Sub queries

Queries can be joined together through
* Edge traversal:

 ```rust
 let query = Query::new("User")
     .join_inbound(1, 2, false, Query::new("edgeCollection"));
 ```

* Named Graph traversal:

 ```rust
 let query = Query::new("User")
     .join_inbound(1, 2, true, Query::new("SomeGraph"));
 ```

It works with complex queries:

 ```rust
 let query = Query::new("User")
     .filter(Comparison::field("age").greater_than(10).into())
     .join_inbound(1, 2, false,
         Query::new("edgeCollection")
             .sort("_key", None)
             .join_outbound(1, 5, true,
                 Query::new("SomeGraph")
                     .filter(Comparison::any("roles").like("%Manager%").into())
                     .distinct()
                 )
     );
 ```

## Technical notes

### Todo list

- [ ] Advanced query system supporting:
  - [ ] Custom return system
  - [ ] Procedural Macros for syntax simplification and field presence validation at compile time
  - [ ] ArangoDB functions (`LENGTH`, `ABS`, etc.)
- [ ] Make batch write operation queries
- [ ] Make query engine optional through a feature gate