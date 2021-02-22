
# Query Object

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

## Filter

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
# use aragog::query::{Filter, Comparison};
 let filter :Filter = Comparison::field("name").equals_str("felix").into();
 ```

### Example

```rust
    let query = Query::new("Company").filter(
        Filter::new(
        Comparison::field("company_name").not_like("%google%"))
            .and(Comparison::field("company_age").greater_than(15))
            .or(Comparison::any("emails").like("%gmail.com"))
            .and(Comparison::field("roles").in_str_array(&["SHIPPER", "FORWARDER"]))
    );
```

[ComparisonBuilder]: https://docs.rs/aragog/0.9.1/aragog/query/struct.ComparisonBuilder.html "Comparison Builder"