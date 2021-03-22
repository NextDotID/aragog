# The Query system

You can retrieve document from the database two ways:
- from the unique ArangoDB `_key` (see the [record](../record_trait/index.md) section)
- from an [AQL](https://www.arangodb.com/docs/stable/aql/index.html) query

`aragog` provides an AQL query builder system, allowing safer queries than direct string literals.

The example below show different ways to retrieve records, look at each function documentation for more exhaustive explanations.

**Example**
 ```rust
 // User creation
 let record = DatabaseRecord::create(user, &database_connection).await.unwrap();

 // Find with the primary key or...
 let user_record = User::find(record.key(), &database_connection).await.unwrap();
 // .. Generate a query and...
 let query = User::query().filter(Filter::new(Comparison::field("last_name").equals_str("Surcouf")).and(Comparison::field("age").greater_than(15)));
 // get the only record (fails if no or multiple records)
 let user_record = User::get(query, &database_connection).await.unwrap().uniq().unwrap();

 // Find all users with multiple conditions
 let query = User::query().filter(Filter::new(Comparison::field("last_name").like("%Surc%")).and(Comparison::field("age").in_array(&[15,16,17,18])));
 let clone_query = query.clone(); // we clone the query

 // This syntax is valid...
 let user_records = User::get(query, &database_connection).await.unwrap();
 // ... This one too
 let user_records = clone_query.call(&database_connection).await.unwrap().get_records::<User>();
 ```