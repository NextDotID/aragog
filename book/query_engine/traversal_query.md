
# Traversal Querying

You can use graph features with sub-queries with different ways:

## Straightforward Traversal query

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

## Sub queries

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
